import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../l10n/l10n_service.dart';
import '../services/wallet_service.dart';

class QuantumBountyScreen extends StatefulWidget {
  const QuantumBountyScreen({super.key});

  @override
  State<QuantumBountyScreen> createState() => _QuantumBountyScreenState();
}

class _QuantumBountyScreenState extends State<QuantumBountyScreen> {
  Map<String, dynamic>? _info;
  String? _error;
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() { _loading = true; _error = null; });
    try {
      final wallet = context.read<WalletService>();
      final info = await wallet.rpc.quantumBountyInfo();
      setState(() { _info = info; _loading = false; });
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: Text(l10n.t('quantum_bounty')),
        actions: [
          IconButton(icon: const Icon(Icons.refresh), onPressed: _load),
        ],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator(color: Color(0xFF4A9EFF)))
          : _error != null
              ? Center(child: Text('${l10n.t('error')}: $_error',
                  style: const TextStyle(color: Colors.redAccent)))
              : _content(l10n),
    );
  }

  Widget _content(L10nService l10n) {
    final info = _info!;
    final classicReward = AicRpcHelper.hexToAic(info['classicReward'] ?? '0x0');
    final pqReward = AicRpcHelper.hexToAic(info['pqReward'] ?? '0x0');
    final totalPool = AicRpcHelper.hexToAic(info['totalPrizePool'] ?? '0x0');
    final classicSolved = info['classicSolved'] == true;
    final pqSolved = info['pqSolved'] == true;
    final classicTarget = info['classicTarget'] ?? '0x0';

    return ListView(
      padding: const EdgeInsets.all(20),
      children: [
        _heroCard(totalPool, l10n),
        const SizedBox(height: 20),
        _modeCard(
          title: l10n.t('mode_a_classic'),
          reward: classicReward,
          solved: classicSolved,
          description: l10n.t('break_classic'),
          icon: Icons.vpn_key,
          color: const Color(0xFF4A9EFF),
          l10n: l10n,
        ),
        const SizedBox(height: 12),
        if (classicTarget != '0x0000000000000000000000000000000000000000' && classicTarget != '0x0')
          _targetCard(classicTarget, l10n),
        const SizedBox(height: 20),
        _modeCard(
          title: l10n.t('mode_b_pq'),
          reward: pqReward,
          solved: pqSolved,
          description: l10n.t('break_pq'),
          icon: Icons.security,
          color: const Color(0xFF9C6AFF),
          l10n: l10n,
        ),
        const SizedBox(height: 30),
        _infoCard(l10n),
      ],
    );
  }

  Widget _heroCard(String totalPool, L10nService l10n) {
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        gradient: const LinearGradient(colors: [Color(0xFF2D1A4D), Color(0xFF1A2D4D)]),
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: const Color(0xFF9C6AFF), width: 1),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              const Icon(Icons.emoji_events, color: Colors.amber, size: 28),
              const SizedBox(width: 8),
              Text(l10n.t('total_prize_pool'),
                  style: const TextStyle(color: Colors.white70, fontSize: 14)),
            ],
          ),
          const SizedBox(height: 12),
          Row(
            crossAxisAlignment: CrossAxisAlignment.baseline,
            textBaseline: TextBaseline.alphabetic,
            children: [
              Text(totalPool, style: const TextStyle(color: Colors.white, fontSize: 36, fontWeight: FontWeight.bold)),
              const SizedBox(width: 8),
              const Text('AIC', style: TextStyle(color: Colors.white70, fontSize: 16)),
            ],
          ),
        ],
      ),
    );
  }

  Widget _modeCard({
    required String title,
    required String reward,
    required bool solved,
    required String description,
    required IconData icon,
    required Color color,
    required L10nService l10n,
  }) {
    return Container(
      padding: const EdgeInsets.all(18),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: solved ? Colors.green : color.withOpacity(0.3), width: 1),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, color: color, size: 22),
              const SizedBox(width: 10),
              Expanded(
                child: Text(title, style: const TextStyle(color: Colors.white, fontSize: 15, fontWeight: FontWeight.bold)),
              ),
              if (solved)
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                  decoration: BoxDecoration(color: Colors.green, borderRadius: BorderRadius.circular(6)),
                  child: Text(l10n.t('solved'),
                      style: const TextStyle(color: Colors.white, fontSize: 10, fontWeight: FontWeight.bold)),
                ),
            ],
          ),
          const SizedBox(height: 12),
          Row(
            children: [
              Text(reward, style: TextStyle(color: color, fontSize: 22, fontWeight: FontWeight.bold)),
              const SizedBox(width: 6),
              const Text('AIC', style: TextStyle(color: Colors.white54, fontSize: 13)),
            ],
          ),
          const SizedBox(height: 12),
          Text(description, style: const TextStyle(color: Colors.white54, fontSize: 12, height: 1.4)),
        ],
      ),
    );
  }

  Widget _targetCard(String target, L10nService l10n) {
    return Container(
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: const Color(0xFF0A1A0A),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: const Color(0xFF3A6D3A)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(l10n.t('target_address'),
              style: const TextStyle(color: Colors.white54, fontSize: 11)),
          const SizedBox(height: 6),
          SelectableText(target,
              style: const TextStyle(color: Colors.greenAccent, fontSize: 12, fontFamily: 'monospace')),
        ],
      ),
    );
  }

  Widget _infoCard(L10nService l10n) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(l10n.t('how_it_works'),
              style: const TextStyle(color: Colors.white70, fontSize: 13, fontWeight: FontWeight.bold)),
          const SizedBox(height: 8),
          Text(l10n.t('how_it_works_text'),
              style: const TextStyle(color: Colors.white38, fontSize: 11, height: 1.4)),
        ],
      ),
    );
  }
}

class AicRpcHelper {
  static String hexToAic(String hex) {
    final clean = hex.replaceFirst('0x', '');
    if (clean.isEmpty) return '0';
    final n = BigInt.parse(clean, radix: 16);
    final div = BigInt.from(10).pow(18);
    return (n ~/ div).toString();
  }
}
