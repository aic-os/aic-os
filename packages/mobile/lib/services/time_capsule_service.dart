import 'aic_rpc.dart';

class TimeCapsule {
  final int id;
  final String sender;
  final String recipient;
  final DateTime unlockAt;
  final String balanceHex;
  final bool claimed;
  final bool cancelled;

  TimeCapsule({
    required this.id,
    required this.sender,
    required this.recipient,
    required this.unlockAt,
    required this.balanceHex,
    required this.claimed,
    required this.cancelled,
  });

  double get balanceAic {
    final clean = balanceHex.replaceFirst('0x', '');
    if (clean.isEmpty) return 0;
    final n = BigInt.parse(clean, radix: 16);
    return (n / BigInt.from(10).pow(18)).toDouble();
  }

  bool get isUnlocked => DateTime.now().isAfter(unlockAt);
  bool get isActive => !claimed && !cancelled;

  factory TimeCapsule.fromJson(Map<String, dynamic> j) {
    final unlockTs = j['unlockAt'] as int? ?? 0;
    return TimeCapsule(
      id: j['id'] as int? ?? 0,
      sender: j['sender'] as String? ?? '',
      recipient: j['recipient'] as String? ?? '',
      unlockAt: DateTime.fromMillisecondsSinceEpoch(unlockTs * 1000),
      balanceHex: j['balance'] as String? ?? '0x0',
      claimed: j['claimed'] as bool? ?? false,
      cancelled: j['cancelled'] as bool? ?? false,
    );
  }
}

class TimeCapsuleService {
  final AicRpc rpc;
  TimeCapsuleService({required this.rpc});

  Future<List<TimeCapsule>> listForAddress(String address) async {
    final raw = await rpc.timeCapsuleList(address);
    return raw.map(TimeCapsule.fromJson).toList();
  }
}
