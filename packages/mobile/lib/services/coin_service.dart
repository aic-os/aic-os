import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../models/coin.dart';
import '../models/coin_catalog.dart';
import 'aic_rpc.dart';

class CoinService extends ChangeNotifier {
  static const _keySelected = 'aic_selected_coins_v1';
  static const _defaults = ['AIC', 'ETH', 'USDT', 'BTC'];

  final AicRpc rpc;
  final String address;

  List<Coin> _coins = [];
  List<Map<String, dynamic>> _transactions = [];
  bool _loading = false;
  String? _error;
  List<String> _selectedSymbols = List.from(_defaults);

  CoinService({AicRpc? rpc, required this.address}) : rpc = rpc ?? AicRpc();

  List<Coin> get coins => List.unmodifiable(_coins);
  List<Map<String, dynamic>> get transactions => List.unmodifiable(_transactions);
  bool get loading => _loading;
  String? get error => _error;
  List<String> get selectedSymbols => List.unmodifiable(_selectedSymbols);

  double get totalUsd => _coins.fold(0.0, (sum, c) => sum + c.valueUsd);
  double get totalRub => _coins.fold(0.0, (sum, c) => sum + c.valueRub);

  List<Coin> get availableToAdd =>
      kCoinCatalog.where((c) => !_selectedSymbols.contains(c.symbol)).toList();

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

      // 1) Баланс AIC
      final aicWei = await rpc.getBalance(addr);
      final aic = AicRpc.weiToAic(aicWei);

      // 2) Монеты из каталога
      final list = <Coin>[];
      for (final symbol in _selectedSymbols) {
        final t = findCoin(symbol);
        if (t == null) continue;
        list.add(Coin(
          symbol: t.symbol,
          name: t.name,
          balance: symbol == 'AIC' ? aic : t.balance,
          usdPrice: t.usdPrice,
          rubPrice: t.rubPrice,
          decimals: t.decimals,
          tokenAddress: t.tokenAddress,
          color: t.color,
        ));
      }
      _coins = list;

      // 3) История транзакций
      _transactions = await rpc.getAccountTransactions(addr, limit: 20);
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }
}
