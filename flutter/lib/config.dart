class Config {
  static final Uri identityApiUrl = Uri.parse(
    const String.fromEnvironment('IDENTITY_API_URL', defaultValue: 'https://api.id.mango3.app/'),
  );

  static final String identityClientId = const String.fromEnvironment('IDENTITY_CLIENT_ID');

  static final String identityClientToken = const String.fromEnvironment('IDENTITY_CLIENT_TOKEN');

  static final Uri identityRedirectUrl = Uri.parse(
    const String.fromEnvironment('IDENTITY_REDIRECT_URL', defaultValue: ''),
  );

  static final Uri identityUrl = Uri.parse(
    const String.fromEnvironment('IDENTITY_URL', defaultValue: 'https://id.mango3.app/'),
  );
}
