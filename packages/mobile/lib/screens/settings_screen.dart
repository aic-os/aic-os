import 'package:flutter/material.dart';

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
      body: ListView(
        children: [
          ListTile(
            leading: const Icon(Icons.dns, color: Color(0xFF4A9EFF)),
            title: const Text('RPC endpoint', style: TextStyle(color: Colors.white)),
            subtitle: const Text('http://127.0.0.1:8545',
                style: TextStyle(color: Colors.white54, fontSize: 11)),
          ),
          ListTile(
            leading: const Icon(Icons.dark_mode, color: Color(0xFF4A9EFF)),
            title: const Text('Theme', style: TextStyle(color: Colors.white)),
            subtitle: const Text('Dark (default)', style: TextStyle(color: Colors.white54, fontSize: 11)),
          ),
          ListTile(
            leading: const Icon(Icons.security, color: Color(0xFF4A9EFF)),
            title: const Text('Security', style: TextStyle(color: Colors.white)),
            subtitle: const Text('Coming soon', style: TextStyle(color: Colors.white54, fontSize: 11)),
          ),
        ],
      ),
    );
  }
}
