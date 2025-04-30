import 'package:dynamic_color/dynamic_color.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_localized_locales/flutter_localized_locales.dart';
import 'package:go_router/go_router.dart';
import 'package:material_leap/l10n/leap_localizations.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:vulpine/cubits/settings.dart';
import 'package:vulpine/pages/home/page.dart';
import 'package:vulpine/theme.dart';
import 'package:window_manager/window_manager.dart';

void main() {
  runApp(const MyApp());
}

final _router = GoRouter(
  routes: [GoRoute(path: '/', builder: (context, state) => HomePage())],
);

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    if (kIsWeb) return _buildApp(null, null);
    return DynamicColorBuilder(
      builder:
          (lightDynamic, darkDynamic) => _buildApp(lightDynamic, darkDynamic),
    );
  }

  Widget _buildApp(ColorScheme? lightDynamic, ColorScheme? darkDynamic) {
    final virtualWindowFrameBuilder = VirtualWindowFrameInit();
    return BlocBuilder<SettingsCubit, VulpineSettings>(
      buildWhen:
          (previous, current) =>
              previous.design != current.design ||
              previous.themeMode != current.themeMode ||
              previous.locale != current.locale ||
              previous.density != current.density ||
              previous.highContrast != current.highContrast,
      builder:
          (context, state) => MaterialApp.router(
            debugShowCheckedModeBanner: false,
            routerConfig: _router,
            title: applicationName,
            theme: getThemeData(
              state.design,
              false,
              state.density.toFlutter(),
              lightDynamic,
              state.highContrast,
            ),
            darkTheme: getThemeData(
              state.design,
              true,
              state.density.toFlutter(),
              darkDynamic,
              state.highContrast,
            ),
            themeMode: state.themeMode,
            locale: state.locale.isEmpty ? null : Locale(state.locale),
            localizationsDelegates: const [
              LocaleNamesLocalizationsDelegate(),
              LeapLocalizations.delegate,
            ],
            builder: (context, child) {
              if (!state.nativeTitleBar) {
                child = virtualWindowFrameBuilder(context, child);
              }
              return child ?? Container();
            },
          ),
    );
  }
}

const flavor = String.fromEnvironment('flavor');
const isNightly =
    flavor == 'nightly' || flavor == 'dev' || flavor == 'development';
const shortApplicationName = isNightly ? 'Flow Nightly' : 'Flow';
const applicationMinorVersion = "0.4.3";
const applicationName = 'Linwood $shortApplicationName';

Future<String> getCurrentVersion() async {
  const envVersion = String.fromEnvironment('version');
  if (envVersion.isNotEmpty) return envVersion;
  final info = await PackageInfo.fromPlatform();
  return info.version;
}
