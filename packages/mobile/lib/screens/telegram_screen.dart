import 'package:flutter/material.dart';
import 'chat_detail_screen.dart';

class TelegramScreen extends StatelessWidget {
  const TelegramScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0A0F),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0A0A0F),
        title: const Text('Chats'),
        actions: [
          IconButton(icon: const Icon(Icons.search, color: Colors.white70), onPressed: () {}),
          IconButton(icon: const Icon(Icons.edit, color: Colors.white70), onPressed: () {}),
        ],
      ),
      body: ListView(
        children: [
          _chatTile(
            context,
            name: 'Alice',
            lastMessage: 'Отправила 100 AIC ✓✓',
            time: '12:34',
            unread: 0,
            online: true,
            color: const Color(0xFF4A9EFF),
          ),
          _chatTile(
            context,
            name: 'Bob',
            lastMessage: 'Спасибо, получил!',
            time: '11:20',
            unread: 2,
            online: false,
            color: const Color(0xFF9C6AFF),
          ),
          _chatTile(
            context,
            name: 'AIC News',
            lastMessage: '🎁 Quantum Bounty: 300 000 AIC',
            time: '09:15',
            unread: 5,
            online: false,
            color: Colors.amber,
            isChannel: true,
          ),
          _chatTile(
            context,
            name: 'Crypto Chat RU',
            lastMessage: 'Кто-нибудь пробовал VPN за 1 AIC?',
            time: 'Вчера',
            unread: 0,
            online: false,
            color: const Color(0xFF4CAF50),
            isGroup: true,
          ),
        ],
      ),
    );
  }

  Widget _chatTile(
    BuildContext context, {
    required String name,
    required String lastMessage,
    required String time,
    required int unread,
    required bool online,
    required Color color,
    bool isChannel = false,
    bool isGroup = false,
  }) {
    return ListTile(
      onTap: () => Navigator.push(
        context,
        MaterialPageRoute(
          builder: (_) => ChatDetailScreen(name: name, color: color),
        ),
      ),
      leading: Stack(
        children: [
          Container(
            width: 52,
            height: 52,
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [color, color.withOpacity(0.6)],
              ),
              borderRadius: BorderRadius.circular(16),
            ),
            child: Center(
              child: isChannel
                  ? const Icon(Icons.campaign, color: Colors.white)
                  : Text(
                      name[0].toUpperCase(),
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
            ),
          ),
          if (online)
            Positioned(
              right: 0,
              bottom: 0,
              child: Container(
                width: 14,
                height: 14,
                decoration: BoxDecoration(
                  color: Colors.greenAccent,
                  shape: BoxShape.circle,
                  border: Border.all(color: const Color(0xFF0A0A0F), width: 2),
                ),
              ),
            ),
        ],
      ),
      title: Row(
        children: [
          Text(
            name,
            style: const TextStyle(
              color: Colors.white,
              fontSize: 15,
              fontWeight: FontWeight.w600,
            ),
          ),
          if (isChannel) ...[
            const SizedBox(width: 4),
            const Icon(Icons.verified, color: Color(0xFF4A9EFF), size: 14),
          ],
          if (isGroup) ...[
            const SizedBox(width: 4),
            const Icon(Icons.group, color: Colors.white38, size: 14),
          ],
        ],
      ),
      subtitle: Text(
        lastMessage,
        style: TextStyle(
          color: unread > 0 ? Colors.white70 : Colors.white38,
          fontSize: 12,
        ),
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
      ),
      trailing: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Text(
            time,
            style: const TextStyle(color: Colors.white38, fontSize: 11),
          ),
          const SizedBox(height: 4),
          if (unread > 0)
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 2),
              decoration: BoxDecoration(
                color: const Color(0xFF4A9EFF),
                borderRadius: BorderRadius.circular(10),
              ),
              child: Text(
                unread.toString(),
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 10,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
        ],
      ),
    );
  }
}

