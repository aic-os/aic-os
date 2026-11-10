import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

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
    setState(() {
      _loading = true;
      _error = null;
    });
    try {
      final wallet = context.read<WalletService>();
      final info = await wallet.rpc.quantumBountyInfo();
      setState(() {
        _info = info;
        _loading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _loading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: const Text('Quantum Bounty'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _load,
          ),
        ],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator(color: Color(0xFF4A9EFF)))
          : _error != null
              ? _errorView()
              : _content(),
    );
  }

  Widget _errorView() {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Text('Error: $_error',
            style: const TextStyle(color: Colors.redAccent), textAlign: TextAlign.center),
      ),
    );
  }

  Widget _content() {
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
        _heroCard(totalPool),
        const SizedBox(height: 20),
        _modeCard(
          title: 'Mode A: Classic ECDSA',
          reward: classicReward,
          solved: classicSolved,
          description: 'Break secp256k1 (Bitcoin/Ethereum crypto) with a quantum computer. '
              'Sign a challenge message with the hidden private key -> 1M AIC.',
          icon: Icons.vpn_key,
          color: Color(0xFF4A9EFF),
        ),
        const SizedBox(height: 12),
        if (classicTarget != '0x0000000000000000000000000000000000000000' && classicTarget != '0x0')
          _targetCard(classicTarget),
        const SizedBox(height: 20),
        _modeCard(
          title: 'Mode B: PQ (ML-DSA-65)',
          reward: pqReward,
          solved: pqSolved,
          description: 'Break our post-quantum protection (Schnorr + Dilithium). '
              'Validators verify off-chain and vote -> 2M AIC.',
          icon: Icons.security,
          color: Color(0xFF9C6AFF),
        ),
        const SizedBox(height: 30),
        _infoCard(),
      ],
    );
  }

  Widget _heroCard(String totalPool) {
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        gradient: const LinearGradient(
          colors: [Color(0xFF2D1A4D), Color(0xFF1A2D4D)],
        ),
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: const Color(0xFF9C6AFF), width: 1),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.emoji_events, color: Colors.amber, size: 28),
              SizedBox(width: 8),
              Text('Total Prize Pool',
                  style: TextStyle(color: Colors.white70, fontSize: 14)),
            ],
          ),
          const SizedBox(height: 12),
          Row(
            crossAxisAlignment: CrossAxisAlignment.baseline,
            textBaseline: TextBaseline.alphabetic,
            children: [
              Text(totalPool,
                  style: const TextStyle(
                    color: Colors.white, fontSize: 36, fontWeight: FontWeight.bold,
                  )),
              const SizedBox(width: 8),
              const Text('AIC',
                  style: TextStyle(color: Colors.white70, fontSize: 16)),
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
  }) {
    return Container(
      padding: const EdgeInsets.all(18),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: solved ? Colors.green : color.withOpacity(0.3),
          width: 1,
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, color: color, size: 22),
              const SizedBox(width: 10),
              Expanded(
                child: Text(title,
                    style: const TextStyle(
                      color: Colors.white, fontSize: 15, fontWeight: FontWeight.bold,
                    )),
              ),
              if (solved)
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                  decoration: BoxDecoration(
                    color: Colors.green,
                    borderRadius: BorderRadius.circular(6),
                  ),
                  child: const Text('SOLVED',
                      style: TextStyle(color: Colors.white, fontSize: 10, fontWeight: FontWeight.bold)),
                ),
            ],
          ),
          const SizedBox(height: 12),
          Row(
            children: [
              Text(reward,
                  style: TextStyle(
                    color: color, fontSize: 22, fontWeight: FontWeight.bold,
                  )),
              const SizedBox(width: 6),
              const Text('AIC',
                  style: TextStyle(color: Colors.white54, fontSize: 13)),
            ],
          ),
          const SizedBox(height: 12),
          Text(description,
              style: const TextStyle(
                color: Colors.white54, fontSize: 12, height: 1.4,
              )),
        ],
      ),
    );
  }

  Widget _targetCard(String target) {
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
          const Text('Target address (hidden key):',
              style: TextStyle(color: Colors.white54, fontSize: 11)),
          const SizedBox(height: 6),
          SelectableText(
            target,
            style: const TextStyle(
              color: Colors.greenAccent, fontSize: 12, fontFamily: 'monospace',
            ),
          ),
        ],
      ),
    );
  }

  Widget _infoCard() {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(12),
      ),
      child: const Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('How it works',
              style: TextStyle(color: Colors.white70, fontSize: 13, fontWeight: FontWeight.bold)),
          SizedBox(height: 8),
          Text(
            'The prize is locked in the AIC Chain contract. When someone breaks '
            'the crypto (using a quantum computer or a math breakthrough), they '
            'claim the reward. Until then, the pool grows as AIC Chain generates blocks.',
            style: TextStyle(color: Colors.white38, fontSize: 11, height: 1.4),
          ),
        ],
      ),
    );
  }
}

// Хелпер, чтобы не тянуть aic_rpc целиком через импорт в этом файле
class AicRpcHelper {
  static String hexToAic(String hex) {
    final clean = hex.replaceFirst('0x', '');
    if (clean.isEmpty) return '0';
    final n = BigInt.parse(clean, radix: 16);
    final div = BigInt.from(10).pow(18);
    return (n ~/ div).toString();
  }
}
