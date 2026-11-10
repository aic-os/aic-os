import 'dart:convert';
import 'package:http/http.dart' as http;

/// Swap service: обмен монет через 1inch API (или DEX-агрегатор).
///
/// MVP: для AIC <-> других монет используем фиктивный курс из каталога
/// (реальный swap для AIC будет через наш AIC DEX, когда он появится).
/// Для ETH/USDT/BTC -> реальный 1inch API (Ethereum mainnet).
class SwapService {
  static const String _oneInchBase = 'https://api.1inch.dev/swap/v6.0/1';

  /// Получить цену обмена: сколько получим за amountIn.
  /// Возвращает (amountOut, price).
  static Future<SwapQuote?> getQuote({
    required String fromSymbol,
    required String toSymbol,
    required double amount,
  }) async {
    try {
      // AIC -- наша монета, курс из каталога (фикс 10 USD)
      if (fromSymbol == 'AIC' || toSymbol == 'AIC') {
        return _fakeAicQuote(fromSymbol, toSymbol, amount);
      }

      // Для остальных -- реальный 1inch (Ethereum mainnet)
      // Адреса токенов
      final fromAddr = _tokenAddress(fromSymbol);
      final toAddr = _tokenAddress(toSymbol);
      if (fromAddr == null || toAddr == null) {
        return _fakeAicQuote(fromSymbol, toSymbol, amount);
      }

      // 1inch API: /quote?src=...&dst=...&amount=...
      final amountInWei = (amount * 1e18).toInt();
      final url = Uri.parse(
          '$_oneInchBase/quote?src=$fromAddr&dst=$toAddr&amount=$amountInWei');

      final resp = await http.get(url).timeout(const Duration(seconds: 10));
      if (resp.statusCode != 200) return null;

      final json = jsonDecode(resp.body) as Map<String, dynamic>;
      final amountOut = json['toAmount'] as String?;
      if (amountOut == null) return null;

      final out = double.parse(amountOut) / 1e18;
      return SwapQuote(
        fromSymbol: fromSymbol,
        toSymbol: toSymbol,
        amountIn: amount,
        amountOut: out,
        price: out / amount,
      );
    } catch (_) {
      return null;
    }
  }

  /// Фиктивный quote для AIC (стартовая цена $10).
  static SwapQuote _fakeAicQuote(String from, String to, double amount) {
    const aicUsd = 10.0;
    const ethUsd = 3245.80;
    const btcUsd = 68500.0;
    const usdtUsd = 1.0;

    double priceUsd(String symbol) {
      switch (symbol) {
        case 'AIC': return aicUsd;
        case 'ETH': return ethUsd;
        case 'BTC': return btcUsd;
        case 'USDT': return usdtUsd;
        default: return 1.0;
      }
    }

    final fromUsd = priceUsd(from);
    final toUsd = priceUsd(to);
    final out = amount * fromUsd / toUsd;

    return SwapQuote(
      fromSymbol: from,
      toSymbol: to,
      amountIn: amount,
      amountOut: out,
      price: out / amount,
    );
  }

  static String? _tokenAddress(String symbol) {
    // USDT/ETH/BTC wrapped addresses на Ethereum mainnet
    switch (symbol) {
      case 'USDT': return '0xdAC17F958D2ee523a2206206994597C13D831ec7';
      case 'ETH':  return '0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE'; // native
      case 'WBTC': return '0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599';
      default:     return null;
    }
  }
}

class SwapQuote {
  final String fromSymbol;
  final String toSymbol;
  final double amountIn;
  final double amountOut;
  final double price;

  SwapQuote({
    required this.fromSymbol,
    required this.toSymbol,
    required this.amountIn,
    required this.amountOut,
    required this.price,
  });
}
