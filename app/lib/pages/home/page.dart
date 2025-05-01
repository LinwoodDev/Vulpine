import 'package:flutter/material.dart';
import 'package:material_leap/widgets.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';
import 'package:vulpine/api/settings.dart';
import 'package:vulpine/cubits/settings.dart';
import 'package:vulpine/main.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
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
    );
  }
}
