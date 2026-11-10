import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../l10n/l10n_service.dart';

class HistoryList extends StatelessWidget {
  final List<Map<String, dynamic>> transactions;

  const HistoryList({super.key, required this.transactions});

  @override
  Widget build(BuildContext context) {
    final l10n = context.watch<L10nService>();

    if (transactions.isEmpty) {
      return Container(
        padding: const EdgeInsets.all(24),
        decoration: BoxDecoration(
          color: const Color(0xFF1A1A2D),
          borderRadius: BorderRadius.circular(16),
        ),
        child: Center(
          child: Text(
            l10n.t('no_transactions'),
            style: const TextStyle(color: Colors.white54, fontSize: 13),
          ),
        ),
      );
    }

    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A2D),
        borderRadius: BorderRadius.circular(16),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
            child: Text(
              l10n.t('history'),
              style: const TextStyle(
                color: Colors.white70,
                fontSize: 13,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          ...transactions.map((tx) => _buildRow(tx, l10n)).toList(),
        ],
      ),
    );
  }

  Widget _buildRow(Map<String, dynamic> tx, L10nService l10n) {
    final isOut = tx['direction'] == 'out';
    final valueAic = tx['valueAic'] as String? ?? '0';
    final blockNum = tx['blockNumber'] as int? ?? 0;
    final to = tx['to'] as String? ?? '';

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
      child: Row(
        children: [
          Container(
            width: 36,
            height: 36,
            decoration: BoxDecoration(
              color: isOut
                  ? const Color(0xFF3A1A1A)
                  : const Color(0xFF1A3A1A),
              borderRadius: BorderRadius.circular(10),
            ),
            child: Icon(
              isOut ? Icons.arrow_upward : Icons.arrow_downward,
              color: isOut ? Colors.redAccent : Colors.greenAccent,
              size: 18,
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  isOut ? l10n.t('sent') : l10n.t('received'),
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 13,
                    fontWeight: FontWeight.w500,
                  ),
                ),
                Text(
                  '#$blockNum  ${to.length > 14 ? to.substring(0, 10) + "..." : to}',
                  style: const TextStyle(
                    color: Colors.white38,
                    fontSize: 11,
                  ),
                ),
              ],
            ),
          ),
          Text(
            valueAic == '0'
                ? l10n.t('contract_call')
                : '${isOut ? "-" : "+"}$valueAic AIC',
            style: TextStyle(
              color: valueAic == '0'
                  ? Colors.white54
                  : (isOut ? Colors.redAccent : Colors.greenAccent),
              fontSize: 13,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }
}
