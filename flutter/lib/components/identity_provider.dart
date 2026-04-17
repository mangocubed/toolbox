import 'package:flutter/material.dart';

import '../identity_client.dart';

class IdentityProvider extends StatefulWidget {
  const IdentityProvider({super.key, required this.child});

  final Widget child;

  @override
  State<IdentityProvider> createState() => _IdentityProviderState();
}

class _IdentityProviderState extends State<IdentityProvider> with WidgetsBindingObserver {
  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    super.didChangeAppLifecycleState(state);

    if (state == AppLifecycleState.resumed) {
      IdentityClient.checkAuthorization();
    }
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    IdentityClient.checkAuthorization();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}
