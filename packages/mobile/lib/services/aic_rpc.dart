import 'dart:convert';
import 'package:http/http.dart' as http;

/// RPC-клиент для AIC Chain.
/// Все методы -- через JSON-RPC 2.0 на /.
class AicRpc {
  final String rpcUrl;
  final http.Client _client = http.Client();
  int _idCounter = 1;

  AicRpc({this.rpcUrl = 'http://127.0.0.1:8545'});

  Future<dynamic> _call(String method, List<dynamic> params) async {
    final id = _idCounter++;
    final body = jsonEncode({
      'jsonrpc': '2.0',
      'id': id,
      'method': method,
      'params': params,
    });

    final resp = await _client.post(
      Uri.parse(rpcUrl),
      headers: {'Content-Type': 'application/json'},
      body: body,
    );

    if (resp.statusCode != 200) {
      throw Exception('RPC HTTP ${resp.statusCode}: ${resp.body}');
    }

    final json = jsonDecode(resp.body) as Map<String, dynamic>;

    if (json.containsKey('error')) {
      throw Exception('RPC error: ${json['error']}');
    }

    return json['result'];
  }

  /// Высота цепочки (hex -> int).
  Future<int> blockNumber() async {
    final result = await _call('aic_blockNumber', []) as String;
    return int.parse(result.replaceFirst('0x', ''), radix: 16);
  }

  /// Chain ID (2026 для AIC).
  Future<int> chainId() async {
    return await _call('aic_chainId', []) as int;
  }

  /// Баланс в wei (hex -> BigInt).
  Future<BigInt> getBalance(String address) async {
    final result = await _call('aic_getBalance', [address]) as String;
    final hex = result.replaceFirst('0x', '');
    if (hex.isEmpty) return BigInt.zero;
    return BigInt.parse(hex, radix: 16);
  }

  /// Nonce аккаунта.
  Future<int> getNonce(String address) async {
    final result = await _call('aic_getAccount', [address]) as Map<String, dynamic>;
    return result['nonce'] as int;
  }

  /// Резолв @name -> address (null если не зарегистрирован).
  Future<String?> resolveName(String name) async {
    // selector("resolve(string)") = 0x461a4478
    final data = _encodeResolveCall(name);
    final result = await _call('aic_call', [
      '0x0000000000000000000000000000000000a1c1d0',
      data,
    ]) as String;

    final hex = result.replaceFirst('0x', '');
    if (hex.length < 64) return null;
    final addrHex = hex.substring(24);
    if (addrHex == '0' * 40) return null;
    return '0x$addrHex';
  }

  /// ABI-encode для resolve(string).
  String _encodeResolveCall(String name) {
    final nameBytes = utf8.encode(name);
    final len = nameBytes.length;
    final padded = ((len + 31) ~/ 32) * 32;

    final buf = StringBuffer('0x461a4478');
    buf.write('0000000000000000000000000000000000000000000000000000000000000020');
    buf.write(len.toRadixString(16).padLeft(64, '0'));
    for (final b in nameBytes) {
      buf.write(b.toRadixString(16).padLeft(2, '0'));
    }
    for (var i = len; i < padded; i++) {
      buf.write('00');
    }
    return buf.toString();
  }

  /// Форматировать wei -> AIC (18 decimals).
  Future<String> sendRawTransaction(String rawHex, [String? pqPk, String? pqSig]) async {
    final params = <dynamic>[rawHex];
    if (pqPk != null) params.add(pqPk); else params.add(null);
    if (pqSig != null) params.add(pqSig); else params.add(null);
    return await _call('aic_sendRawTransaction', params) as String;
  }

  Future<List<Map<String, dynamic>>> getAccountTransactions(String address, {int limit = 20}) async {
    final result = await _call('aic_getAccountTransactions', [address, limit]) as List<dynamic>;
    return result.map((e) => Map<String, dynamic>.from(e as Map)).toList();
  }
  Future<Map<String, dynamic>> quantumBountyInfo() async {
    final result = await _call('aic_quantumBountyInfo', []) as Map<String, dynamic>;
    return result;
  }

  /// Конвертирует hex U256 строку в человекочитаемое число AIC.
  static String hexToAic(String hex) {
    final clean = hex.replaceFirst('0x', '');
    if (clean.isEmpty) return '0';
    final n = BigInt.parse(clean, radix: 16);
    return weiToAic(n);
  }
  Future<List<Map<String, dynamic>>> timeCapsuleList(String address) async {
    final result = await _call('aic_timeCapsuleList', [address]) as List<dynamic>;
    return result.map((e) => Map<String, dynamic>.from(e as Map)).toList();
  }

  Future<int> timeCapsuleNextId() async {
    final result = await _call('aic_timeCapsuleNextId', []) as String;
    final clean = result.replaceFirst('0x', '');
    if (clean.isEmpty) return 0;
    return int.parse(clean, radix: 16);
  }
  static String weiToAic(BigInt wei, {int decimals = 18, int precision = 4}) {
    final divisor = BigInt.from(10).pow(decimals);
    final whole = wei ~/ divisor;
    final frac = wei % divisor;
    if (frac == BigInt.zero) return whole.toString();
    final fracStr = frac.toString().padLeft(decimals, '0');
    final trimmed = fracStr.substring(0, precision).replaceAll(RegExp(r'0+$'), '');
    if (trimmed.isEmpty) return whole.toString();
    return '$whole.$trimmed';
  }
}




