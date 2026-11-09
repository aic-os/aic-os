import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:intl/intl.dart';

import '../services/wallet_service.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      context.read<WalletService>().refresh();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      body: SafeArea(
        child: RefreshIndicator(
          onRefresh: () => context.read<WalletService>().refresh(),
          child: ListView(
            padding: const EdgeInsets.all(20),
            children: [
              _buildHeader(),
              const SizedBox(height: 30),
              _buildBalanceCard(),
              const SizedBox(height: 20),
              _buildActions(),
              const SizedBox(height: 30),
              _buildChainInfo(),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildHeader() {
    return Row(
      children: [
        Container(
          width: 48, height: 48,
          decoration: BoxDecoration(
            gradient: const LinearGradient(
              colors: [Color(0xFF4A9EFF), Color(0xFF9C6AFF)],
            ),
            borderRadius: BorderRadius.circular(14),
          ),
          child: const Center(
            child: Text('AIC', style: TextStyle(
              color: Colors.white, fontWeight: FontWeight.bold, fontSize: 14,
            )),
          ),
        ),
        const SizedBox(width: 12),
        const Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('AIC Wallet', style: TextStyle(
                color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold,
              )),
              Text('Chain ID 2026', style: TextStyle(
                color: Colors.white54, fontSize: 12,
              )),
            ],
          ),
        ),
        IconButton(
          icon: const Icon(Icons.refresh, color: Colors.white70),
          onPressed: () => context.read<WalletService>().refresh(),
        ),
      ],
    );
  }

  Widget _buildBalanceCard() {
    return Consumer<WalletService>(
      builder: (context, w, _) {
        if (w.loading && w.balance == BigInt.zero) {
          return _loadingCard();
        }

        if (w.error != null && w.balance == BigInt.zero) {
          return _errorCard(w.error!);
        }

        final usdFmt = NumberFormat.currency(symbol: r'$', decimalDigits: 2);
        final rubFmt = NumberFormat.currency(symbol: '₽', decimalDigits: 2);

        return Container(
          padding: const EdgeInsets.all(24),
          decoration: BoxDecoration(
            gradient: const LinearGradient(
              colors: [Color(0xFF1A2D4D), Color(0xFF2D1A4D)],
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
            ),
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: const Color(0xFF3A4D6D), width: 1),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Total balance', style: TextStyle(
                color: Colors.white70, fontSize: 13,
              )),
              const SizedBox(height: 8),
              Row(
                crossAxisAlignment: CrossAxisAlignment.baseline,
                textBaseline: TextBaseline.alphabetic,
                children: [
                  Text(w.balanceAic, style: const TextStyle(
                    color: Colors.white, fontSize: 40, fontWeight: FontWeight.bold,
                  )),
                  const SizedBox(width: 8),
                  const Text('AIC', style: TextStyle(
                    color: Colors.white70, fontSize: 18,
                  )),
                ],
              ),
              const SizedBox(height: 8),
              Text('${usdFmt.format(w.balanceUsd)}  •  ${rubFmt.format(w.balanceRub)}',
                style: const TextStyle(color: Colors.white70, fontSize: 15)),
              const SizedBox(height: 4),
              Text('1 AIC = \$${w.priceUsd.toStringAsFixed(2)}',
                style: const TextStyle(color: Colors.white38, fontSize: 12)),
            ],
          ),
        );
      },
    );
  }

  Widget _loadingCard() {
    return Container(
      height: 200,
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(20),
      ),
      child: const Center(
        child: CircularProgressIndicator(color: Color(0xFF4A9EFF)),
      ),
    );
  }

  Widget _errorCard(String err) {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: const Color(0xFF2D1A1A),
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: const Color(0xFF6D3A3A)),
      ),
      child: Column(
        children: [
          const Icon(Icons.error_outline, color: Colors.redAccent, size: 40),
          const SizedBox(height: 12),
          const Text('Нода недоступна', style: TextStyle(
            color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold,
          )),
          const SizedBox(height: 8),
          Text('Запусти aic-node.exe и потяни вниз',
            style: const TextStyle(color: Colors.white70, fontSize: 12),
            textAlign: TextAlign.center),
          const SizedBox(height: 8),
          Text(err, style: const TextStyle(color: Colors.white38, fontSize: 10),
            textAlign: TextAlign.center),
        ],
      ),
    );
  }

  Widget _buildActions() {
    return Row(
      children: [
        Expanded(child: _actionButton(Icons.arrow_upward, 'Отправить', () {
          _showComingSoon('Отправка AIC');
        })),
        const SizedBox(width: 12),
        Expanded(child: _actionButton(Icons.arrow_downward, 'Получить', () {
          _showComingSoon('QR-код для приёма');
        })),
        const SizedBox(width: 12),
        Expanded(child: _actionButton(Icons.swap_horiz, 'Своп', () {
          _showComingSoon('DEX-агрегатор');
        })),
      ],
    );
  }

  Widget _actionButton(IconData icon, String label, VoidCallback onTap) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 18),
        decoration: BoxDecoration(
          color: const Color(0xFF1A1A2D),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: const Color(0xFF2A2A3D)),
        ),
        child: Column(
          children: [
            Icon(icon, color: const Color(0xFF4A9EFF), size: 28),
            const SizedBox(height: 8),
            Text(label, style: const TextStyle(color: Colors.white, fontSize: 13)),
          ],
        ),
      ),
    );
  }

  Widget _buildChainInfo() {
    return Consumer<WalletService>(
      builder: (context, w, _) {
        return Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: const Color(0xFF1A1A2D),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Сеть', style: TextStyle(
                color: Colors.white70, fontSize: 12,
              )),
              const SizedBox(height: 12),
              _infoRow('Chain ID', w.chainId == 0 ? '—' : w.chainId.toString()),
              _infoRow('Block height', w.blockNumber == 0 ? '—' : w.blockNumber.toString()),
              _infoRow('Адрес', '${w.address.substring(0, 6)}...${w.address.substring(38)}'),
            ],
          ),
        );
      },
    );
  }

  Widget _infoRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: const TextStyle(color: Colors.white54, fontSize: 13)),
          Text(value, style: const TextStyle(color: Colors.white, fontSize: 13)),
        ],
      ),
    );
  }

  void _showComingSoon(String feature) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('$feature -- в разработке'),
        backgroundColor: const Color(0xFF4A9EFF),
      ),
    );
  }
}
