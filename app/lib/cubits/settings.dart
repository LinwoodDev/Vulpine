import 'package:dart_mappable/dart_mappable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:material_leap/material_leap.dart';
import 'package:shared_preferences/shared_preferences.dart';

part 'settings.mapper.dart';

@MappableEnum()
enum ThemeDensity {
  system,
  maximize,
  desktop,
  compact,
  comfortable,
  standard;

  VisualDensity toFlutter() => switch (this) {
    ThemeDensity.maximize => const VisualDensity(horizontal: -4, vertical: -4),
    ThemeDensity.desktop => const VisualDensity(horizontal: -3, vertical: -3),
    ThemeDensity.compact => VisualDensity.compact,
    ThemeDensity.comfortable => VisualDensity.comfortable,
    ThemeDensity.standard => VisualDensity.standard,
    ThemeDensity.system => VisualDensity.adaptivePlatformDensity,
  };
}

final class ThemeModeMapper extends SimpleMapper<ThemeMode> {
  const ThemeModeMapper();

  @override
  ThemeMode decode(Object value) {
    return ThemeMode.values.byName(value.toString());
  }

  @override
  String encode(ThemeMode value) {
    return value.name;
  }
}

@MappableClass(includeCustomMappers: [ThemeModeMapper()])
class VulpineSettings with VulpineSettingsMappable, LeapSettings {
  final String locale;
  final ThemeMode themeMode;
  @override
  final bool nativeTitleBar;
  final String design;
  final ThemeDensity density;
  final bool highContrast;

  const VulpineSettings({
    this.locale = '',
    this.themeMode = ThemeMode.system,
    this.nativeTitleBar = false,
    this.design = '',
    this.density = ThemeDensity.system,
    this.highContrast = false,
  });

  factory VulpineSettings.fromPrefs(SharedPreferences prefs) => VulpineSettings(
    themeMode: ThemeMode.values.byName(
      prefs.getString('themeMode') ?? 'system',
    ),
    design: prefs.getString('design') ?? '',
    nativeTitleBar: prefs.getBool('nativeTitleBar') ?? false,
    locale: prefs.getString('locale') ?? '',
    density: ThemeDensity.values.byName(prefs.getString('density') ?? 'system'),
    highContrast: prefs.getBool('highContrast') ?? false,
  );

  Future<void> save() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('themeMode', themeMode.name);
    await prefs.setString('design', design);
    await prefs.setBool('nativeTitleBar', nativeTitleBar);
    await prefs.setString('locale', locale);
    await prefs.setString('density', density.name);
    await prefs.setBool('highContrast', highContrast);
  }
}

class SettingsCubit extends Cubit<VulpineSettings>
    with LeapSettingsBlocBaseMixin<VulpineSettings> {
  SettingsCubit(SharedPreferences prefs)
    : super(VulpineSettings.fromPrefs(prefs));

  Future<void> changeThemeMode(ThemeMode mode) {
    emit(state.copyWith(themeMode: mode));
    return state.save();
  }

  Future<void> changeDesign(String design) {
    emit(state.copyWith(design: design));
    return state.save();
  }

  Future<void> changeNativeTitleBar(bool nativeTitleBar) {
    emit(state.copyWith(nativeTitleBar: nativeTitleBar));
    return state.save();
  }

  Future<void> changeLocale(String locale) {
    emit(state.copyWith(locale: locale));
    return state.save();
  }

  Future<void> changeDensity(ThemeDensity density) {
    emit(state.copyWith(density: density));
    return state.save();
  }

  Future<void> changeHighContrast(bool highContrast) {
    emit(state.copyWith(highContrast: highContrast));
    return state.save();
  }

  Future<void> importSettings(String data) {
    final settings = VulpineSettingsMapper.fromJson(data).copyWith();
    emit(settings);
    return state.save();
  }

  Future<String> exportSettings() async {
    return state.toJson();
  }
}
