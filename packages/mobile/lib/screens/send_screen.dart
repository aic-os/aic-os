import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../l10n/l10n_service.dart';
import '../services/account_service.dart';
import '../services/wallet_service.dart';
import '../services/wasm_bridge.dart';
import '../services/voice_service.dart';

class SendScreen extends StatefulWidget {
  const SendScreen({super.key});

  @override
  State<SendScreen> createState() => _SendScreenState();
}

class _SendScreenState extends State<SendScreen> {
  final _toController = TextEditingController();
  final _amountController = TextEditingController();
  bool _sending = false;
  bool _listening = false;
  String _voicePartial = '';
  String? _error;
  String? _success;

  @override
  void dispose() {
    _toController.dispose();
    _amountController.dispose();
    super.dispose();
  }

  Future<void> _startVoice() async {
    if (_listening) {
      await VoiceService.stop();
      setState(() => _listening = false);
      return;
    }
    final ok = await VoiceService.init();
    if (!ok) {
      setState(() => _error = 'Микрофон недоступен');
      return;
    }
    setState(() { _listening = true; _voicePartial = ''; _error = null; });
    await VoiceService.start(onResult: (text, isFinal) {
      setState(() => _voicePartial = text);
      if (isFinal) {
        _applyVoiceCommand(text);
        setState(() => _listening = false);
      }
    });
  }

  void _applyVoiceCommand(String text) {
    final cmd = VoiceService.parseCommand(text);
    if (cmd == null) return;
    if (cmd.amount != null) _amountController.text = cmd.amount!.toString();
    if (cmd.recipient != null) _toController.text = cmd.recipient!;
    if (cmd.isComplete) _showConfirmDialog(cmd);
  }

  void _showConfirmDialog(VoiceCommand cmd) {
    final l10n = context.read<L10nService>();
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: Text(l10n.t('confirm'), style: const TextStyle(color: Colors.white)),
        content: Text('${cmd.amount} ${cmd.currency ?? "AIC"} → ${cmd.recipient}',
            style: const TextStyle(color: Colors.white70)),
        actions: [
          TextButton(onPressed: () => Navigator.pop(ctx), child: Text(l10n.t('cancel'))),
          TextButton(
            onPressed: () { Navigator.pop(ctx); _send(); },
            child: Text(l10n.t('confirm'), style: const TextStyle(color: Colors.greenAccent)),
          ),
        ],
      ),
    );
  }

  Future<void> _send() async {
    final l10n = context.read<L10nService>();
    setState(() { _sending = true; _error = null; _success = null; });
    try {
      final acc = context.read<AccountService>().current;
      final wallet = context.read<WalletService>();
      if (acc == null) throw Exception('No account');

      String to = _toController.text.trim();
      if (to.startsWith('@')) {
        final resolved = await wallet.rpc.resolveName(to.substring(1));
        if (resolved == null) throw Exception('${l10n.t('name_not_found')}: $to');
        to = resolved;
      }
      if (!to.startsWith('0x') || to.length != 42) {
        throw Exception('${l10n.t('invalid_address')}: $to');
      }
      final amountAic = double.parse(_amountController.text.trim());
      final amountWei = BigInt.from(amountAic * 1e18);
      final nonce = await wallet.rpc.getNonce(acc.address);

      final signed = await WasmBridge.signLegacyTx(
        privkeyHex: acc.privateKey,
        toHex: to,
        valueHex: amountWei.toRadixString(16),
        nonce: nonce,
        gasPriceHex: '3b9aca00',
        gasLimit: 21000,
        dataHex: '',
        chainId: 2026,
      );
      final txHash = await wallet.rpc.sendRawTransaction(signed['raw_hex'] as String);
      setState(() => _success = '${l10n.t('sent_ok')} $txHash');
      await wallet.refresh();
    } catch (e) {
      setState(() => _error = e.toString());
    } finally {
      setState(() => _sending = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: Text(l10n.t('send_aic')),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            TextField(
              controller: _toController,
              style: const TextStyle(color: Colors.white),
              decoration: InputDecoration(
                labelText: l10n.t('recipient'),
                border: const OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _amountController,
              style: const TextStyle(color: Colors.white),
              keyboardType: TextInputType.number,
              decoration: InputDecoration(
                labelText: l10n.t('amount_aic'),
                border: const OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            if (!kIsWeb) ...[
              GestureDetector(
                onTap: _startVoice,
                child: Container(
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  decoration: BoxDecoration(
                    color: _listening ? Colors.redAccent.withOpacity(0.2) : const Color(0xFF1A1A2D),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: _listening ? Colors.redAccent : const Color(0xFF9C6AFF)),
                  ),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(_listening ? Icons.stop : Icons.mic,
                          color: _listening ? Colors.redAccent : const Color(0xFF9C6AFF)),
                      const SizedBox(width: 8),
                      Text(_listening ? 'Listening...' : 'Voice command',
                          style: TextStyle(
                            color: _listening ? Colors.redAccent : const Color(0xFF9C6AFF),
                            fontWeight: FontWeight.bold,
                          )),
                    ],
                  ),
                ),
              ),
              if (_voicePartial.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text('"$_voicePartial"',
                    style: const TextStyle(color: Colors.white54, fontSize: 12)),
              ],
              const SizedBox(height: 16),
            ],
            GestureDetector(
              onTap: _sending ? null : _send,
              child: Container(
                padding: const EdgeInsets.symmetric(vertical: 18),
                decoration: BoxDecoration(
                  color: _sending ? Colors.grey : const Color(0xFF4A9EFF),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Center(
                  child: _sending
                      ? const SizedBox(width: 20, height: 20,
                          child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
                      : Text(l10n.t('send'),
                          style: const TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
                ),
              ),
            ),
            if (_error != null) ...[
              const SizedBox(height: 16),
              Text(_error!, style: const TextStyle(color: Colors.redAccent)),
            ],
            if (_success != null) ...[
              const SizedBox(height: 16),
              Text(_success!, style: const TextStyle(color: Colors.greenAccent)),
            ],
          ],
        ),
      ),
    );
  }
}
