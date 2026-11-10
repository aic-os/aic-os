import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../services/settings_service.dart';
import '../services/account_service.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: const Text('Settings'),
      ),
      body: Consumer<SettingsService>(
        builder: (context, settings, _) {
          final acc = context.watch<AccountService>().current;
          return ListView(
            padding: const EdgeInsets.all(16),
            children: [
              // RPC endpoint
              _section('Network'),
              _tile(
                icon: Icons.dns,
                title: 'RPC endpoint',
                subtitle: settings.rpcUrl,
                onTap: () => _editRpcUrl(context, settings),
              ),
              _tile(
                icon: Icons.numbers,
                title: 'Chain ID',
                subtitle: '2026',
                onTap: null,
              ),
              const SizedBox(height: 16),
              _section('Account'),
              _tile(
                icon: Icons.account_circle,
                title: 'Current account',
                subtitle: acc?.name ?? 'none',
                onTap: null,
              ),
              _tile(
                icon: Icons.key,
                title: 'Seed phrase',
                subtitle: 'Coming soon (secured)',
                onTap: () {},
              ),
              const SizedBox(height: 16),
              _section('Preferences'),
              _tile(
                icon: Icons.dark_mode,
                title: 'Theme',
                subtitle: 'Dark (default)',
                onTap: () {},
              ),
              _tile(
                icon: Icons.language,
                title: 'Language',
                subtitle: 'Russian',
                onTap: () {},
              ),
              const SizedBox(height: 16),
              _section('About'),
              _tile(
                icon: Icons.info_outline,
                title: 'Version',
                subtitle: '0.3.0 (dev)',
                onTap: null,
              ),
            ],
          );
        },
      ),
    );
  }

  Widget _section(String title) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(8, 12, 8, 8),
      child: Text(title,
          style: const TextStyle(
            color: Colors.white38,
            fontSize: 11,
            fontWeight: FontWeight.w600,
            letterSpacing: 1.2,
          )),
    );
  }

  Widget _tile({
    required IconData icon,
    required String title,
    required String subtitle,
    required VoidCallback? onTap,
  }) {
    return ListTile(
      leading: Icon(icon, color: const Color(0xFF4A9EFF)),
      title: Text(title, style: const TextStyle(color: Colors.white, fontSize: 14)),
      subtitle: Text(subtitle,
          style: const TextStyle(color: Colors.white54, fontSize: 11)),
      trailing: onTap != null
          ? const Icon(Icons.chevron_right, color: Colors.white38, size: 18)
          : null,
      onTap: onTap,
    );
  }

  void _editRpcUrl(BuildContext context, SettingsService settings) {
    final ctrl = TextEditingController(text: settings.rpcUrl);
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: const Text('RPC endpoint', style: TextStyle(color: Colors.white)),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: ctrl,
              style: const TextStyle(color: Colors.white, fontFamily: 'monospace'),
              decoration: const InputDecoration(
                labelText: 'http://...',
                labelStyle: TextStyle(color: Colors.white54),
              ),
            ),
            const SizedBox(height: 12),
            const Text(
              'Web: http://127.0.0.1:8545\nMobile: http://192.168.1.X:8545 (IP компа в Wi-Fi)',
              style: TextStyle(color: Colors.white38, fontSize: 11),
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
              settings.setRpcUrl(ctrl.text);
              Navigator.pop(ctx);
            },
            child: const Text('Save', style: TextStyle(color: Color(0xFF4A9EFF))),
          ),
        ],
      ),
    );
  }
}
