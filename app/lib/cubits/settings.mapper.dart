// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, unnecessary_cast, override_on_non_overriding_member
// ignore_for_file: strict_raw_type, inference_failure_on_untyped_parameter

part of 'settings.dart';

class ThemeDensityMapper extends EnumMapper<ThemeDensity> {
  ThemeDensityMapper._();

  static ThemeDensityMapper? _instance;
  static ThemeDensityMapper ensureInitialized() {
    if (_instance == null) {
      MapperContainer.globals.use(_instance = ThemeDensityMapper._());
    }
    return _instance!;
  }

  static ThemeDensity fromValue(dynamic value) {
    ensureInitialized();
    return MapperContainer.globals.fromValue(value);
  }

  @override
  ThemeDensity decode(dynamic value) {
    switch (value) {
      case r'system':
        return ThemeDensity.system;
      case r'maximize':
        return ThemeDensity.maximize;
      case r'desktop':
        return ThemeDensity.desktop;
      case r'compact':
        return ThemeDensity.compact;
      case r'comfortable':
        return ThemeDensity.comfortable;
      case r'standard':
        return ThemeDensity.standard;
      default:
        throw MapperException.unknownEnumValue(value);
    }
  }

  @override
  dynamic encode(ThemeDensity self) {
    switch (self) {
      case ThemeDensity.system:
        return r'system';
      case ThemeDensity.maximize:
        return r'maximize';
      case ThemeDensity.desktop:
        return r'desktop';
      case ThemeDensity.compact:
        return r'compact';
      case ThemeDensity.comfortable:
        return r'comfortable';
      case ThemeDensity.standard:
        return r'standard';
    }
  }
}

extension ThemeDensityMapperExtension on ThemeDensity {
  String toValue() {
    ThemeDensityMapper.ensureInitialized();
    return MapperContainer.globals.toValue<ThemeDensity>(this) as String;
  }
}

class VulpineSettingsMapper extends ClassMapperBase<VulpineSettings> {
  VulpineSettingsMapper._();

  static VulpineSettingsMapper? _instance;
  static VulpineSettingsMapper ensureInitialized() {
    if (_instance == null) {
      MapperContainer.globals.use(_instance = VulpineSettingsMapper._());
      MapperContainer.globals.useAll([ThemeModeMapper()]);
      ThemeDensityMapper.ensureInitialized();
    }
    return _instance!;
  }

  @override
  final String id = 'VulpineSettings';

  static String _$locale(VulpineSettings v) => v.locale;
  static const Field<VulpineSettings, String> _f$locale = Field(
    'locale',
    _$locale,
    opt: true,
    def: '',
  );
  static ThemeMode _$themeMode(VulpineSettings v) => v.themeMode;
  static const Field<VulpineSettings, ThemeMode> _f$themeMode = Field(
    'themeMode',
    _$themeMode,
    opt: true,
    def: ThemeMode.system,
  );
  static bool _$nativeTitleBar(VulpineSettings v) => v.nativeTitleBar;
  static const Field<VulpineSettings, bool> _f$nativeTitleBar = Field(
    'nativeTitleBar',
    _$nativeTitleBar,
    opt: true,
    def: false,
  );
  static String _$design(VulpineSettings v) => v.design;
  static const Field<VulpineSettings, String> _f$design = Field(
    'design',
    _$design,
    opt: true,
    def: '',
  );
  static ThemeDensity _$density(VulpineSettings v) => v.density;
  static const Field<VulpineSettings, ThemeDensity> _f$density = Field(
    'density',
    _$density,
    opt: true,
    def: ThemeDensity.system,
  );
  static bool _$highContrast(VulpineSettings v) => v.highContrast;
  static const Field<VulpineSettings, bool> _f$highContrast = Field(
    'highContrast',
    _$highContrast,
    opt: true,
    def: false,
  );

  @override
  final MappableFields<VulpineSettings> fields = const {
    #locale: _f$locale,
    #themeMode: _f$themeMode,
    #nativeTitleBar: _f$nativeTitleBar,
    #design: _f$design,
    #density: _f$density,
    #highContrast: _f$highContrast,
  };

  static VulpineSettings _instantiate(DecodingData data) {
    return VulpineSettings(
      locale: data.dec(_f$locale),
      themeMode: data.dec(_f$themeMode),
      nativeTitleBar: data.dec(_f$nativeTitleBar),
      design: data.dec(_f$design),
      density: data.dec(_f$density),
      highContrast: data.dec(_f$highContrast),
    );
  }

  @override
  final Function instantiate = _instantiate;

  static VulpineSettings fromMap(Map<String, dynamic> map) {
    return ensureInitialized().decodeMap<VulpineSettings>(map);
  }

  static VulpineSettings fromJson(String json) {
    return ensureInitialized().decodeJson<VulpineSettings>(json);
  }
}

mixin VulpineSettingsMappable {
  String toJson() {
    return VulpineSettingsMapper.ensureInitialized()
        .encodeJson<VulpineSettings>(this as VulpineSettings);
  }

  Map<String, dynamic> toMap() {
    return VulpineSettingsMapper.ensureInitialized().encodeMap<VulpineSettings>(
      this as VulpineSettings,
    );
  }

  VulpineSettingsCopyWith<VulpineSettings, VulpineSettings, VulpineSettings>
  get copyWith =>
      _VulpineSettingsCopyWithImpl<VulpineSettings, VulpineSettings>(
        this as VulpineSettings,
        $identity,
        $identity,
      );
  @override
  String toString() {
    return VulpineSettingsMapper.ensureInitialized().stringifyValue(
      this as VulpineSettings,
    );
  }

  @override
  bool operator ==(Object other) {
    return VulpineSettingsMapper.ensureInitialized().equalsValue(
      this as VulpineSettings,
      other,
    );
  }

  @override
  int get hashCode {
    return VulpineSettingsMapper.ensureInitialized().hashValue(
      this as VulpineSettings,
    );
  }
}

extension VulpineSettingsValueCopy<$R, $Out>
    on ObjectCopyWith<$R, VulpineSettings, $Out> {
  VulpineSettingsCopyWith<$R, VulpineSettings, $Out> get $asVulpineSettings =>
      $base.as((v, t, t2) => _VulpineSettingsCopyWithImpl<$R, $Out>(v, t, t2));
}

abstract class VulpineSettingsCopyWith<$R, $In extends VulpineSettings, $Out>
    implements ClassCopyWith<$R, $In, $Out> {
  $R call({
    String? locale,
    ThemeMode? themeMode,
    bool? nativeTitleBar,
    String? design,
    ThemeDensity? density,
    bool? highContrast,
  });
  VulpineSettingsCopyWith<$R2, $In, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  );
}

class _VulpineSettingsCopyWithImpl<$R, $Out>
    extends ClassCopyWithBase<$R, VulpineSettings, $Out>
    implements VulpineSettingsCopyWith<$R, VulpineSettings, $Out> {
  _VulpineSettingsCopyWithImpl(super.value, super.then, super.then2);

  @override
  late final ClassMapperBase<VulpineSettings> $mapper =
      VulpineSettingsMapper.ensureInitialized();
  @override
  $R call({
    String? locale,
    ThemeMode? themeMode,
    bool? nativeTitleBar,
    String? design,
    ThemeDensity? density,
    bool? highContrast,
  }) => $apply(
    FieldCopyWithData({
      if (locale != null) #locale: locale,
      if (themeMode != null) #themeMode: themeMode,
      if (nativeTitleBar != null) #nativeTitleBar: nativeTitleBar,
      if (design != null) #design: design,
      if (density != null) #density: density,
      if (highContrast != null) #highContrast: highContrast,
    }),
  );
  @override
  VulpineSettings $make(CopyWithData data) => VulpineSettings(
    locale: data.get(#locale, or: $value.locale),
    themeMode: data.get(#themeMode, or: $value.themeMode),
    nativeTitleBar: data.get(#nativeTitleBar, or: $value.nativeTitleBar),
    design: data.get(#design, or: $value.design),
    density: data.get(#density, or: $value.density),
    highContrast: data.get(#highContrast, or: $value.highContrast),
  );

  @override
  VulpineSettingsCopyWith<$R2, VulpineSettings, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  ) => _VulpineSettingsCopyWithImpl<$R2, $Out2>($value, $cast, t);
}
