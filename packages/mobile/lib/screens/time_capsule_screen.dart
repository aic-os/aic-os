import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../services/account_service.dart';
import '../services/time_capsule_service.dart';
import '../services/aic_rpc.dart';
import '../l10n/l10n_service.dart';
import '../services/wallet_service.dart';
import '../services/wasm_bridge.dart';
import '../services/settings_service.dart';

class TimeCapsuleScreen extends StatefulWidget {
  const TimeCapsuleScreen({super.key});

  @override
  State<TimeCapsuleScreen> createState() => _TimeCapsuleScreenState();
}

class _TimeCapsuleScreenState extends State<TimeCapsuleScreen> {
  List<TimeCapsule> _capsules = [];
  bool _loading = true;
  String? _error;

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
      final acc = context.read<AccountService>().current;
      final settings = context.read<SettingsService>();
      if (acc == null) {
        setState(() { _loading = false; });
        return;
      }
      final service = TimeCapsuleService(
        rpc: AicRpc(rpcUrl: settings.rpcUrl),
      );
      _capsules = await service.listForAddress(acc.address);
    } catch (e) {
      _error = e.toString();
    } finally {
      setState(() => _loading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: Text(context.watch<L10nService>().t('time_capsule')),
        actions: [
          IconButton(icon: const Icon(Icons.refresh), onPressed: _load),
        ],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator(color: Color(0xFF4A9EFF)))
          : _error != null
              ? Center(child: Text('Error: $_error', style: const TextStyle(color: Colors.redAccent)))
              : _buildList(),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: _showCreateDialog,
        backgroundColor: const Color(0xFF4A9EFF),
        icon: const Icon(Icons.add, color: Colors.white),
        label: Text(context.watch<L10nService>().t('new_capsule'), style: const TextStyle(color: Colors.white)),
      ),
    );
  }

  Widget _buildList() {
    if (_capsules.isEmpty) {
      return const Center(
        child: Padding(
          padding: EdgeInsets.all(20),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.lock_clock, color: Colors.white24, size: 64),
              SizedBox(height: 16),
              Text('No capsules yet',
                  style: TextStyle(color: Colors.white54, fontSize: 16)),
              SizedBox(height: 8),
              Text('Lock AIC until a specific date',
                  style: TextStyle(color: Colors.white38, fontSize: 12)),
            ],
          ),
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: _capsules.length,
      itemBuilder: (_, i) => _capsuleCard(_capsules[i]),
    );
  }

  Widget _capsuleCard(TimeCapsule c) {
    final l10n = context.watch<L10nService>();
    Color statusColor;
    String statusText;
    if (c.claimed) {
      statusColor = Colors.green;
      statusText = l10n.t('claimed');
    } else if (c.cancelled) {
      statusColor = Colors.red;
      statusText = l10n.t('cancelled');
    } else if (c.isUnlocked) {
      statusColor = Colors.green;
      statusText = l10n.t('unlocked');
    } else {
      statusColor = const Color(0xFF4A9EFF);
      statusText = l10n.t('locked');
    }

    final daysLeft = c.unlockAt.difference(DateTime.now()).inDays;

    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: statusColor.withOpacity(0.3)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.lock_clock, color: statusColor, size: 22),
              const SizedBox(width: 8),
              Text('${l10n.t('capsule')} #${c.id}',
                  style: const TextStyle(
                      color: Colors.white, fontSize: 15, fontWeight: FontWeight.bold)),
              const Spacer(),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                decoration: BoxDecoration(
                  color: statusColor.withOpacity(0.15),
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(color: statusColor.withOpacity(0.5)),
                ),
                child: Text(statusText,
                    style: TextStyle(
                        color: statusColor,
                        fontSize: 10,
                        fontWeight: FontWeight.bold)),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text('${c.balanceAic.toStringAsFixed(2)} AIC',
              style: const TextStyle(
                  color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
          const SizedBox(height: 8),
          Text('${l10n.t('to')}: ${c.recipient.substring(0, 6)}...${c.recipient.substring(c.recipient.length - 4)}',
              style: const TextStyle(color: Colors.white54, fontSize: 12)),
          const SizedBox(height: 4),
          Text(
            c.isUnlocked
                ? 'Unlocked ${c.unlockAt.toString().substring(0, 16)}'
                : 'Unlocks in $daysLeft days (${c.unlockAt.toString().substring(0, 10)})',
            style: const TextStyle(color: Colors.white54, fontSize: 12),
          ),
        ],
      ),
    );
  }

  void _showCreateDialog() {
    final toCtrl = TextEditingController();
    final amountCtrl = TextEditingController(text: '100');
    final daysCtrl = TextEditingController(text: '30');

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: const Text('New capsule', style: TextStyle(color: Colors.white)),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: toCtrl,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                labelText: 'Recipient (0x...)',
                labelStyle: TextStyle(color: Colors.white54),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: amountCtrl,
              style: const TextStyle(color: Colors.white),
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(
                labelText: 'Amount (AIC)',
                labelStyle: TextStyle(color: Colors.white54),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: daysCtrl,
              style: const TextStyle(color: Colors.white),
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(
                labelText: 'Lock for (days)',
                labelStyle: TextStyle(color: Colors.white54),
              ),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              Navigator.pop(ctx);
              _createCapsule(toCtrl.text.trim(), amountCtrl.text.trim(), daysCtrl.text.trim());
            },
            child: const Text('Create', style: TextStyle(color: Color(0xFF4A9EFF))),
          ),
        ],
      ),
    );
  }

  /// ABI-encode create(address,uint64) -- selector 0xe8a56cc1
  String _buildCreateCalldata(String recipient, int unlockAt) {
    const sel = 'e8a56cc1';
    final addr = recipient.replaceFirst('0x', '').toLowerCase();
    if (addr.length != 40) {
      throw Exception('Invalid recipient address (need 0x + 40 hex)');
    }
    final addrPadded = addr.padLeft(64, '0');
    final unlockHex = unlockAt.toRadixString(16).padLeft(16, '0');
    final unlockPadded = unlockHex.padLeft(64, '0');
    return '0x$sel$addrPadded$unlockPadded';
  }

  Future<void> _createCapsule(String to, String amountStr, String daysStr) async {
    try {
      final wallet = context.read<WalletService>();
      final acc = context.read<AccountService>().current;
      if (acc == null) throw Exception('No active account');

      final amount = double.tryParse(amountStr);
      final days = int.tryParse(daysStr);
      if (amount == null || amount <= 0) throw Exception('Invalid amount');
      if (days == null || days < 1) throw Exception('Invalid days');

      final unlockTs = DateTime.now().add(Duration(days: days)).millisecondsSinceEpoch ~/ 1000;
      final calldata = _buildCreateCalldata(to, unlockTs);
      final valueWei = BigInt.from(amount * 1e18);
      final nonce = await wallet.rpc.getNonce(acc.address);

      final signed = await WasmBridge.signLegacyTx(
        privkeyHex: acc.privateKey,
        toHex: '0x0000000000000000000000000000000000c4b5e0',
        valueHex: valueWei.toRadixString(16),
        nonce: nonce,
        gasPriceHex: '3b9aca00',
        gasLimit: 200000,
        dataHex: calldata,
        chainId: 2026,
      );

      final txHash = await wallet.rpc.sendRawTransaction(signed['raw_hex'] as String);

      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Capsule created! TX: ' + txHash.substring(0, 18) + '...'),
          backgroundColor: Colors.green,
          duration: const Duration(seconds: 3),
        ),
      );

      await Future.delayed(const Duration(seconds: 4));
      if (mounted) _load();
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Error: $e'),
          backgroundColor: Colors.redAccent,
          duration: const Duration(seconds: 5),
        ),
      );
    }
  }
}




