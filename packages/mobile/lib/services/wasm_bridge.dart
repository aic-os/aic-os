import 'dart:js_interop';
import 'dart:js_interop_unsafe';
import 'dart:async';
import 'dart:convert';

/// Мост между Dart и WASM. Все крипто-операции (подпись, работа с ключами) -- в WASM.
/// Ключи НЕ покидают браузер.
class WasmBridge {
  static bool _ready = false;

  /// Дождаться загрузки WASM (index.html грузит его асинхронно).
  static Future<void> ensureReady() async {
    if (_ready) return;
    final completer = Completer<void>();

    // Проверяем, не загружен ли уже
    final global = globalContext;
    final wasm = global['_aicWasm'];
    if (wasm != null && !wasm.isUndefinedOrNull) {
      _ready = true;
      completer.complete();
      return;
    }

    // Слушаем событие от index.html
    final listener = (JSAny? event) {
      if (!completer.isCompleted) {
        _ready = true;
        completer.complete();
      }
    }.toJS;

    globalContext.callMethod(
      'addEventListener'.toJS,
      'aic-wasm-ready'.toJS,
      listener,
    );

    // Таймаут через 10 сек
    Future.delayed(const Duration(seconds: 10)).then((_) {
      if (!completer.isCompleted) {
        completer.completeError(Exception('WASM не загрузился за 10 сек'));
      }
    });

    return completer.future;
  }

  static JSObject get _aic {
    final w = globalContext['_aicWasm'];
    if (w == null || w.isUndefinedOrNull) {
      throw Exception('WASM не инициализирован. Перезагрузи страницу.');
    }
    return w as JSObject;
  }

  /// Сгенерировать 12-словную мнемонику.
  static Future<String> generateMnemonic12() async {
    await ensureReady();
    final result = _aic.callMethod('generate_mnemonic_12'.toJS);
    return (result as JSString).toDart;
  }

  /// Восстановить кошелёк из мнемоники, получить адрес.
  static Future<Map<String, dynamic>> walletFromMnemonic(
    String mnemonic,
    int index,
  ) async {
    await ensureReady();
    final result = _aic.callMethod(
      'wallet_from_mnemonic'.toJS,
      mnemonic.toJS,
      index.toJS,
    );
    return jsonDecode(result.dartify() is String
        ? result.dartify() as String
        : jsonEncode(result.dartify()));
  }

  /// Адрес из приватного ключа.
  static Future<String> addressFromPrivkey(String privkeyHex) async {
    await ensureReady();
    final result = _aic.callMethod(
      'address_from_privkey'.toJS,
      privkeyHex.toJS,
    );
    return (result as JSString).toDart;
  }

  /// ABI-encode для register(string,address) в AICID.
  static Future<String> encodeAicidRegister(String name, String owner) async {
    await ensureReady();
    final result = _aic.callMethod(
      'encode_aicid_register'.toJS,
      name.toJS,
      owner.toJS,
    );
    return (result as JSString).toDart;
  }

  /// ABI-encode для resolve(string).
  static Future<String> encodeResolveCall(String name) async {
    await ensureReady();
    final result = _aic.callMethod(
      'encode_resolve_call'.toJS,
      name.toJS,
    );
    return (result as JSString).toDart;
  }

  /// Подписать Legacy EIP-155 транзакцию.
  /// Возвращает { from, raw_hex, tx_hash }.
  static Future<Map<String, dynamic>> signLegacyTx({
    required String privkeyHex,
    required String toHex,
    required String valueHex,
    required int nonce,
    required String gasPriceHex,
    required int gasLimit,
    required String dataHex,
    required int chainId,
  }) async {
    await ensureReady();
    final result = _aic.callMethod(
      'sign_legacy_tx'.toJS,
      privkeyHex.toJS,
      toHex.toJS,
      valueHex.toJS,
      nonce.toJS,
      gasPriceHex.toJS,
      gasLimit.toJS,
      dataHex.toJS,
      chainId.toJS,
    );
    final json = result.dartify();
    if (json is Map) {
      return Map<String, dynamic>.from(json);
    }
    // serde-wasm-bindgen может вернуть объект -- конвертируем через JSON
    return Map<String, dynamic>.from(
      jsonDecode(jsonEncode(json)) as Map,
    );
  }

  /// Версия WASM (для диагностики).
  static Future<String> version() async {
    await ensureReady();
    final result = _aic.callMethod('version'.toJS);
    return (result as JSString).toDart;
  }
}
