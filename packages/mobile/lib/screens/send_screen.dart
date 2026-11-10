import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

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

  static const _alicePrivkey =
      '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';

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
      setState(() => _error = 'Микрофон недоступен (нет разрешения?)');
      return;
    }

    setState(() {
      _listening = true;
      _voicePartial = '';
      _error = null;
    });

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

    if (cmd.amount != null) {
      _amountController.text = cmd.amount!.toString();
    }
    if (cmd.recipient != null) {
      _toController.text = cmd.recipient!;
    }

    if (cmd.isComplete) {
      _showConfirmDialog(cmd);
    } else {
      setState(() {
        _error = 'Не удалось распознать получателя или сумму. Попробуй: "Send 100 AIC to alice"';
      });
    }
  }

  void _showConfirmDialog(VoiceCommand cmd) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: const Text('Confirm voice command',
            style: TextStyle(color: Colors.white)),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Heard:',
                style: TextStyle(color: Colors.white54, fontSize: 11)),
            Text('"${cmd.rawText}"',
                style: const TextStyle(
                    color: Colors.white70, fontSize: 13, fontStyle: FontStyle.italic)),
            const SizedBox(height: 12),
            _row('Amount', '${cmd.amount} ${cmd.currency ?? "AIC"}'),
            _row('To', cmd.recipient ?? '?'),
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
              _send();
            },
            child: const Text('Confirm', style: TextStyle(color: Colors.greenAccent)),
          ),
        ],
      ),
    );
  }

  Widget _row(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        children: [
          SizedBox(width: 70, child: Text(label,
              style: const TextStyle(color: Colors.white54, fontSize: 12))),
          Expanded(child: Text(value,
              style: const TextStyle(color: Colors.white, fontSize: 13))),
        ],
      ),
    );
  }

  Future<void> _send() async {
    setState(() {
      _sending = true;
      _error = null;
      _success = null;
    });

    try {
      final wallet = context.read<WalletService>();
      final rpc = wallet.rpc;

      String to = _toController.text.trim();
      if (to.startsWith('@')) {
        final resolved = await rpc.resolveName(to.substring(1));
        if (resolved == null) throw Exception('Name $to not found');
        to = resolved;
      }
      if (!to.startsWith('0x') || to.length != 42) {
        throw Exception('Invalid address: $to');
      }

      final amountAic = double.parse(_amountController.text.trim());
      final amountWei = BigInt.from(amountAic * 1e18);
      final nonce = await rpc.getNonce(wallet.address);

      final signed = await WasmBridge.signLegacyTx(
        privkeyHex: _alicePrivkey,
        toHex: to,
        valueHex: amountWei.toRadixString(16),
        nonce: nonce,
        gasPriceHex: '3b9aca00',
        gasLimit: 21000,
        dataHex: '',
        chainId: 2026,
      );

      final txHash = await rpc.sendRawTransaction(signed['raw_hex'] as String);

      setState(() {
        _success = 'Sent! TX: $txHash';
      });
      await wallet.refresh();
    } catch (e) {
      setState(() => _error = e.toString());
    } finally {
      setState(() => _sending = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: const Text('Send AIC'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            TextField(
              controller: _toController,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                labelText: 'Recipient (@name or 0x...)',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _amountController,
              style: const TextStyle(color: Colors.white),
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(
                labelText: 'Amount (AIC)',
                border: OutlineInputBorder(),
              ),
            ),
            if (!kIsWeb) ...[
              const SizedBox(height: 20),
              GestureDetector(
                onTap: _startVoice,
                child: Container(
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  decoration: BoxDecoration(
                    color: _listening
                        ? Colors.redAccent.withOpacity(0.2)
                        : const Color(0xFF1A1A2D),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(
                      color: _listening ? Colors.redAccent : const Color(0xFF9C6AFF),
                      width: 1,
                    ),
                  ),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        _listening ? Icons.stop : Icons.mic,
                        color: _listening ? Colors.redAccent : const Color(0xFF9C6AFF),
                      ),
                      const SizedBox(width: 8),
                      Text(
                        _listening ? 'Listening... tap to stop' : 'Voice command',
                        style: TextStyle(
                          color: _listening ? Colors.redAccent : const Color(0xFF9C6AFF),
                          fontSize: 14,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              if (_voicePartial.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text('"$_voicePartial"',
                    style: const TextStyle(
                        color: Colors.white54, fontSize: 12, fontStyle: FontStyle.italic)),
              ],
            ],
            const SizedBox(height: 16),
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
                      ? const SizedBox(
                          width: 20, height: 20,
                          child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2),
                        )
                      : const Text('Send',
                          style: TextStyle(
                              color: Colors.white,
                              fontSize: 16,
                              fontWeight: FontWeight.bold)),
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

