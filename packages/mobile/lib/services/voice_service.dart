import 'dart:async';
import 'package:speech_to_text/speech_to_text.dart' as stt;

class VoiceCommand {
  final double? amount;
  final String? currency;
  final String? recipient;
  final String rawText;

  VoiceCommand({
    this.amount,
    this.currency,
    this.recipient,
    required this.rawText,
  });

  bool get isComplete => amount != null && recipient != null;

  @override
  String toString() => 'VoiceCommand(amount=$amount, currency=$currency, recipient=$recipient)';
}

class VoiceService {
  static final stt.SpeechToText _speech = stt.SpeechToText();
  static bool _initialized = false;

  static Future<bool> init() async {
    if (_initialized) return true;
    _initialized = await _speech.initialize(
      onStatus: (s) => print('[Voice] status: $s'),
      onError: (e) => print('[Voice] error: $e'),
    );
    if (_initialized) {
      final locales = await _speech.locales();
      print('[Voice] Available locales: ${locales.map((l) => l.localeId).toList()}');
    }
    return _initialized;
  }

  static bool get isListening => _speech.isListening;

  static Future<void> start({
    required void Function(String text, bool isFinal) onResult,
  }) async {
    if (!_initialized) await init();
    await _speech.listen(
      onResult: (result) => onResult(result.recognizedWords, result.finalResult),
      listenFor: const Duration(seconds: 10),
      pauseFor: const Duration(seconds: 3),
      localeId: 'ru_RU',
      listenOptions: stt.SpeechListenOptions(
        partialResults: true,
        cancelOnError: true,
      ),
    );
  }

  static Future<void> stop() async {
    await _speech.stop();
  }

  static VoiceCommand? parseCommand(String text) {
    final lower = text.toLowerCase().trim();
    if (lower.isEmpty) return null;

    final amountMatch = RegExp(r'(\d+[.,]?\d*)').firstMatch(lower);
    final amount = amountMatch != null
        ? double.tryParse(amountMatch.group(1)!.replaceAll(',', '.'))
        : null;

    String? currency;
    if (RegExp(r'\b(aic|эй\s*ай\s*си|эйайси|аик|айкос|aicos)\b').hasMatch(lower)) {
      currency = 'AIC';
    } else if (RegExp(r'\b(eth|эфир|ethereum)\b').hasMatch(lower)) {
      currency = 'ETH';
    } else if (RegExp(r'\b(usdt|юсдт|tether)\b').hasMatch(lower)) {
      currency = 'USDT';
    } else if (RegExp(r'\b(btc|биткоин|bitcoin)\b').hasMatch(lower)) {
      currency = 'BTC';
    }

    String? recipient;
    final atMatch = RegExp(r'@([a-z0-9_\-]+)').firstMatch(lower);
    if (atMatch != null) {
      recipient = '@' + atMatch.group(1)!;
    } else {
      final addrMatch = RegExp(r'0x[a-f0-9]{40}').firstMatch(lower);
      if (addrMatch != null) {
        recipient = addrMatch.group(0);
      } else {
        final toMatch = RegExp(r'(?:to|на|кому|отправь\s+\w+\s+)([a-zа-яё0-9_\-]+)')
            .firstMatch(lower);
        if (toMatch != null) {
          recipient = toMatch.group(1);
        }
      }
    }

    return VoiceCommand(
      amount: amount,
      currency: currency,
      recipient: recipient,
      rawText: text,
    );
  }
}
