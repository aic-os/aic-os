import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../models/account.dart';

/// Управление аккаунтами: список, текущий, добавление, переключение.
/// Persist в SharedPreferences (JSON).
class AccountService extends ChangeNotifier {
  static const _keyAccounts = 'aic_accounts_v1';
  static const _keyCurrentIndex = 'aic_current_account_index';

  final List<Account> _accounts = [];
  int _currentIndex = 0;
  bool _loaded = false;

  List<Account> get accounts => List.unmodifiable(_accounts);
  Account? get current => _accounts.isEmpty ? null : _accounts[_currentIndex];
  int get currentIndex => _currentIndex;
  bool get loaded => _loaded;

  Future<void> load() async {
    final prefs = await SharedPreferences.getInstance();
    final raw = prefs.getString(_keyAccounts);
    final idx = prefs.getInt(_keyCurrentIndex) ?? 0;

    _accounts.clear();
    if (raw != null && raw.isNotEmpty) {
      try {
        final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
        for (final j in list) {
          _accounts.add(Account.fromJson(j));
        }
      } catch (_) {}
    }

    // Если пусто -- добавляем дефолтный dev-аккаунт (Hardhat-0)
    if (_accounts.isEmpty) {
      _accounts.add(Account(
        name: 'Alice (dev)',
        address: '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
        privateKey: '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80',
        mnemonic: 'test test test test test test test test test test test junk',
      ));
      _accounts.add(Account(
        name: 'Bob (dev)',
        address: '0x70997970C51812dc3A010C7d01b50e0d17dc79C8',
        privateKey: '0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d',
        mnemonic: 'test test test test test test test test test test test junk',
      ));
      await _save();
    }

    _currentIndex = idx.clamp(0, _accounts.length - 1);
    _loaded = true;
    notifyListeners();
  }

  Future<void> switchTo(int index) async {
    if (index < 0 || index >= _accounts.length) return;
    _currentIndex = index;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setInt(_keyCurrentIndex, _currentIndex);
    notifyListeners();
  }

  Future<void> addAccount(Account a) async {
    _accounts.add(a);
    _currentIndex = _accounts.length - 1;
    await _save();
    notifyListeners();
  }

  Future<void> removeAccount(int index) async {
    if (_accounts.length <= 1) return;
    _accounts.removeAt(index);
    if (_currentIndex >= _accounts.length) _currentIndex = _accounts.length - 1;
    await _save();
    notifyListeners();
  }

  Future<void> _save() async {
    final prefs = await SharedPreferences.getInstance();
    final raw = jsonEncode(_accounts.map((a) => a.toJson()).toList());
    await prefs.setString(_keyAccounts, raw);
    await prefs.setInt(_keyCurrentIndex, _currentIndex);
  }
}
