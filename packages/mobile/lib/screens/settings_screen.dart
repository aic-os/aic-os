import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../services/settings_service.dart';
import '../services/account_service.dart';
import '../l10n/l10n_service.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: Text(l10n.t('settings')),
      ),
      body: Consumer<SettingsService>(
        builder: (context, settings, _) {
          final acc = context.watch<AccountService>().current;
          return ListView(
            padding: const EdgeInsets.all(16),
            children: [
              _section(l10n.t('network')),
              _tile(
                icon: Icons.dns,
                title: l10n.t('rpc_endpoint'),
                subtitle: settings.rpcUrl,
                onTap: () => _editRpcUrl(context, settings, l10n),
              ),
              _tile(
                icon: Icons.numbers,
                title: l10n.t('chain_id'),
                subtitle: '2026',
                onTap: null,
              ),
              const SizedBox(height: 16),
              _section(l10n.t('current_account')),
              _tile(
                icon: Icons.account_circle,
                title: l10n.t('current_account'),
                subtitle: acc?.name ?? 'none',
                onTap: null,
              ),
              _tile(
                icon: Icons.key,
                title: l10n.t('seed_phrase'),
                subtitle: l10n.t('secured_soon'),
                onTap: () {},
              ),
              const SizedBox(height: 16),
              _section(l10n.t('preferences')),
              _languageTile(context, l10n),
              _tile(
                icon: Icons.dark_mode,
                title: l10n.t('theme'),
                subtitle: l10n.t('dark'),
                onTap: () {},
              ),
              const SizedBox(height: 16),
              _section(l10n.t('about')),
              _tile(
                icon: Icons.info_outline,
                title: l10n.t('version'),
                subtitle: '0.3.0 (dev)',
                onTap: null,
              ),
            ],
          );
        },
      ),
    );
  }

  Widget _languageTile(BuildContext context, L10nService l10n) {
    return ListTile(
      leading: const Icon(Icons.language, color: Color(0xFF4A9EFF)),
      title: Text(l10n.t('language'),
          style: const TextStyle(color: Colors.white, fontSize: 14)),
      subtitle: Text(
        l10n.isRu ? 'Русский' : 'English',
        style: const TextStyle(color: Colors.white54, fontSize: 11),
      ),
      trailing: const Icon(Icons.chevron_right, color: Colors.white38, size: 18),
      onTap: () => _pickLanguage(context, l10n),
    );
  }

  void _pickLanguage(BuildContext context, L10nService l10n) {
    showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF1A1A2D),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (ctx) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const SizedBox(height: 16),
            ListTile(
              leading: const Text('🇷🇺', style: TextStyle(fontSize: 24)),
              title: const Text('Русский',
                  style: TextStyle(color: Colors.white, fontSize: 15)),
              trailing: l10n.isRu
                  ? const Icon(Icons.check, color: Color(0xFF4A9EFF))
                  : null,
              onTap: () {
                l10n.setLang(L10nService.RU);
                Navigator.pop(ctx);
              },
            ),
            ListTile(
              leading: const Text('🇬🇧', style: TextStyle(fontSize: 24)),
              title: const Text('English',
                  style: TextStyle(color: Colors.white, fontSize: 15)),
              trailing: !l10n.isRu
                  ? const Icon(Icons.check, color: Color(0xFF4A9EFF))
                  : null,
              onTap: () {
                l10n.setLang(L10nService.EN);
                Navigator.pop(ctx);
              },
            ),
            const SizedBox(height: 16),
          ],
        ),
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

  void _editRpcUrl(BuildContext context, SettingsService settings, L10nService l10n) {
    final ctrl = TextEditingController(text: settings.rpcUrl);
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: Text(l10n.t('rpc_endpoint'), style: const TextStyle(color: Colors.white)),
        content: TextField(
          controller: ctrl,
          style: const TextStyle(color: Colors.white, fontFamily: 'monospace'),
          decoration: const InputDecoration(
            labelText: 'http://...',
            labelStyle: TextStyle(color: Colors.white54),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text(l10n.t('cancel')),
          ),
          TextButton(
            onPressed: () {
              settings.setRpcUrl(ctrl.text);
              Navigator.pop(ctx);
            },
            child: Text(l10n.t('save'),
                style: const TextStyle(color: Color(0xFF4A9EFF))),
          ),
        ],
      ),
    );
  }
}
