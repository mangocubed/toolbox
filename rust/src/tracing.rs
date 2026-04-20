use sentry::ClientInitGuard;
use sentry::integrations::tracing::EventFilter;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

use crate::config::SENTRY_CONFIG;

pub fn start_tracing_subscriber() -> Option<ClientInitGuard> {
    let fmt_layer = tracing_subscriber::fmt::layer().with_filter(if cfg!(debug_assertions) {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    });

    let sentry_guard = if let Some(sentry_dsn) = SENTRY_CONFIG.dsn.as_deref() {
        let guard = sentry::init((
            sentry_dsn,
            sentry::ClientOptions {
                debug: cfg!(debug_assertions),
                enable_logs: true,
                release: Some(env!("CARGO_PKG_VERSION").into()),
                traces_sample_rate: SENTRY_CONFIG.traces_sample_rate,
                send_default_pii: SENTRY_CONFIG.send_default_pii,
                ..Default::default()
            },
        ));

        let sentry_layer = sentry::integrations::tracing::layer()
            .event_filter(|metadata| match *metadata.level() {
                tracing::Level::ERROR => EventFilter::Event | EventFilter::Log,
                tracing::Level::WARN => EventFilter::Breadcrumb | EventFilter::Log,
                _ => EventFilter::Ignore,
            })
            .span_filter(|metadata| matches!(*metadata.level(), tracing::Level::ERROR | tracing::Level::WARN));

        tracing_subscriber::registry().with(fmt_layer).with(sentry_layer).init();

        Some(guard)
    } else {
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(tracing_subscriber::fmt::layer())
            .init();

        None
    };

    tracing::info!("Tracing subscriber initialized.");

    sentry_guard
}
