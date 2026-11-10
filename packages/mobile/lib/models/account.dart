class Account {
  final String name;        // "Alice"
  final String address;     // "0xf39f..."
  final String privateKey;  // "0xac09..."
  final String mnemonic;    // "test test test..."

  Account({
    required this.name,
    required this.address,
    required this.privateKey,
    required this.mnemonic,
  });

  Map<String, dynamic> toJson() => {
    'name': name,
    'address': address,
    'privateKey': privateKey,
    'mnemonic': mnemonic,
  };

  factory Account.fromJson(Map<String, dynamic> j) => Account(
    name: j['name'] as String,
    address: j['address'] as String,
    privateKey: j['privateKey'] as String,
    mnemonic: j['mnemonic'] as String,
  );

  String get shortAddress =>
      address.length > 10 ? '${address.substring(0, 6)}...${address.substring(address.length - 4)}' : address;
}
