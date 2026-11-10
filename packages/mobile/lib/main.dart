import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'services/account_service.dart';
import 'services/settings_service.dart';
import 'services/wallet_service.dart';
import 'l10n/l10n_service.dart';
import 'screens/home_screen.dart';
import 'screens/onboarding_screen.dart';
import 'services/seed_service.dart';

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
          final s = SettingsService();
          s.load();
          return s;
        }),
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
        ChangeNotifierProvider(create: (_) {
          final l = L10nService();
          l.load();
          return l;
        }),
      ],
      child: Consumer<L10nService>(
        builder: (context, l10n, _) {
          return MaterialApp(
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
            locale: Locale(l10n.lang),
            home: const _StartupRouter(),
          );
        },
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
class _StartupRouter extends StatefulWidget {
  const _StartupRouter();

  @override
  State<_StartupRouter> createState() => _StartupRouterState();
}

class _StartupRouterState extends State<_StartupRouter> {
  bool? _hasWallet;

  @override
  void initState() {
    super.initState();
    SeedService.hasWallet().then((v) {
      if (mounted) setState(() => _hasWallet = v);
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_hasWallet == null) {
      return const Scaffold(
        backgroundColor: Color(0xFF0A0A0F),
        body: Center(child: CircularProgressIndicator(color: Color(0xFF4A9EFF))),
      );
    }
    return _hasWallet! ? const HomeScreen() : const OnboardingScreen();
  }
}
