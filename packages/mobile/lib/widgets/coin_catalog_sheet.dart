import 'package:flutter/material.dart';

import '../models/coin.dart';

/// Модалка с каталогом монет для добавления в слайдер.
/// Поиск + список с иконками, курс, кнопка Add.
class CoinCatalogSheet extends StatefulWidget {
  final List<Coin> available;
  final Function(String symbol) onAdd;

  const CoinCatalogSheet({
    super.key,
    required this.available,
    required this.onAdd,
  });

  @override
  State<CoinCatalogSheet> createState() => _CoinCatalogSheetState();
}

class _CoinCatalogSheetState extends State<CoinCatalogSheet> {
  String _search = '';

  Color _parseColor(String hex) {
    final clean = hex.replaceFirst('#', '');
    return Color(int.parse('FF$clean', radix: 16));
  }

  @override
  Widget build(BuildContext context) {
    final filtered = widget.available.where((c) {
      if (_search.isEmpty) return true;
      final q = _search.toLowerCase();
      return c.symbol.toLowerCase().contains(q) || c.name.toLowerCase().contains(q);
    }).toList();

    return DraggableScrollableSheet(
      initialChildSize: 0.85,
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
              Container(
                width: 40, height: 4,
                decoration: BoxDecoration(
                  color: Colors.white24, borderRadius: BorderRadius.circular(2),
                ),
              ),
              const SizedBox(height: 16),
              const Text('Add coin',
                  style: TextStyle(
                    color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold,
                  )),
              const SizedBox(height: 12),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: TextField(
                  style: const TextStyle(color: Colors.white),
                  onChanged: (v) => setState(() => _search = v),
                  decoration: InputDecoration(
                    hintText: 'Search by symbol or name',
                    hintStyle: const TextStyle(color: Colors.white38, fontSize: 13),
                    prefixIcon: const Icon(Icons.search, color: Colors.white54),
                    filled: true,
                    fillColor: const Color(0xFF0A0A0F),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                      borderSide: BorderSide.none,
                    ),
                  ),
                ),
              ),
              const SizedBox(height: 12),
              Expanded(
                child: filtered.isEmpty
                    ? const Center(
                        child: Text('Nothing found',
                            style: TextStyle(color: Colors.white54)),
                      )
                    : ListView.builder(
                        controller: scrollCtrl,
                        padding: const EdgeInsets.symmetric(horizontal: 12),
                        itemCount: filtered.length,
                        itemBuilder: (_, i) => _row(filtered[i]),
                      ),
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _row(Coin coin) {
    final accent = _parseColor(coin.color);
    return ListTile(
      leading: Container(
        width: 40, height: 40,
        decoration: BoxDecoration(
          color: accent.withOpacity(0.15),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: accent.withOpacity(0.5)),
        ),
        child: Center(
          child: Text(coin.symbol.substring(0, coin.symbol.length > 3 ? 3 : coin.symbol.length),
              style: TextStyle(color: accent, fontSize: 10, fontWeight: FontWeight.bold)),
        ),
      ),
      title: Text(coin.symbol,
          style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600)),
      subtitle: Text(coin.name,
          style: const TextStyle(color: Colors.white54, fontSize: 11)),
      trailing: IconButton(
        icon: const Icon(Icons.add_circle, color: Color(0xFF4A9EFF)),
        onPressed: () {
          widget.onAdd(coin.symbol);
          Navigator.pop(context);
        },
      ),
    );
  }
}
