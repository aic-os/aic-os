import 'dart:async';
import 'package:flutter/foundation.dart';

import 'aic_rpc.dart';

class WalletService extends ChangeNotifier {
  AicRpc rpc;
  String address;

  BigInt _balance = BigInt.zero;
  int _blockNumber = 0;
  int _chainId = 0;
  bool _loading = false;
  Timer? _pollTimer;
  String? _error;
  double _priceUsd = 10.0;
  double _priceRub = 950.0;

  WalletService({
    AicRpc? rpc,
    this.address = '0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266',
  }) : rpc = rpc ?? AicRpc();

  BigInt get balance => _balance;
  int get blockNumber => _blockNumber;
  int get chainId => _chainId;
  bool get loading => _loading;
  String? get error => _error;
  double get priceUsd => _priceUsd;
  double get priceRub => _priceRub;

  String get balanceAic => AicRpc.weiToAic(_balance);
  double get balanceUsd => (double.tryParse(balanceAic) ?? 0) * _priceUsd;
  double get balanceRub => (double.tryParse(balanceAic) ?? 0) * _priceRub;

  void updateRpcUrl(String url) {
    if (rpc.rpcUrl == url) return;
    rpc = AicRpc(rpcUrl: url);
    refresh();
  }

  void updateAddress(String newAddress) {
    if (address == newAddress) return;
    address = newAddress;
    refresh();
  }

  void startAutoRefresh({Duration interval = const Duration(seconds: 3)}) {
    _pollTimer?.cancel();
    _pollTimer = Timer.periodic(interval, (_) => refresh());
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
    } catch (e) {
      _error = e.toString();
    } finally {
      _loading = false;
      notifyListeners();
    }
  }
}
