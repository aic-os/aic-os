import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../services/wallet_service.dart';
import '../widgets/history_list.dart';
import 'send_screen.dart';
import 'register_name_screen.dart';
import 'receive_screen.dart';

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
        child: Consumer<WalletService>(
          builder: (context, wallet, _) {
            return RefreshIndicator(
              onRefresh: () => wallet.refresh(),
              child: ListView(
                padding: const EdgeInsets.all(20),
                children: [
                  _header(),
                  const SizedBox(height: 30),
                  _balance(wallet),
                  const SizedBox(height: 20),
                  _networkInfo(wallet),
                  const SizedBox(height: 20),
                  _sendButton(),
                  const SizedBox(height: 20),
                  const SizedBox(height: 12),
                  GestureDetector(
                    onTap: () => Navigator.push(
                      context,
                      MaterialPageRoute(builder: (_) => const RegisterNameScreen()),
                    ),
                    child: Container(
                      padding: const EdgeInsets.symmetric(vertical: 14),
                      decoration: BoxDecoration(
                        color: const Color(0xFF1A2D4D),
                        borderRadius: BorderRadius.circular(12),
                        border: Border.all(color: const Color(0xFF9C6AFF), width: 1),
                      ),
                      child: const Center(
                        child: Text(
                          'Register AIC-ID (10 AIC)',
                          style: TextStyle(
                            color: Color(0xFF9C6AFF),
                            fontSize: 14,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(height: 20),
                  HistoryList(transactions: wallet.transactions),
                ],
              ),
            );
          },
        ),
      ),
    );
  }

  Widget _header() {
    return Row(
      children: [
        Container(
          width: 48,
          height: 48,
          decoration: BoxDecoration(
            gradient: const LinearGradient(
              colors: [Color(0xFF4A9EFF), Color(0xFF9C6AFF)],
            ),
            borderRadius: BorderRadius.circular(14),
          ),
          child: const Center(
            child: Text('AIC', style: TextStyle(
              color: Colors.white, fontWeight: FontWeight.bold,
            )),
          ),
        ),
        const SizedBox(width: 12),
        const Text('AIC Wallet', style: TextStyle(
          color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold,
        )),
      ],
    );
  }

  Widget _balance(WalletService w) {
    final usd = w.balanceUsd.toStringAsFixed(2);
    final rub = w.balanceRub.toStringAsFixed(0);
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        gradient: const LinearGradient(
          colors: [Color(0xFF1A2D4D), Color(0xFF2D1A4D)],
        ),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Balance', style: TextStyle(color: Colors.white70)),
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
          const SizedBox(height: 10),
          Text('\$' + usd + '  ·  RUB ' + rub,
              style: const TextStyle(color: Colors.white70, fontSize: 15)),
          const SizedBox(height: 4),
          Text('1 AIC = \$' + w.priceUsd.toStringAsFixed(2) + '  ·  RUB ' + w.priceRub.toStringAsFixed(0),
              style: const TextStyle(color: Colors.white38, fontSize: 12)),
        ],
      ),
    );
  }

  Widget _networkInfo(WalletService w) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(16),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Block height', style: TextStyle(color: Colors.white54)),
              Text(w.blockNumber.toString(), style: const TextStyle(color: Colors.white)),
            ],
          ),
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Address', style: TextStyle(color: Colors.white54)),
              Text(w.address.substring(0, 8) + '...' + w.address.substring(38),
                  style: const TextStyle(color: Colors.white)),
            ],
          ),
        ],
      ),
    );
  }

  Widget _sendButton() {
    return Row(
      children: [
        Expanded(
          child: GestureDetector(
            onTap: () => Navigator.push(
              context,
              MaterialPageRoute(builder: (_) => const SendScreen()),
            ),
            child: Container(
              padding: const EdgeInsets.symmetric(vertical: 18),
              decoration: BoxDecoration(
                color: const Color(0xFF4A9EFF),
                borderRadius: BorderRadius.circular(12),
              ),
              child: const Center(
                child: Text(
                  'Send',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 15,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: GestureDetector(
            onTap: () => Navigator.push(
              context,
              MaterialPageRoute(builder: (_) => const ReceiveScreen()),
            ),
            child: Container(
              padding: const EdgeInsets.symmetric(vertical: 18),
              decoration: BoxDecoration(
                color: const Color(0xFF2D2D4D),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: const Color(0xFF4A9EFF), width: 1),
              ),
              child: const Center(
                child: Text(
                  'Receive',
                  style: TextStyle(
                    color: Color(0xFF4A9EFF),
                    fontSize: 15,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}



