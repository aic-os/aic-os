import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Настройки приложения: RPC URL, тема, язык.
/// Persist в SharedPreferences.
class SettingsService extends ChangeNotifier {
  static const _keyRpcUrl = 'aic_rpc_url';
  static const _keyTheme = 'aic_theme';
  static const _defaultRpcWeb = 'http://127.0.0.1:8545';
  static const _defaultRpcMobile = 'http://192.168.1.100:8545'; // замени на IP компа

  String _rpcUrl = _defaultRpcWeb;
  String _theme = 'dark';
  bool _loaded = false;

  String get rpcUrl => _rpcUrl;
  String get theme => _theme;
  bool get loaded => _loaded;

  Future<void> load() async {
    final prefs = await SharedPreferences.getInstance();
    final saved = prefs.getString(_keyRpcUrl);
    // На mobile -- дефолтный не 127.0.0.1, а локальный IP
    final defaultUrl = kIsWeb ? _defaultRpcWeb : _defaultRpcMobile;
    _rpcUrl = saved ?? defaultUrl;
    _theme = prefs.getString(_keyTheme) ?? 'dark';
    _loaded = true;
    notifyListeners();
  }

  Future<void> setRpcUrl(String url) async {
    _rpcUrl = url.trim();
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_keyRpcUrl, _rpcUrl);
    notifyListeners();
  }

  Future<void> setTheme(String theme) async {
    _theme = theme;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_keyTheme, _theme);
    notifyListeners();
  }

  Future<void> resetRpcUrl() async {
    final defaultUrl = kIsWeb ? _defaultRpcWeb : _defaultRpcMobile;
    await setRpcUrl(defaultUrl);
  }
}
