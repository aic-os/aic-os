import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../models/coin.dart';
import '../services/account_service.dart';
import '../services/settings_service.dart';
import '../services/wallet_service.dart';
import '../services/coin_service.dart';
import '../services/aic_rpc.dart';
import '../l10n/l10n_service.dart';
import '../widgets/account_header.dart';
import '../widgets/crypto_slider.dart';
import '../widgets/features_slider.dart';
import '../widgets/coin_catalog_sheet.dart';
import '../widgets/history_list.dart';
import 'send_screen.dart';
import 'receive_screen.dart';
import 'register_name_screen.dart';
import 'quantum_bounty_screen.dart';
import 'settings_screen.dart';
import 'telegram_screen.dart';
import 'time_capsule_screen.dart';
import 'swap_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  CoinService? _coinService;
  String? _lastAddress;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final settings = context.read<SettingsService>();
      final wallet = context.read<WalletService>();
      if (wallet.rpc.rpcUrl != settings.rpcUrl) {
        wallet.updateRpcUrl(settings.rpcUrl);
      }
      final acc = context.read<AccountService>().current;
      if (acc != null && acc.address != _lastAddress) {
        _lastAddress = acc.address;
        wallet.updateAddress(acc.address);
        setState(() {
          _coinService = CoinService(
            rpc: AicRpc(rpcUrl: settings.rpcUrl),
            address: acc.address,
          );
        });
        _coinService!.loadSelected().then((_) => _coinService!.refresh());
      }
    });
  }

  void _showFeature(String name) {
    final l10n = context.read<L10nService>();
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('$name -- ${l10n.t('coming_soon')}'),
        backgroundColor: const Color(0xFF4A9EFF),
      ),
    );
  }

  void _openCoinCatalog() {
    if (_coinService == null) return;
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (_) => CoinCatalogSheet(
        available: _coinService!.availableToAdd,
        onAdd: (symbol) => _coinService!.addCoin(symbol),
      ),
    );
  }

  void _showRemoveCoin(Coin coin) {
    if (_coinService == null) return;
    final l10n = context.read<L10nService>();
    if (coin.symbol == 'AIC') {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(l10n.t('aic_cannot_remove'))),
      );
      return;
    }
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: Text('${l10n.t('remove_coin_q')} ${coin.symbol}?',
            style: const TextStyle(color: Colors.white)),
        content: const Text(
          'Removed from slider. Balance preserved on-chain.',
          style: TextStyle(color: Colors.white70),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text(l10n.t('cancel')),
          ),
          TextButton(
            onPressed: () {
              _coinService!.removeCoin(coin.symbol);
              Navigator.pop(ctx);
            },
            child: Text(l10n.t('remove_coin'),
                style: const TextStyle(color: Colors.redAccent)),
          ),
        ],
      ),
    );
  }

  void _openCoinDetail(Coin coin) {
    showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF1A1A2D),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(24)),
      ),
      builder: (ctx) => _coinDetailSheet(coin),
    );
  }

  @override
  Widget build(BuildContext context) {
    final acc = context.watch<AccountService>().current;
    final l10n = context.watch<L10nService>();

    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      body: SafeArea(
        child: RefreshIndicator(
          onRefresh: () async {
            await _coinService?.refresh();
          },
          child: ListView(
            padding: const EdgeInsets.only(top: 8, bottom: 20),
            children: [
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: AccountHeader(
                  onSettings: () => Navigator.push(
                    context,
                    MaterialPageRoute(builder: (_) => const SettingsScreen()),
                  ),
                ),
              ),
              const SizedBox(height: 20),
              if (_coinService != null)
                AnimatedBuilder(
                  animation: _coinService!,
                  builder: (_, __) => CryptoSlider(
                    coins: _coinService!.coins,
                    address: acc?.address ?? '',
                    onTapCoin: _openCoinDetail,
                    onAddCoin: _openCoinCatalog,
                    onLongPressCoin: _showRemoveCoin,
                  ),
                )
              else
                const SizedBox(height: 210),
              const SizedBox(height: 16),
              if (_coinService != null)
                AnimatedBuilder(
                  animation: _coinService!,
                  builder: (_, __) => _totalBalanceCard(_coinService!, l10n),
                ),
              const SizedBox(height: 24),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: Text(l10n.t('features'),
                    style: const TextStyle(
                      color: Colors.white70, fontSize: 13, fontWeight: FontWeight.w600,
                    )),
              ),
              const SizedBox(height: 10),
              FeaturesSlider(
                features: [
                  FeatureItem(
                    id: 'quantum',
                    title: l10n.t('feat_quantum'),
                    subtitle: l10n.t('feat_quantum_sub'),
                    icon: Icons.emoji_events,
                    color: Colors.amber,
                  ),
                  FeatureItem(
                    id: 'aicid',
                    title: l10n.t('feat_aicid'),
                    subtitle: l10n.t('feat_aicid_sub'),
                    icon: Icons.alternate_email,
                    color: const Color(0xFF9C6AFF),
                  ),
                  FeatureItem(
                    id: 'telegram',
                    title: l10n.t('feat_telegram'),
                    subtitle: l10n.t('feat_telegram_sub'),
                    icon: Icons.send,
                    color: const Color(0xFF29A9EB),
                  ),
                  FeatureItem(
                    id: 'vpn',
                    title: l10n.t('feat_vpn'),
                    subtitle: l10n.t('feat_vpn_sub'),
                    icon: Icons.shield,
                    color: const Color(0xFF4CAF50),
                  ),
                  FeatureItem(
                    id: 'capsule',
                    title: l10n.t('feat_capsule'),
                    subtitle: l10n.t('feat_capsule_sub'),
                    icon: Icons.lock_clock,
                    color: const Color(0xFF4A9EFF),
                  ),
                  FeatureItem(
                    id: 'inherit',
                    title: l10n.t('feat_inherit'),
                    subtitle: l10n.t('feat_inherit_sub'),
                    icon: Icons.family_restroom,
                    color: const Color(0xFFE91E63),
                  ),
                ],
                onTap: (f) {
                  if (f.id == 'quantum') {
                    Navigator.push(context,
                        MaterialPageRoute(builder: (_) => const QuantumBountyScreen()));
                  } else if (f.id == 'aicid') {
                    Navigator.push(context,
                        MaterialPageRoute(builder: (_) => const RegisterNameScreen()));
                  } else if (f.id == 'capsule') {
                    Navigator.push(context,
                        MaterialPageRoute(builder: (_) => const TimeCapsuleScreen()));
                  } else if (f.id == 'telegram') {
                    Navigator.push(context,
                        MaterialPageRoute(builder: (_) => const TelegramScreen()));
                  } else {
                    _showFeature(f.title);
                  }
                },
              ),
              const SizedBox(height: 24),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: Row(
                  children: [
                    Expanded(child: _quickAction(Icons.arrow_upward, l10n.t('send'), () {
                      Navigator.push(context,
                          MaterialPageRoute(builder: (_) => const SendScreen()));
                    })),
                    const SizedBox(width: 10),
                    Expanded(child: _quickAction(Icons.arrow_downward, l10n.t('receive'), () {
                      Navigator.push(context,
                          MaterialPageRoute(builder: (_) => const ReceiveScreen()));
                    })),
                    const SizedBox(width: 10),
                    Expanded(child: _quickAction(Icons.swap_horiz, l10n.t('swap'), () {
                      Navigator.push(context,
                          MaterialPageRoute(builder: (_) => const SwapScreen()));
                    })),
                  ],
                ),
              ),
              const SizedBox(height: 24),
              if (_coinService != null)
                AnimatedBuilder(
                  animation: _coinService!,
                  builder: (_, __) => Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 20),
                    child: HistoryList(
                      transactions: _coinService!.transactions,
                    ),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _totalBalanceCard(CoinService svc, L10nService l10n) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 20),
      child: Container(
        padding: const EdgeInsets.all(18),
        decoration: BoxDecoration(
          color: const Color(0xFF1A1A2D),
          borderRadius: BorderRadius.circular(16),
        ),
        child: Row(
          children: [
            const Icon(Icons.account_balance_wallet,
                color: Color(0xFF4A9EFF), size: 22),
            const SizedBox(width: 10),
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(l10n.t('total_balance'),
                    style: const TextStyle(color: Colors.white54, fontSize: 11)),
                const SizedBox(height: 2),
                Text('\$' + svc.totalUsd.toStringAsFixed(2),
                    style: const TextStyle(
                      color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold,
                    )),
              ],
            ),
            const Spacer(),
            Text('RUB ' + svc.totalRub.toStringAsFixed(0),
                style: const TextStyle(color: Colors.white54, fontSize: 13)),
          ],
        ),
      ),
    );
  }

  Widget _quickAction(IconData icon, String label, VoidCallback onTap) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 14),
        decoration: BoxDecoration(
          color: const Color(0xFF1A1A2D),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: const Color(0xFF2A2A3D)),
        ),
        child: Column(
          children: [
            Icon(icon, color: const Color(0xFF4A9EFF), size: 22),
            const SizedBox(height: 6),
            Text(label, style: const TextStyle(color: Colors.white, fontSize: 12)),
          ],
        ),
      ),
    );
  }

  Widget _coinDetailSheet(Coin coin) {
    final l10n = context.read<L10nService>();
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(width: 40, height: 4,
                decoration: BoxDecoration(color: Colors.white24, borderRadius: BorderRadius.circular(2))),
            const SizedBox(height: 20),
            Text(coin.symbol,
                style: const TextStyle(
                  color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold,
                )),
            const SizedBox(height: 4),
            Text('\$' + coin.valueUsd.toStringAsFixed(2) +
                 '  ·  RUB ' + coin.valueRub.toStringAsFixed(0),
                style: const TextStyle(color: Colors.white70, fontSize: 14)),
            const SizedBox(height: 20),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                _detailAction(Icons.arrow_upward, l10n.t('send'), () {
                  Navigator.pop(context);
                  Navigator.push(context,
                      MaterialPageRoute(builder: (_) => const SendScreen()));
                }),
                _detailAction(Icons.arrow_downward, l10n.t('receive'), () {
                  Navigator.pop(context);
                  Navigator.push(context,
                      MaterialPageRoute(builder: (_) => const ReceiveScreen()));
                }),
                _detailAction(Icons.swap_horiz, l10n.t('swap'), () {
                  Navigator.pop(context);
                  Navigator.push(context,
                      MaterialPageRoute(builder: (_) => SwapScreen(
                        initialFrom: coin.symbol,
                        initialTo: coin.symbol == 'AIC' ? 'ETH' : 'AIC',
                      )));
                }),
                _detailAction(Icons.show_chart, l10n.t('trade'), () {
                  Navigator.pop(context);
                  _showFeature(l10n.t('trade'));
                }),
              ],
            ),
            const SizedBox(height: 10),
          ],
        ),
      ),
    );
  }

  Widget _detailAction(IconData icon, String label, VoidCallback onTap) {
    return GestureDetector(
      onTap: onTap,
      child: Column(
        children: [
          Container(
            width: 50, height: 50,
            decoration: BoxDecoration(
              color: const Color(0xFF4A9EFF).withOpacity(0.15),
              borderRadius: BorderRadius.circular(16),
            ),
            child: Icon(icon, color: const Color(0xFF4A9EFF), size: 22),
          ),
          const SizedBox(height: 6),
          Text(label, style: const TextStyle(color: Colors.white, fontSize: 11)),
        ],
      ),
    );
  }
}
