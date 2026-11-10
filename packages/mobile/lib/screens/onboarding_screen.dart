import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';

import '../l10n/l10n_service.dart';
import '../services/seed_service.dart';
import 'home_screen.dart';

class OnboardingScreen extends StatefulWidget {
  const OnboardingScreen({super.key});

  @override
  State<OnboardingScreen> createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  int _step = 0;
  bool _loading = false;
  String? _error;
  String? _generatedMnemonic;
  final _importCtrl = TextEditingController();

  Future<void> _createWallet() async {
    setState(() { _loading = true; _error = null; });
    try {
      final result = await SeedService.createNew();
      setState(() {
        _generatedMnemonic = result['mnemonic'];
        _step = 2;
        _loading = false;
      });
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  Future<void> _importWallet() async {
    final phrase = _importCtrl.text.trim();
    if (phrase.split(' ').length < 12) {
      setState(() => _error = 'Введите 12 или 24 слова');
      return;
    }
    setState(() { _loading = true; _error = null; });
    try {
      await SeedService.importFromMnemonic(phrase);
      if (!mounted) return;
      Navigator.of(context).pushReplacement(
        MaterialPageRoute(builder: (_) => const HomeScreen()),
      );
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  void _copyMnemonic() {
    final phrase = _generatedMnemonic ?? '';
    Clipboard.setData(ClipboardData(text: phrase));
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Сид-фраза скопирована в буфер'),
        backgroundColor: Color(0xFF4A9EFF),
        duration: Duration(seconds: 2),
      ),
    );
  }

  void _downloadMnemonic() {
    final phrase = _generatedMnemonic ?? '';
    final content = 'AIC-OS Seed Phrase\n'
        '====================\n\n'
        'Сохраните эту фразу в безопасном месте.\n'
        'Никогда никому не показывайте.\n\n'
        'Фраза:\n$phrase\n\n'
        'Создано: ${DateTime.now().toIso8601String()}\n';
    Clipboard.setData(ClipboardData(text: content));
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Фраза с инструкцией скопирована. Сохрани в заметках.'),
        backgroundColor: Color(0xFF9C6AFF),
        duration: Duration(seconds: 4),
      ),
    );
  }

  void _finish() {
    Navigator.of(context).pushReplacement(
      MaterialPageRoute(builder: (_) => const HomeScreen()),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: _step == 0 ? _welcome(l10n) : (_step == 1 ? _import(l10n) : _showPhrase(l10n)),
        ),
      ),
    );
  }

  Widget _welcome(L10nService l10n) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Container(
          width: 80, height: 80,
          decoration: BoxDecoration(
            gradient: const LinearGradient(colors: [Color(0xFF4A9EFF), Color(0xFF9C6AFF)]),
            borderRadius: BorderRadius.circular(24),
          ),
          child: const Center(child: Text('AIC', style: TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold))),
        ),
        const SizedBox(height: 24),
        const Text('AIC-OS', textAlign: TextAlign.center,
            style: TextStyle(color: Colors.white, fontSize: 32, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Операционная система для крипто-активов',
            textAlign: TextAlign.center, style: TextStyle(color: Colors.white54, fontSize: 14)),
        const SizedBox(height: 60),
        GestureDetector(
          onTap: _loading ? null : _createWallet,
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 18),
            decoration: BoxDecoration(color: const Color(0xFF4A9EFF), borderRadius: BorderRadius.circular(14)),
            child: Center(
              child: _loading
                  ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
                  : const Text('Создать кошелёк', style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
            ),
          ),
        ),
        const SizedBox(height: 12),
        GestureDetector(
          onTap: () => setState(() => _step = 1),
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 18),
            decoration: BoxDecoration(
              color: const Color(0xFF1A1A2D),
              borderRadius: BorderRadius.circular(14),
              border: Border.all(color: const Color(0xFF4A9EFF)),
            ),
            child: const Center(child: Text('Импортировать', style: TextStyle(color: Color(0xFF4A9EFF), fontSize: 16, fontWeight: FontWeight.bold))),
          ),
        ),
        if (_error != null) ...[
          const SizedBox(height: 16),
          Text(_error!, style: const TextStyle(color: Colors.redAccent, fontSize: 12), textAlign: TextAlign.center),
        ],
      ],
    );
  }

  Widget _import(L10nService l10n) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        IconButton(
          icon: const Icon(Icons.arrow_back, color: Colors.white),
          onPressed: () => setState(() => _step = 0),
          alignment: Alignment.centerLeft,
        ),
        const SizedBox(height: 16),
        const Text('Импорт кошелька', style: TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Введите 12 или 24 слова через пробел', style: TextStyle(color: Colors.white54, fontSize: 13)),
        const SizedBox(height: 24),
        TextField(
          controller: _importCtrl,
          maxLines: 4,
          style: const TextStyle(color: Colors.white, fontFamily: 'monospace'),
          decoration: const InputDecoration(
            hintText: 'abandon abandon abandon ...',
            hintStyle: TextStyle(color: Colors.white24),
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 16),
        GestureDetector(
          onTap: _loading ? null : _importWallet,
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 18),
            decoration: BoxDecoration(color: const Color(0xFF4A9EFF), borderRadius: BorderRadius.circular(14)),
            child: Center(
              child: _loading
                  ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
                  : const Text('Импортировать', style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
            ),
          ),
        ),
        if (_error != null) ...[
          const SizedBox(height: 16),
          Text(_error!, style: const TextStyle(color: Colors.redAccent, fontSize: 12), textAlign: TextAlign.center),
        ],
      ],
    );
  }

  Widget _showPhrase(L10nService l10n) {
    final words = (_generatedMnemonic ?? '').split(' ');
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SizedBox(height: 24),
        const Text('Сохраните сид-фразу', style: TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Никогда не показывайте её никому. Запишите на бумагу, не сохраняйте в облаке.',
            style: TextStyle(color: Colors.amber, fontSize: 13, height: 1.4)),
        const SizedBox(height: 24),
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: const Color(0xFF1A1A2D),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color(0xFF4A9EFF)),
          ),
          child: Wrap(
            spacing: 8, runSpacing: 8,
            children: List.generate(words.length, (i) {
              return Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
                decoration: BoxDecoration(color: const Color(0xFF0A0A0F), borderRadius: BorderRadius.circular(8)),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text('${i + 1}. ', style: const TextStyle(color: Colors.white38, fontSize: 11)),
                    Text(words[i], style: const TextStyle(color: Colors.white, fontSize: 13, fontFamily: 'monospace')),
                  ],
                ),
              );
            }),
          ),
        ),
        const SizedBox(height: 16),
        Row(
          children: [
            Expanded(
              child: GestureDetector(
                onTap: _copyMnemonic,
                child: Container(
                  padding: const EdgeInsets.symmetric(vertical: 14),
                  decoration: BoxDecoration(
                    color: const Color(0xFF1A1A2D),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: const Color(0xFF4A9EFF)),
                  ),
                  child: const Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.copy, color: Color(0xFF4A9EFF), size: 18),
                      SizedBox(width: 6),
                      Text('Скопировать', style: TextStyle(color: Color(0xFF4A9EFF), fontWeight: FontWeight.bold)),
                    ],
                  ),
                ),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: GestureDetector(
                onTap: _downloadMnemonic,
                child: Container(
                  padding: const EdgeInsets.symmetric(vertical: 14),
                  decoration: BoxDecoration(
                    color: const Color(0xFF1A1A2D),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: const Color(0xFF9C6AFF)),
                  ),
                  child: const Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.download, color: Color(0xFF9C6AFF), size: 18),
                      SizedBox(width: 6),
                      Text('Скачать', style: TextStyle(color: Color(0xFF9C6AFF), fontWeight: FontWeight.bold)),
                    ],
                  ),
                ),
              ),
            ),
          ],
        ),
        const SizedBox(height: 24),
        GestureDetector(
          onTap: _finish,
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 18),
            decoration: BoxDecoration(color: const Color(0xFF4A9EFF), borderRadius: BorderRadius.circular(14)),
            child: const Center(child: Text('Я сохранил фразу', style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold))),
          ),
        ),
      ],
    );
  }
}
