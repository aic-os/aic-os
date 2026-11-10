import 'dart:async';
import 'package:flutter/foundation.dart';
import 'aic_rpc.dart';

/// Простое состояние кошелька (без приватных ключей -- для демо).
class WalletService extends ChangeNotifier {
  final AicRpc rpc;
  final String address;

  BigInt _balance = BigInt.zero;
  int _blockNumber = 0;
  int _chainId = 0;
  bool _loading = false;
  Timer? _pollTimer;
  List<Map<String, dynamic>> _transactions = [];
  String? _error;
  double _priceUsd = 10.0; // стартовая цена AIC = $10
  double _priceRub = 950.0; // ~950₽ за AIC

  WalletService({AicRpc? rpc, this.address = '0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266'})
      : rpc = rpc ?? AicRpc();

  BigInt get balance => _balance;
  int get blockNumber => _blockNumber;
  int get chainId => _chainId;
  bool get loading => _loading;
  String? get error => _error;
  double get priceUsd => _priceUsd;
  double get priceRub => _priceRub;

  String get balanceAic => AicRpc.weiToAic(_balance);
  List<Map<String, dynamic>> get transactions => _transactions;
  double get balanceUsd => (double.parse(balanceAic) * _priceUsd);
  double get balanceRub => (double.parse(balanceAic) * _priceRub);

  void startAutoRefresh({Duration interval = const Duration(seconds: 3)}) {
    _pollTimer?.cancel();
    _pollTimer = Timer.periodic(interval, (_) {
      refresh();
    });
  }

  @override
  void dispose() {
    _pollTimer?.cancel();
    super.dispose();
  }

  Future<void> refresh() async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _chainId = await rpc.chainId();
      _blockNumber = await rpc.blockNumber();
      _balance = await rpc.getBalance(address);
      _transactions = await rpc.getAccountTransactions(address, limit: 20);
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }
}


