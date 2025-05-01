import 'package:flutter/services.dart';

import 'cubits/settings.dart';
import 'setup.dart' as general_setup;

Future<void> setup(SettingsCubit settingsCubit) async {
  await BrowserContextMenu.disableContextMenu();
  await general_setup.setup(settingsCubit);
}
