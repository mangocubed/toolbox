import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:http/http.dart' as http;
import 'package:oauth2_client/access_token_response.dart';
import 'package:oauth2_client/oauth2_client.dart';
import 'package:restart_app/restart_app.dart';
import 'package:url_launcher/url_launcher.dart';

import 'constants.dart';
import 'components.dart';
import 'config.dart';

class IdentityClient {
  static AccessTokenResponse? _accessToken;

  static final OAuth2Client _oauth2Client = OAuth2Client(
    authorizeUrl: Config.identityUrl.replace(path: '/oauth/authorize').toString(),
    redirectUri: Config.identityRedirectUrl.toString(),
    customUriScheme: Config.identityRedirectUrl.scheme,
    tokenUrl: Config.identityApiUrl.replace(path: '/oauth/token').toString(),
    refreshUrl: Config.identityApiUrl.replace(path: '/oauth/token').toString(),
    revokeUrl: Config.identityApiUrl.replace(path: '/oauth/revoke').toString(),
    credentialsLocation: CredentialsLocation.body,
  );

  static final _storage = FlutterSecureStorage();

  static Future<AccessTokenResponse?> _decodeAccessToken(String value) async {
    var accessToken = AccessTokenResponse.fromMap(jsonDecode(value));

    if (accessToken.accessToken == null) {
      await _deleteAccessToken();

      return null;
    }

    if (accessToken.refreshNeeded() && accessToken.refreshToken != null) {
      accessToken = await _oauth2Client.refreshToken(accessToken.refreshToken!, clientId: Config.identityClientId);

      if (accessToken.accessToken == null) {
        await _deleteAccessToken();

        return null;
      }

      await _writeAccessToken(accessToken);
    }

    return accessToken;
  }

  static Future<AccessTokenResponse?> _readAccessToken() async {
    final value = await _storage.read(key: keyAccessToken);

    if (value == null) {
      return null;
    }

    return await _decodeAccessToken(value);
  }

  static Future<void> _deleteAccessToken() async {
    await _storage.delete(key: keyAccessToken);
  }

  static Future<void> _writeAccessToken(AccessTokenResponse accessToken) async {
    await _storage.write(key: keyAccessToken, value: jsonEncode(accessToken.toMap()));
  }

  static Future<void> init() async {
    _accessToken = await _readAccessToken();

    _storage.registerListener(
      key: keyAccessToken,
      listener: (value) async {
        if (value == null) {
          _accessToken = null;

          return;
        }

        _accessToken = await _decodeAccessToken(value);
      },
    );
  }

  static Future<void> authorize(
    BuildContext context, {
    FutureOr<void> Function(BuildContext context)? onSuccess,
  }) async {
    try {
      final accessToken = await _oauth2Client.getTokenWithAuthCodeFlow(
        clientId: Config.identityClientId,
        enableState: false,
      );

      if (accessToken.accessToken == null) {
        if (context.mounted) {
          showSnackBarAlert(context, 'Failed to authenticate user');
        }

        return;
      }

      await _writeAccessToken(accessToken);

      if (context.mounted) {
        await onSuccess?.call(context);
      }

      await Restart.restartApp();
    } catch (error) {
      if (context.mounted) {
        showSnackBarAlert(context, 'Failed to authenticate user');
      }
    }
  }

  static Future<void> revoke() async {
    if (!hasAccessToken) {
      return;
    }

    await _oauth2Client.revokeAccessToken(_accessToken!, clientId: Config.identityClientId);
    await _deleteAccessToken();
    await Restart.restartApp();
  }

  /// Checks if the app is authorized and restarts if not.
  static Future<void> checkAuthorization() async {
    // Restart the app if the access token has changed on another instance.
    if (_accessToken?.accessToken != (await _readAccessToken())?.accessToken) {
      await Restart.restartApp();
    }

    // Restart the app if the access token is not valid.
    if (!await isAuthorized() && hasAccessToken) {
      await _deleteAccessToken();
      await Restart.restartApp();
    }
  }

  static String? get bearer {
    final token = _accessToken?.accessToken ?? Config.identityClientToken;

    if (token.isEmpty) {
      return null;
    }

    return 'Bearer $token';
  }

  static Future<void> goToIdentity() async {
    if (await canLaunchUrl(Config.identityUrl)) {
      await launchUrl(Config.identityUrl, mode: LaunchMode.inAppBrowserView);
    }
  }

  static bool get hasAccessToken {
    return _accessToken != null;
  }

  static Future<bool> isAuthorized() async {
    final response = await http.get(
      Config.identityApiUrl.replace(path: '/authorized'),
      headers: {'Authorization': bearer ?? ''},
    );

    return response.statusCode == 200;
  }

  static void withAuthentication(BuildContext context, FutureOr<void> Function(BuildContext context) callback) async {
    if (hasAccessToken) {
      await callback(context);
    } else {
      await IdentityClient.authorize(context, onSuccess: callback);
    }
  }
}
