import 'package:flutter/material.dart';
import 'package:material_leap/widgets.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';
import 'package:vulpine/api/settings.dart';
import 'package:vulpine/cubits/settings.dart';
import 'package:vulpine/main.dart';
import 'package:vulpine/widgets/carousel.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final CarouselController _carouselController = CarouselController();

  @override
  void dispose() {
    super.dispose();
    _carouselController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: WindowTitleBar<SettingsCubit, VulpineSettings>(
        title: Text(applicationName),
        actions: [
          IconButton(
            icon: const Icon(PhosphorIconsLight.gear),
            onPressed: () => openSettings(context),
          ),
        ],
      ),
      body: ListView(
        children: [
          VulpineCarouselView(
            children: List.generate(
              10,
              (index) =>
                  UncontainedLayoutCard(index: index, label: 'Item $index'),
            ),
          ),
        ],
      ),
    );
  }
}

class UncontainedLayoutCard extends StatelessWidget {
  const UncontainedLayoutCard({
    super.key,
    required this.index,
    required this.label,
  });

  final int index;
  final String label;

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: Colors.primaries[index % Colors.primaries.length].withValues(
        alpha: 0.5,
      ),
      child: Center(
        child: Text(
          label,
          style: const TextStyle(color: Colors.white, fontSize: 20),
          overflow: TextOverflow.clip,
          softWrap: false,
        ),
      ),
    );
  }
}
