import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../l10n/l10n_service.dart';
import '../services/account_service.dart';
import '../models/account.dart';

class AccountHeader extends StatelessWidget {
  final VoidCallback onSettings;
  const AccountHeader({super.key, required this.onSettings});

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();
    return Consumer<AccountService>(
      builder: (context, acc, _) {
        final current = acc.current;
        if (current == null) return const SizedBox.shrink();

        return Row(
          children: [
            GestureDetector(
              onTap: () => _showAccountSwitcher(context, acc, l10n),
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                decoration: BoxDecoration(
                  color: const Color(0xFF1A1A2D),
                  borderRadius: BorderRadius.circular(14),
                  border: Border.all(color: const Color(0xFF2A2A3D)),
                ),
                child: Row(
                  children: [
                    Container(
                      width: 32, height: 32,
                      decoration: BoxDecoration(
                        gradient: const LinearGradient(colors: [Color(0xFF4A9EFF), Color(0xFF9C6AFF)]),
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Center(
                        child: Text(
                          current.name.isNotEmpty ? current.name[0].toUpperCase() : '?',
                          style: const TextStyle(color: Colors.white, fontSize: 14, fontWeight: FontWeight.bold),
                        ),
                      ),
                    ),
                    const SizedBox(width: 8),
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(current.name, style: const TextStyle(color: Colors.white, fontSize: 13, fontWeight: FontWeight.w600)),
                        Text(current.shortAddress, style: const TextStyle(color: Colors.white38, fontSize: 10)),
                      ],
                    ),
                    const SizedBox(width: 4),
                    const Icon(Icons.expand_more, color: Colors.white54, size: 18),
                  ],
                ),
              ),
            ),
            const Spacer(),
            IconButton(
              onPressed: onSettings,
              icon: const Icon(Icons.settings_outlined, color: Colors.white70),
            ),
          ],
        );
      },
    );
  }

  void _showAccountSwitcher(BuildContext context, AccountService acc, L10nService l10n) {
    showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF1A1A2D),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (ctx) {
        return SafeArea(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const SizedBox(height: 12),
              Container(width: 40, height: 4, decoration: BoxDecoration(color: Colors.white24, borderRadius: BorderRadius.circular(2))),
              const SizedBox(height: 16),
              Text(l10n.t('accounts'), style: const TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
              const SizedBox(height: 12),
              ...acc.accounts.asMap().entries.map((e) => _accountTile(context, acc, e.key, e.value)),
              const Divider(color: Color(0xFF2A2A3D), height: 1),
              ListTile(
                leading: const Icon(Icons.add_circle_outline, color: Color(0xFF4A9EFF)),
                title: Text(l10n.t('add_account'), style: const TextStyle(color: Color(0xFF4A9EFF))),
                onTap: () {
                  Navigator.pop(context);
                  _showAddDialog(context, acc, l10n);
                },
              ),
              const SizedBox(height: 8),
            ],
          ),
        );
      },
    );
  }

  Widget _accountTile(BuildContext context, AccountService acc, int index, Account a) {
    final isCurrent = index == acc.currentIndex;
    return ListTile(
      leading: CircleAvatar(
        backgroundColor: isCurrent ? const Color(0xFF4A9EFF) : const Color(0xFF2A2A3D),
        child: Text(a.name[0].toUpperCase(), style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
      ),
      title: Text(a.name, style: const TextStyle(color: Colors.white)),
      subtitle: Text(a.shortAddress, style: const TextStyle(color: Colors.white54, fontSize: 11)),
      trailing: isCurrent ? const Icon(Icons.check, color: Color(0xFF4A9EFF)) : null,
      onTap: () {
        acc.switchTo(index);
        Navigator.pop(context);
      },
    );
  }

  void _showAddDialog(BuildContext context, AccountService acc, L10nService l10n) {
    final nameCtrl = TextEditingController();
    final keyCtrl = TextEditingController();

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: Text(l10n.t('add_account'), style: const TextStyle(color: Colors.white)),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: nameCtrl,
              style: const TextStyle(color: Colors.white),
              decoration: InputDecoration(
                labelText: l10n.t('name'),
                labelStyle: const TextStyle(color: Colors.white54),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: keyCtrl,
              style: const TextStyle(color: Colors.white),
              decoration: InputDecoration(
                labelText: l10n.t('private_key'),
                labelStyle: const TextStyle(color: Colors.white54),
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'MVP: вставьте приватный ключ. Позже: импорт мнемоники / новый кошелёк.',
              style: const TextStyle(color: Colors.white38, fontSize: 11),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text(l10n.t('cancel')),
          ),
          TextButton(
            onPressed: () {
              if (nameCtrl.text.isEmpty || keyCtrl.text.isEmpty) return;
              acc.addAccount(Account(
                name: nameCtrl.text.trim(),
                address: '0x0000000000000000000000000000000000000000',
                privateKey: keyCtrl.text.trim(),
                mnemonic: '',
              ));
              Navigator.pop(ctx);
            },
            child: Text(l10n.t('add'), style: const TextStyle(color: Color(0xFF4A9EFF))),
          ),
        ],
      ),
    );
  }
}
