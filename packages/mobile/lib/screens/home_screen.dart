import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../models/coin.dart';
import '../services/account_service.dart';
import '../services/coin_service.dart';
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
    final acc = context.watch<AccountService>().current;
    if (acc != null && acc.address != _lastAddress) {
      _lastAddress = acc.address;
      _coinService = CoinService(address: acc.address);
      _coinService!.loadSelected().then((_) => _coinService!.refresh());
    }
  }

  void _showFeature(String name) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('$name -- coming soon'),
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
        onAdd: (symbol) {
          _coinService!.addCoin(symbol);
        },
      ),
    );
  }

  void _showRemoveCoin(Coin coin) {
    if (_coinService == null) return;
    if (coin.symbol == 'AIC') {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('AIC cannot be removed')),
      );
      return;
    }
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: Text('Remove ' + coin.symbol + '?',
            style: const TextStyle(color: Colors.white)),
        content: const Text(
          'It will be removed from your slider (balance preserved on-chain).',
          style: TextStyle(color: Colors.white70),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              _coinService!.removeCoin(coin.symbol);
              Navigator.pop(ctx);
            },
            child: const Text('Remove', style: TextStyle(color: Colors.redAccent)),
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
                  builder: (_, __) => _totalBalanceCard(_coinService!),
                ),

              const SizedBox(height: 24),

              const Padding(
                padding: EdgeInsets.symmetric(horizontal: 20),
                child: Text('Features',
                    style: TextStyle(
                      color: Colors.white70, fontSize: 13, fontWeight: FontWeight.w600,
                    )),
              ),
              const SizedBox(height: 10),
              FeaturesSlider(
                features: [
                  FeatureItem(
                    id: 'quantum',
                    title: 'Quantum Bounty',
                    subtitle: '300k AIC prize',
                    icon: Icons.emoji_events,
                    color: Colors.amber,
                  ),
                  FeatureItem(
                    id: 'aicid',
                    title: 'AIC-ID',
                    subtitle: 'Register @name',
                    icon: Icons.alternate_email,
                    color: const Color(0xFF9C6AFF),
                  ),
                  FeatureItem(
                    id: 'telegram',
                    title: 'Telegram',
                    subtitle: 'Built-in messenger',
                    icon: Icons.send,
                    color: const Color(0xFF29A9EB),
                  ),
                  FeatureItem(
                    id: 'vpn',
                    title: 'VPN',
                    subtitle: '1 AIC/month',
                    icon: Icons.shield,
                    color: const Color(0xFF4CAF50),
                  ),
                  FeatureItem(
                    id: 'capsule',
                    title: 'TimeCapsule',
                    subtitle: 'Lock AIC',
                    icon: Icons.lock_clock,
                    color: const Color(0xFF4A9EFF),
                  ),
                  FeatureItem(
                    id: 'inherit',
                    title: 'Inheritance',
                    subtitle: 'Dead man switch',
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
                    Expanded(child: _quickAction(Icons.arrow_upward, 'Send', () {
                      Navigator.push(context,
                          MaterialPageRoute(builder: (_) => const SendScreen()));
                    })),
                    const SizedBox(width: 10),
                    Expanded(child: _quickAction(Icons.arrow_downward, 'Receive', () {
                      Navigator.push(context,
                          MaterialPageRoute(builder: (_) => const ReceiveScreen()));
                    })),
                    const SizedBox(width: 10),
                    Expanded(child: _quickAction(Icons.swap_horiz, 'Swap', () => _showFeature('Swap'))),
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
                      transactions: _loadHistory(),
                    ),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }

  List<Map<String, dynamic>> _loadHistory() {
    // MVP: пустая история. В следующем блоке -- реальная загрузка через aic_getAccountTransactions.
    return [];
  }

  Widget _totalBalanceCard(CoinService svc) {
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
                const Text('Total balance',
                    style: TextStyle(color: Colors.white54, fontSize: 11)),
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
                _detailAction(Icons.arrow_upward, 'Send', () {
                  Navigator.pop(context);
                  Navigator.push(context,
                      MaterialPageRoute(builder: (_) => const SendScreen()));
                }),
                _detailAction(Icons.arrow_downward, 'Receive', () {
                  Navigator.pop(context);
                  Navigator.push(context,
                      MaterialPageRoute(builder: (_) => const ReceiveScreen()));
                }),
                _detailAction(Icons.swap_horiz, 'Swap', () {
                  Navigator.pop(context);
                  _showFeature('Swap');
                }),
                _detailAction(Icons.show_chart, 'Trade', () {
                  Navigator.pop(context);
                  _showFeature('Trade');
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

