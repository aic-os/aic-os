import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'services/account_service.dart';
import 'services/wallet_service.dart';
import 'screens/home_screen.dart';

void main() {
  runApp(const AicWalletApp());
}

class AicWalletApp extends StatelessWidget {
  const AicWalletApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) {
          final a = AccountService();
          a.load();
          return a;
        }),
        ChangeNotifierProvider(create: (_) {
          final w = WalletService();
          w.startAutoRefresh();
          return w;
        }),
      ],
      child: MaterialApp(
        title: 'AIC Wallet',
        debugShowCheckedModeBanner: false,
        scrollBehavior: const AppScrollBehavior(),
        theme: ThemeData(
          brightness: Brightness.dark,
          scaffoldBackgroundColor: const Color(0xFF0A0A0F),
          colorScheme: const ColorScheme.dark(
            primary: Color(0xFF4A9EFF),
            secondary: Color(0xFF9C6AFF),
          ),
          useMaterial3: true,
        ),
        home: const HomeScreen(),
      ),
    );
  }
}

class AppScrollBehavior extends MaterialScrollBehavior {
  const AppScrollBehavior();

  @override
  Set<PointerDeviceKind> get dragDevices => {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
        PointerDeviceKind.trackpad,
        PointerDeviceKind.stylus,
      };
}
