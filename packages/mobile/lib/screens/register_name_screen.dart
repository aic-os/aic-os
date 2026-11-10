import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../services/wallet_service.dart';
import '../services/wasm_bridge.dart';

class RegisterNameScreen extends StatefulWidget {
  const RegisterNameScreen({super.key});

  @override
  State<RegisterNameScreen> createState() => _RegisterNameScreenState();
}

class _RegisterNameScreenState extends State<RegisterNameScreen> {
  final _nameController = TextEditingController();
  bool _sending = false;
  String? _error;
  String? _success;

  static const _alicePrivkey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
  static const _aicidContract = '0x0000000000000000000000000000000000a1c1d0';

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  Future<void> _register() async {
    setState(() {
      _sending = true;
      _error = null;
      _success = null;
    });

    try {
      final wallet = context.read<WalletService>();
      final rpc = wallet.rpc;

      String name = _nameController.text.trim();
      if (name.startsWith('@')) name = name.substring(1);
      if (name.isEmpty) throw Exception('Name is empty');
      if (name.length > 32) throw Exception('Name is too long (max 32)');

      // 1. ABI-encode register(name, address) через WASM
      final calldata = await WasmBridge.encodeAicidRegister(name, wallet.address);

      // 2. nonce
      final nonce = await rpc.getNonce(wallet.address);

      // 3. подпись: to = AICID contract, value = 10 AIC
      final tenAicHex = BigInt.from(10 * 1e18.toInt()).toRadixString(16); // "8ac7230489e80000"
      final signed = await WasmBridge.signLegacyTx(
        privkeyHex: _alicePrivkey,
        toHex: _aicidContract,
        valueHex: tenAicHex,
        nonce: nonce,
        gasPriceHex: '3b9aca00',
        gasLimit: 200000,
        dataHex: calldata,
        chainId: 2026,
      );

      // 4. broadcast
      final txHash = await rpc.sendRawTransaction(signed['raw_hex'] as String);

      setState(() {
        _success = 'Registered! TX: $txHash';
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
        title: const Text('Register AIC-ID'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'Your name (without @):',
              style: TextStyle(color: Colors.white70, fontSize: 13),
            ),
            const SizedBox(height: 8),
            TextField(
              controller: _nameController,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                hintText: 'alice',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            const Text(
              'Cost: 10 AIC (goes to treasury)',
              style: TextStyle(color: Colors.white54, fontSize: 12),
            ),
            const SizedBox(height: 24),
            GestureDetector(
              onTap: _sending ? null : _register,
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
                      : const Text(
                          'Register (10 AIC)',
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
