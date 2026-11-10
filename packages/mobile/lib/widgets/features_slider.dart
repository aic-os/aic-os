import 'package:flutter/material.dart';

class FeatureItem {
  final String id;
  final String title;
  final String subtitle;
  final IconData icon;
  final Color color;

  const FeatureItem({
    required this.id,
    required this.title,
    required this.subtitle,
    required this.icon,
    required this.color,
  });
}

class FeaturesSlider extends StatelessWidget {
  final List<FeatureItem> features;
  final Function(FeatureItem)? onTap;

  const FeaturesSlider({super.key, required this.features, this.onTap});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 120,
      child: ListView.separated(
        scrollDirection: Axis.horizontal,
        padding: const EdgeInsets.symmetric(horizontal: 20),
        itemCount: features.length,
        separatorBuilder: (_, __) => const SizedBox(width: 12),
        itemBuilder: (context, i) {
          final f = features[i];
          return GestureDetector(
            onTap: () => onTap?.call(f),
            child: Container(
              width: 160,
              padding: const EdgeInsets.all(14),
              decoration: BoxDecoration(
                color: const Color(0xFF1A1A2D),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: f.color.withOpacity(0.3)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Icon(f.icon, color: f.color, size: 24),
                  const Spacer(),
                  Text(f.title,
                      style: const TextStyle(
                        color: Colors.white, fontSize: 13, fontWeight: FontWeight.bold,
                      ),
                      maxLines: 1, overflow: TextOverflow.ellipsis),
                  const SizedBox(height: 2),
                  Text(f.subtitle,
                      style: const TextStyle(color: Colors.white54, fontSize: 10),
                      maxLines: 2, overflow: TextOverflow.ellipsis),
                ],
              ),
            ),
          );
        },
      ),
    );
  }
}
