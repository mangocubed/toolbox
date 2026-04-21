# Toolbox

## Environment variables

| Name                         | Type    | Default                         | Packages     |
| ---------------------------- | ------- | ------------------------------- | ------------ |
| CACHE_REDIS_URL              | String  | redis://127.0.0.1:6379/0        | rust         |
| CACHE_TTL_SECS               | Number  | 3600                            | rust         |
| IDENTITY_API_URL             | String  | https://api.id.mango3.app/      | flutter,rust |
| IDENTITY_CLIENT_ID           | String  |                                 | flutter      |
| IDENTITY_CLIENT_TOKEN        | String  |                                 | flutter      |
| IDENTITY_REDIRECT_URL        | String  |                                 | flutter      |
| IDENTITY_URL                 | String  | https://id.mango3.app/          | flutter      |
| MAILER_ENABLE                | Boolean | false                           | rust         |
| MAILER_SENDER_ADDRESS        | String  | Mango³ dev <no-reply@localhost> | rust         |
| MAILER_SMTP_ADDRESS          | String  | localhost                       | rust         |
| MAILER_SMTP_PASSWORD         | String  |                                 | rust         |
| MAILER_SMTP_SECURITY         | String  | none                            | rust         |
| MAILER_SMTP_USERNAME         | String  |                                 | rust         |
| MAILER_SUPPORT_EMAIL_ADDRESS | String  | support@localhost               | rust         |
| SENTRY_DSN                   | String  |                                 | rust         |
| SENTRY_TRACES_SAMPLE_RATE    | Number  | 1.0                             | rust         |
| SENTRY_SEND_DEFAULT_PII      | Boolean | true                            | rust         |
