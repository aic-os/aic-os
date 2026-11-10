import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../models/coin.dart';

class CryptoSlider extends StatefulWidget {
  final List<Coin> coins;
  final String address;
  final Function(Coin coin)? onTapCoin;
  final VoidCallback? onAddCoin;
  final Function(Coin coin)? onLongPressCoin;

  const CryptoSlider({
    super.key,
    required this.coins,
    required this.address,
    this.onTapCoin,
    this.onAddCoin,
    this.onLongPressCoin,
  });

  @override
  State<CryptoSlider> createState() => _CryptoSliderState();
}

class _CryptoSliderState extends State<CryptoSlider> {
  final PageController _pageCtrl = PageController(viewportFraction: 0.82, initialPage: 0);
  int _currentPage = 0;

  @override
  void dispose() {
    _pageCtrl.dispose();
    super.dispose();
  }

  Color _parseColor(String hex) {
    final clean = hex.replaceFirst('#', '');
    return Color(int.parse('FF$clean', radix: 16));
  }

  @override
  Widget build(BuildContext context) {
    if (widget.coins.isEmpty) {
      return Container(
        height: 210,
        margin: const EdgeInsets.symmetric(horizontal: 20),
        decoration: BoxDecoration(
          color: const Color(0xFF1A1A2D),
          borderRadius: BorderRadius.circular(20),
        ),
        child: const Center(
          child: CircularProgressIndicator(color: Color(0xFF4A9EFF)),
        ),
      );
    }

    return Column(
      children: [
        SizedBox(
          height: 210,
          child: PageView.builder(
            controller: _pageCtrl,
            itemCount: widget.coins.length + 1,
            onPageChanged: (i) => setState(() => _currentPage = i),
            itemBuilder: (context, i) {
              if (i == widget.coins.length) {
                return Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 6),
                  child: GestureDetector(
                    onTap: widget.onAddCoin,
                    child: _addCard(),
                  ),
                );
              }
              final coin = widget.coins[i];
              return Padding(
                padding: const EdgeInsets.symmetric(horizontal: 4),
                child: GestureDetector(
                  onTap: () => widget.onTapCoin?.call(coin),
                  onLongPress: () => widget.onLongPressCoin?.call(coin),
                  child: _coinCard(coin),
                ),
              );
            },
          ),
        ),
        const SizedBox(height: 12),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: List.generate(widget.coins.length + 1, (i) {
            final active = i == _currentPage;
            return AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              margin: const EdgeInsets.symmetric(horizontal: 3),
              width: active ? 20 : 6,
              height: 6,
              decoration: BoxDecoration(
                color: active ? const Color(0xFF4A9EFF) : Colors.white24,
                borderRadius: BorderRadius.circular(3),
              ),
            );
          }),
        ),
      ],
    );
  }

  Widget _addCard() {
    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: const Color(0xFF2A2A3D), width: 1),
      ),
      child: const Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.add_circle_outline, color: Color(0xFF4A9EFF), size: 48),
            SizedBox(height: 12),
            Text('Add coin',
                style: TextStyle(color: Colors.white70, fontSize: 14)),
            SizedBox(height: 4),
            Text('23 available',
                style: TextStyle(color: Colors.white38, fontSize: 11)),
          ],
        ),
      ),
    );
  }

  Widget _coinCard(Coin coin) {
    final accent = _parseColor(coin.color);
    final usd = coin.valueUsd.toStringAsFixed(2);
    final rub = coin.valueRub.toStringAsFixed(0);
    final addrShort = widget.address.length > 12
        ? '${widget.address.substring(0, 8)}...${widget.address.substring(widget.address.length - 4)}'
        : widget.address;

    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            accent.withOpacity(0.25),
            const Color(0xFF1A1A2D),
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: accent.withOpacity(0.4), width: 1),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Container(
                width: 40, height: 40,
                decoration: BoxDecoration(
                  color: accent.withOpacity(0.2),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: accent, width: 1),
                ),
                child: Center(
                  child: Text(
                    coin.symbol.substring(0, coin.symbol.length > 3 ? 3 : coin.symbol.length),
                    style: TextStyle(
                      color: accent, fontSize: 11, fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ),
              const SizedBox(width: 10),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(coin.symbol,
                      style: const TextStyle(
                        color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold,
                      )),
                  Text(coin.name,
                      style: const TextStyle(color: Colors.white54, fontSize: 11)),
                ],
              ),
              const Spacer(),
              Icon(Icons.chevron_right, color: Colors.white38, size: 20),
            ],
          ),
          const Spacer(),
          Text(coin.balance,
              style: const TextStyle(
                color: Colors.white, fontSize: 32, fontWeight: FontWeight.bold,
              )),
          const SizedBox(height: 4),
          Text('\$' + usd + '  ·  RUB ' + rub,
              style: const TextStyle(color: Colors.white70, fontSize: 14)),
          const SizedBox(height: 10),
          GestureDetector(
            onTap: () {
              Clipboard.setData(ClipboardData(text: widget.address));
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Address copied'),
                  duration: Duration(seconds: 1),
                  backgroundColor: Color(0xFF4A9EFF),
                ),
              );
            },
            child: Row(
              children: [
                const Icon(Icons.copy, color: Colors.white38, size: 12),
                const SizedBox(width: 4),
                Text(addrShort,
                    style: const TextStyle(
                      color: Colors.white54, fontSize: 11, fontFamily: 'monospace',
                    )),
              ],
            ),
          ),
        ],
      ),
    );
  }
}




