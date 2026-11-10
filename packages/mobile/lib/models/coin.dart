class Coin {
  final String symbol;       // "AIC", "ETH", "USDT", "BTC"
  final String name;         // "AI Coin", "Ethereum", ...
  final String balance;      // "1 019 949.00" (строка для форматирования)
  final double usdPrice;     // 10.00
  final double rubPrice;     // 950.00
  final int decimals;        // 18
  final String? tokenAddress;// null для native, 0x... для ERC-20
  final String color;        // "#4A9EFF" (акцент)

  const Coin({
    required this.symbol,
    required this.name,
    required this.balance,
    required this.usdPrice,
    required this.rubPrice,
    required this.decimals,
    this.tokenAddress,
    this.color = '#4A9EFF',
  });

  double get balanceNum => double.tryParse(balance.replaceAll(' ', '')) ?? 0;
  double get valueUsd => balanceNum * usdPrice;
  double get valueRub => balanceNum * rubPrice;
}
