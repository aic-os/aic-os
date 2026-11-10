import '../models/coin.dart';

/// Каталог всех доступных монет в приложении.
/// aCOIN -- наши обёрнутые активы. Остальные -- нативные монеты 150 сетей.
const List<Coin> kCoinCatalog = [
  // ===== aCOIN (наши обёрнутые, появятся после моста) =====
  Coin(symbol: 'aETH',  name: 'AIC-wrapped ETH',  balance: '0', usdPrice: 3245.80, rubPrice: 308350,  decimals: 18, color: '#627EEA'),
  Coin(symbol: 'aBTC',  name: 'AIC-wrapped BTC',  balance: '0', usdPrice: 68500.00, rubPrice: 6507500, decimals: 8,  color: '#F7931A'),
  Coin(symbol: 'aUSDT', name: 'AIC-wrapped USDT', balance: '0', usdPrice: 1.0,     rubPrice: 95,      decimals: 6,  color: '#26A17B'),

  // ===== Нативные (топ-20) =====
  Coin(symbol: 'AIC',  name: 'AI Coin',    balance: '0', usdPrice: 10.0,    rubPrice: 950,        decimals: 18, color: '#4A9EFF'),
  Coin(symbol: 'ETH',  name: 'Ethereum',   balance: '0', usdPrice: 3245.80, rubPrice: 308350,     decimals: 18, color: '#627EEA'),
  Coin(symbol: 'BTC',  name: 'Bitcoin',    balance: '0', usdPrice: 68500.00, rubPrice: 6507500,   decimals: 8,  color: '#F7931A'),
  Coin(symbol: 'USDT', name: 'Tether',     balance: '0', usdPrice: 1.0,     rubPrice: 95,         decimals: 6,  color: '#26A17B'),
  Coin(symbol: 'BNB',  name: 'BNB',        balance: '0', usdPrice: 605.20,  rubPrice: 57494,      decimals: 18, color: '#F3BA2F'),
  Coin(symbol: 'SOL',  name: 'Solana',     balance: '0', usdPrice: 172.40,  rubPrice: 16378,      decimals: 9,  color: '#9945FF'),
  Coin(symbol: 'TRX',  name: 'TRON',       balance: '0', usdPrice: 0.24,    rubPrice: 22.8,       decimals: 6,  color: '#FF060A'),
  Coin(symbol: 'TON',  name: 'Toncoin',    balance: '0', usdPrice: 5.42,    rubPrice: 514.9,      decimals: 9,  color: '#0098EA'),
  Coin(symbol: 'SUI',  name: 'Sui',        balance: '0', usdPrice: 3.15,    rubPrice: 299.25,     decimals: 9,  color: '#4DA2FF'),
  Coin(symbol: 'APT',  name: 'Aptos',      balance: '0', usdPrice: 8.90,    rubPrice: 845.5,      decimals: 8,  color: '#0AE5A0'),
  Coin(symbol: 'XRP',  name: 'Ripple',     balance: '0', usdPrice: 0.62,    rubPrice: 58.9,       decimals: 6,  color: '#23292F'),
  Coin(symbol: 'DOGE', name: 'Dogecoin',   balance: '0', usdPrice: 0.17,    rubPrice: 16.15,      decimals: 8,  color: '#C2A633'),
  Coin(symbol: 'LTC',  name: 'Litecoin',   balance: '0', usdPrice: 84.30,   rubPrice: 8008.5,     decimals: 8,  color: '#345D9D'),
  Coin(symbol: 'ATOM', name: 'Cosmos',     balance: '0', usdPrice: 8.15,    rubPrice: 774.25,     decimals: 6,  color: '#2E3148'),
  Coin(symbol: 'AVAX', name: 'Avalanche',  balance: '0', usdPrice: 37.80,   rubPrice: 3591,       decimals: 18, color: '#E84142'),
  Coin(symbol: 'MATIC', name: 'Polygon',   balance: '0', usdPrice: 0.52,    rubPrice: 49.4,       decimals: 18, color: '#8247E5'),
  Coin(symbol: 'ARB',  name: 'Arbitrum',   balance: '0', usdPrice: 0.89,    rubPrice: 84.55,      decimals: 18, color: '#28A0F0'),
  Coin(symbol: 'OP',   name: 'Optimism',   balance: '0', usdPrice: 2.10,    rubPrice: 199.5,      decimals: 18, color: '#FF0420'),
  Coin(symbol: 'TIA',  name: 'Celestia',   balance: '0', usdPrice: 6.15,    rubPrice: 584.25,     decimals: 6,  color: '#7B2BF9'),
  Coin(symbol: 'INJ',  name: 'Injective',  balance: '0', usdPrice: 23.40,   rubPrice: 2223,       decimals: 18, color: '#00F2FE'),
  Coin(symbol: 'SEI',  name: 'Sei',        balance: '0', usdPrice: 0.32,    rubPrice: 30.4,       decimals: 6,  color: '#9E1F19'),
];

/// Найти монету в каталоге по символу.
Coin? findCoin(String symbol) {
  for (final c in kCoinCatalog) {
    if (c.symbol == symbol) return c;
  }
  return null;
}
