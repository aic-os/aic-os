import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:provider/provider.dart';

import '../services/wallet_service.dart';

class ReceiveScreen extends StatelessWidget {
  const ReceiveScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final address = context.watch<WalletService>().address;
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: const Text('Receive AIC'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          children: [
            const SizedBox(height: 20),
            Container(
              padding: const EdgeInsets.all(20),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(20),
              ),
              child: QrImageView(
                data: address,
                version: QrVersions.auto,
                size: 280,
              ),
            ),
            const SizedBox(height: 30),
            const Text(
              'Your AIC address',
              style: TextStyle(color: Colors.white70, fontSize: 13),
            ),
            const SizedBox(height: 8),
            SelectableText(
              address,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 14,
                fontFamily: 'monospace',
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 20),
            GestureDetector(
              onTap: () {
                Clipboard.setData(ClipboardData(text: address));
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Address copied'),
                    backgroundColor: Color(0xFF4A9EFF),
                  ),
                );
              },
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 14),
                decoration: BoxDecoration(
                  color: const Color(0xFF4A9EFF),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: const Text(
                  'Copy address',
                  style: TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
