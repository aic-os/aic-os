import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../models/coin_catalog.dart';
import '../l10n/l10n_service.dart';
import '../services/swap_service.dart';

class SwapScreen extends StatefulWidget {
  final String initialFrom;
  final String initialTo;

  const SwapScreen({super.key, this.initialFrom = 'AIC', this.initialTo = 'ETH'});

  @override
  State<SwapScreen> createState() => _SwapScreenState();
}

class _SwapScreenState extends State<SwapScreen> {
  late String _fromSymbol;
  late String _toSymbol;
  final _amountCtrl = TextEditingController(text: '100');
  SwapQuote? _quote;
  bool _loading = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _fromSymbol = widget.initialFrom;
    _toSymbol = widget.initialTo;
    _amountCtrl.addListener(_onAmountChanged);
    _loadQuote();
  }

  @override
  void dispose() {
    _amountCtrl.dispose();
    super.dispose();
  }

  void _onAmountChanged() => _loadQuote();

  Future<void> _loadQuote() async {
    final amount = double.tryParse(_amountCtrl.text);
    if (amount == null || amount <= 0) {
      setState(() => _quote = null);
      return;
    }
    setState(() { _loading = true; _error = null; });
    final q = await SwapService.getQuote(
      fromSymbol: _fromSymbol,
      toSymbol: _toSymbol,
      amount: amount,
    );
    setState(() {
      _quote = q;
      _loading = false;
      if (q == null) _error = 'Quote unavailable';
    });
  }

  void _swapDirection() {
    setState(() {
      final t = _fromSymbol;
      _fromSymbol = _toSymbol;
      _toSymbol = t;
    });
    _loadQuote();
  }

  void _pickCoin({required bool isFrom}) {
    final current = isFrom ? _fromSymbol : _toSymbol;
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (_) => _CoinPickerSheet(
        current: current,
        exclude: isFrom ? _toSymbol : _fromSymbol,
        onPick: (symbol) {
          setState(() {
            if (isFrom) { _fromSymbol = symbol; } else { _toSymbol = symbol; }
          });
          _loadQuote();
        },
      ),
    );
  }

  String _formatOut(double v) {
    if (v >= 1000) return v.toStringAsFixed(2);
    if (v >= 1) return v.toStringAsFixed(4);
    return v.toStringAsFixed(6);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: Text(l10n.t('swap')),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          children: [
            _card(
              label: l10n.t('from'),
              symbol: _fromSymbol,
              amountCtrl: _amountCtrl,
              onPick: () => _pickCoin(isFrom: true),
            ),
            const SizedBox(height: 8),
            GestureDetector(
              onTap: _swapDirection,
              child: Container(
                width: 44, height: 44,
                decoration: BoxDecoration(
                  color: const Color(0xFF1A1A2D),
                  shape: BoxShape.circle,
                  border: Border.all(color: const Color(0xFF4A9EFF), width: 1),
                ),
                child: const Icon(Icons.swap_vert, color: Color(0xFF4A9EFF)),
              ),
            ),
            const SizedBox(height: 8),
            _card(
              label: l10n.t('to_estimated'),
              symbol: _toSymbol,
              amountCtrl: TextEditingController(
                text: _quote != null ? _formatOut(_quote!.amountOut) : '',
              ),
              readOnly: true,
              onPick: () => _pickCoin(isFrom: false),
            ),
            const SizedBox(height: 16),
            if (_quote != null) _quoteInfo(_quote!, l10n),
            if (_error != null) ...[
              const SizedBox(height: 8),
              Text(_error!, style: const TextStyle(color: Colors.redAccent, fontSize: 12)),
            ],
            const Spacer(),
            GestureDetector(
              onTap: _quote != null ? () => _onSwapPressed(l10n) : null,
              child: Container(
                width: double.infinity,
                padding: const EdgeInsets.symmetric(vertical: 18),
                decoration: BoxDecoration(
                  color: _quote != null ? const Color(0xFF4A9EFF) : Colors.grey,
                  borderRadius: BorderRadius.circular(14),
                ),
                child: Center(
                  child: Text(l10n.t('swap'),
                      style: const TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _card({
    required String label,
    required String symbol,
    required TextEditingController amountCtrl,
    bool readOnly = false,
    VoidCallback? onPick,
  }) {
    final coin = findCoin(symbol);
    final accent = _parseColor(coin?.color ?? '#4A9EFF');
    return Container(
      padding: const EdgeInsets.all(18),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color(0xFF2A2A3D)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label, style: const TextStyle(color: Colors.white54, fontSize: 11)),
          const SizedBox(height: 8),
          Row(
            children: [
              Expanded(
                child: TextField(
                  controller: amountCtrl,
                  readOnly: readOnly,
                  keyboardType: TextInputType.number,
                  style: const TextStyle(color: Colors.white, fontSize: 28, fontWeight: FontWeight.bold),
                  decoration: const InputDecoration(
                    border: InputBorder.none, isDense: true,
                    hintText: '0', hintStyle: TextStyle(color: Colors.white24, fontSize: 28),
                  ),
                ),
              ),
              GestureDetector(
                onTap: onPick,
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                  decoration: BoxDecoration(
                    color: accent.withOpacity(0.15),
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(color: accent.withOpacity(0.5)),
                  ),
                  child: Row(
                    children: [
                      Text(symbol, style: TextStyle(color: accent, fontWeight: FontWeight.bold, fontSize: 14)),
                      const SizedBox(width: 4),
                      Icon(Icons.expand_more, color: accent, size: 16),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Color _parseColor(String hex) {
    final clean = hex.replaceFirst('#', '');
    return Color(int.parse('FF$clean', radix: 16));
  }

  Widget _quoteInfo(SwapQuote q, L10nService l10n) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(10),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(l10n.t('rate'), style: const TextStyle(color: Colors.white54, fontSize: 12)),
          Text('1 ${q.fromSymbol} = ${q.price.toStringAsFixed(6)} ${q.toSymbol}',
              style: const TextStyle(color: Colors.white70, fontSize: 12)),
        ],
      ),
    );
  }

  void _onSwapPressed(L10nService l10n) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF1A1A2D),
        title: Text(l10n.t('swap'), style: const TextStyle(color: Colors.white)),
        content: Text(
          '${_amountCtrl.text} ${_fromSymbol} -> ${_quote!.amountOut.toStringAsFixed(6)} ${_toSymbol}\n\n'
          'Реальный swap -- следующий блок (1inch + подпись).',
          style: const TextStyle(color: Colors.white70, fontSize: 13),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text(l10n.t('ok'), style: const TextStyle(color: Color(0xFF4A9EFF))),
          ),
        ],
      ),
    );
  }
}

class _CoinPickerSheet extends StatefulWidget {
  final String current;
  final String exclude;
  final Function(String symbol) onPick;

  const _CoinPickerSheet({required this.current, required this.exclude, required this.onPick});

  @override
  State<_CoinPickerSheet> createState() => _CoinPickerSheetState();
}

class _CoinPickerSheetState extends State<_CoinPickerSheet> {
  String _search = '';

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();
    final filtered = kCoinCatalog.where((c) {
      if (c.symbol == widget.exclude) return false;
      if (_search.isEmpty) return true;
      final q = _search.toLowerCase();
      return c.symbol.toLowerCase().contains(q) || c.name.toLowerCase().contains(q);
    }).toList();

    return DraggableScrollableSheet(
      initialChildSize: 0.75,
      minChildSize: 0.5,
      maxChildSize: 0.95,
      expand: false,
      builder: (ctx, scrollCtrl) {
        return Container(
          decoration: const BoxDecoration(
            color: Color(0xFF1A1A2D),
            borderRadius: BorderRadius.vertical(top: Radius.circular(24)),
          ),
          child: Column(
            children: [
              const SizedBox(height: 12),
              Container(width: 40, height: 4,
                  decoration: BoxDecoration(color: Colors.white24, borderRadius: BorderRadius.circular(2))),
              const SizedBox(height: 16),
              Text(l10n.t('select_coin'),
                  style: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
              const SizedBox(height: 12),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: TextField(
                  style: const TextStyle(color: Colors.white),
                  onChanged: (v) => setState(() => _search = v),
                  decoration: InputDecoration(
                    hintText: l10n.t('search'),
                    hintStyle: const TextStyle(color: Colors.white38, fontSize: 13),
                    prefixIcon: const Icon(Icons.search, color: Colors.white54),
                    filled: true,
                    fillColor: const Color(0xFF0A0A0F),
                    border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(12), borderSide: BorderSide.none),
                  ),
                ),
              ),
              const SizedBox(height: 12),
              Expanded(
                child: ListView.builder(
                  controller: scrollCtrl,
                  itemCount: filtered.length,
                  itemBuilder: (_, i) {
                    final c = filtered[i];
                    final isCurrent = c.symbol == widget.current;
                    return ListTile(
                      leading: Container(
                        width: 36, height: 36,
                        decoration: BoxDecoration(
                          color: Color(int.parse('FF${c.color.replaceFirst('#', '')}', radix: 16)).withOpacity(0.15),
                          borderRadius: BorderRadius.circular(10),
                        ),
                        child: Center(
                          child: Text(
                            c.symbol.substring(0, c.symbol.length > 3 ? 3 : c.symbol.length),
                            style: TextStyle(
                                color: Color(int.parse('FF${c.color.replaceFirst('#', '')}', radix: 16)),
                                fontSize: 10, fontWeight: FontWeight.bold),
                          ),
                        ),
                      ),
                      title: Text(c.symbol, style: const TextStyle(color: Colors.white, fontSize: 14)),
                      subtitle: Text(c.name, style: const TextStyle(color: Colors.white54, fontSize: 11)),
                      trailing: isCurrent ? const Icon(Icons.check, color: Color(0xFF4A9EFF)) : null,
                      onTap: () { widget.onPick(c.symbol); Navigator.pop(context); },
                    );
                  },
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
