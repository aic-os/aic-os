import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../services/aic_rpc.dart';
import '../services/wallet_service.dart';
import '../services/wasm_bridge.dart';

class SendScreen extends StatefulWidget {
  const SendScreen({super.key});

  @override
  State<SendScreen> createState() => _SendScreenState();
}

class _SendScreenState extends State<SendScreen> {
  final _toController = TextEditingController();
  final _amountController = TextEditingController();
  bool _sending = false;
  String? _error;
  String? _success;

  // ВРЕМЕННО: hardcoded privkey Alice (Hardhat-0) для теста.
  // В production: брать из WASM (сгенерировать / восстановить).
  static const _alicePrivkey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';

  @override
  void dispose() {
    _toController.dispose();
    _amountController.dispose();
    super.dispose();
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
        if (resolved == null) {
          throw Exception('Name $to not found');
        }
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
      setState(() {
        _error = e.toString();
      });
    } finally {
      setState(() {
        _sending = false;
      });
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
            const SizedBox(height: 24),
            GestureDetector(
              onTap: _sending ? null : _send,
              child: Container(
                width: double.infinity,
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
                      : const Text(
                          'Send',
                          style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold),
                        ),
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

