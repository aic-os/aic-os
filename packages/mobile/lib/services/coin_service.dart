import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../models/coin.dart';
import '../models/coin_catalog.dart';
import 'aic_rpc.dart';

/// Список монет в кошельке. AIC -- реальный баланс из RPC.
/// Остальные -- из каталога (баланс 0, пока не подключены к RPC).
/// Пользователь может добавлять/удалять монеты из слайдера.
class CoinService extends ChangeNotifier {
  static const _keySelected = 'aic_selected_coins_v1';
  static const _defaults = ['AIC', 'ETH', 'USDT', 'BTC'];

  final AicRpc rpc;
  final String address;

  List<Coin> _coins = [];
  bool _loading = false;
  String? _error;
  List<String> _selectedSymbols = List.from(_defaults);

  CoinService({AicRpc? rpc, required this.address})
      : rpc = rpc ?? AicRpc();

  List<Coin> get coins => List.unmodifiable(_coins);
  bool get loading => _loading;
  String? get error => _error;
  List<String> get selectedSymbols => List.unmodifiable(_selectedSymbols);

  double get totalUsd => _coins.fold(0.0, (sum, c) => sum + c.valueUsd);
  double get totalRub => _coins.fold(0.0, (sum, c) => sum + c.valueRub);

  /// Монеты, которых ещё нет в слайдере (для модалки каталога).
  List<Coin> get availableToAdd {
    return kCoinCatalog.where((c) => !_selectedSymbols.contains(c.symbol)).toList();
  }

  Future<void> loadSelected() async {
    final prefs = await SharedPreferences.getInstance();
    final raw = prefs.getString(_keySelected);
    if (raw != null) {
      try {
        final list = (jsonDecode(raw) as List).cast<String>();
        if (list.isNotEmpty) _selectedSymbols = list;
      } catch (_) {}
    }
  }

  Future<void> _saveSelected() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_keySelected, jsonEncode(_selectedSymbols));
  }

  Future<void> addCoin(String symbol) async {
    if (_selectedSymbols.contains(symbol)) return;
    _selectedSymbols.add(symbol);
    await _saveSelected();
    await refresh();
  }

  Future<void> removeCoin(String symbol) async {
    // AIC нельзя удалить
    if (symbol == 'AIC') return;
    _selectedSymbols.remove(symbol);
    await _saveSelected();
    await refresh();
  }

  Future<void> refresh({String? newAddress}) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      final addr = newAddress ?? address;
      // Реальные балансы через RPC
      final aicWei = await rpc.getBalance(addr);
      final aic = AicRpc.weiToAic(aicWei);

      final list = <Coin>[];
      for (final symbol in _selectedSymbols) {
        final template = findCoin(symbol);
        if (template == null) continue;

        // Для AIC -- реальный баланс, для остальных пока 0
        final balance = symbol == 'AIC' ? aic : template.balance;

        list.add(Coin(
          symbol: template.symbol,
          name: template.name,
          balance: balance,
          usdPrice: template.usdPrice,
          rubPrice: template.rubPrice,
          decimals: template.decimals,
          tokenAddress: template.tokenAddress,
          color: template.color,
        ));
      }
      _coins = list;
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }
}
