import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:vulpine/src/generated/i18n/app_localizations.dart';
import 'package:material_leap/material_leap.dart';

import '../../cubits/settings.dart';

class DataSettingsPage extends StatelessWidget {
  final bool inView;
  const DataSettingsPage({super.key, this.inView = false});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: inView ? Colors.transparent : null,
      appBar: WindowTitleBar<SettingsCubit, VulpineSettings>(
        inView: inView,
        backgroundColor: inView ? Colors.transparent : null,
        title: Text(AppLocalizations.of(context).data),
      ),
      body: BlocBuilder<SettingsCubit, VulpineSettings>(
        builder: (context, state) => ListView(children: [
              ],
            ),
      ),
    );
  }
}
