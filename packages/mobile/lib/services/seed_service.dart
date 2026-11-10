import 'dart:convert';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'wasm_bridge.dart';

/// Хранит сид-фразу в защищённом хранилище (Keychain iOS / Keystore Android).
/// На web — IndexedDB с шифрованием.
class SeedService {
  static const _storage = FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
    iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock),
    webOptions: WebOptions(dbName: 'aic_wallet', publicKey: 'aic_seed_v1'),
  );
  static const _keyMnemonic = 'aic_seed_mnemonic_v1';
  static const _keyAddress = 'aic_seed_address_v1';
  static const _keyPrivkey = 'aic_seed_privkey_v1';
  static const _keyHasWallet = 'aic_has_wallet';

  /// Есть ли уже кошелёк (быстрая проверка через SharedPreferences).
  static Future<bool> hasWallet() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getBool(_keyHasWallet) ?? false;
  }

  /// Создать НОВЫЙ кошелёк (генерирует 12 слов через WASM).
  /// Возвращает (mnemonic, address, privkey).
  static Future<Map<String, String>> createNew() async {
    final mnemonic = await WasmBridge.generateMnemonic12();
    final wallet = await WasmBridge.walletFromMnemonic(mnemonic, 0);
    final address = wallet['address'] as String;
    final privkey = wallet['private_key_hex'] as String;

    await _storage.write(key: _keyMnemonic, value: mnemonic);
    await _storage.write(key: _keyAddress, value: address);
    await _storage.write(key: _keyPrivkey, value: privkey);

    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_keyHasWallet, true);

    return {
      'mnemonic': mnemonic,
      'address': address,
      'private_key': privkey,
    };
  }

  /// Импортировать существующий кошелёк из сид-фразы.
  static Future<Map<String, String>> importFromMnemonic(String mnemonic) async {
    final wallet = await WasmBridge.walletFromMnemonic(mnemonic, 0);
    final address = wallet['address'] as String;
    final privkey = wallet['private_key_hex'] as String;

    await _storage.write(key: _keyMnemonic, value: mnemonic);
    await _storage.write(key: _keyAddress, value: address);
    await _storage.write(key: _keyPrivkey, value: privkey);

    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_keyHasWallet, true);

    return {
      'mnemonic': mnemonic,
      'address': address,
      'private_key': privkey,
    };
  }

  static Future<String?> getAddress() => _storage.read(key: _keyAddress);
  static Future<String?> getPrivkey() => _storage.read(key: _keyPrivkey);
  static Future<String?> getMnemonic() => _storage.read(key: _keyMnemonic);

  /// Полный сброс (danger zone).
  static Future<void> reset() async {
    await _storage.deleteAll();
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_keyHasWallet);
  }
}

