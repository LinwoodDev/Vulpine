// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
// ignore_for_file: type=lint
// ignore_for_file: unused_element, unnecessary_cast, override_on_non_overriding_member
// ignore_for_file: strict_raw_type, inference_failure_on_untyped_parameter

part of 'program.dart';

class VulpineProgramMapper extends ClassMapperBase<VulpineProgram> {
  VulpineProgramMapper._();

  static VulpineProgramMapper? _instance;
  static VulpineProgramMapper ensureInitialized() {
    if (_instance == null) {
      MapperContainer.globals.use(_instance = VulpineProgramMapper._());
      VulpineLinkProgramMapper.ensureInitialized();
    }
    return _instance!;
  }

  @override
  final String id = 'VulpineProgram';

  @override
  final MappableFields<VulpineProgram> fields = const {};

  static VulpineProgram _instantiate(DecodingData data) {
    throw MapperException.missingConstructor('VulpineProgram');
  }

  @override
  final Function instantiate = _instantiate;

  static VulpineProgram fromMap(Map<String, dynamic> map) {
    return ensureInitialized().decodeMap<VulpineProgram>(map);
  }

  static VulpineProgram fromJson(String json) {
    return ensureInitialized().decodeJson<VulpineProgram>(json);
  }
}

mixin VulpineProgramMappable {
  String toJson();
  Map<String, dynamic> toMap();
  VulpineProgramCopyWith<VulpineProgram, VulpineProgram, VulpineProgram>
  get copyWith;
}

abstract class VulpineProgramCopyWith<$R, $In extends VulpineProgram, $Out>
    implements ClassCopyWith<$R, $In, $Out> {
  $R call();
  VulpineProgramCopyWith<$R2, $In, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  );
}

class VulpineLinkProgramMapper extends ClassMapperBase<VulpineLinkProgram> {
  VulpineLinkProgramMapper._();

  static VulpineLinkProgramMapper? _instance;
  static VulpineLinkProgramMapper ensureInitialized() {
    if (_instance == null) {
      MapperContainer.globals.use(_instance = VulpineLinkProgramMapper._());
      VulpineProgramMapper.ensureInitialized();
    }
    return _instance!;
  }

  @override
  final String id = 'VulpineLinkProgram';

  static Uri _$url(VulpineLinkProgram v) => v.url;
  static const Field<VulpineLinkProgram, Uri> _f$url = Field('url', _$url);

  @override
  final MappableFields<VulpineLinkProgram> fields = const {#url: _f$url};

  static VulpineLinkProgram _instantiate(DecodingData data) {
    return VulpineLinkProgram(data.dec(_f$url));
  }

  @override
  final Function instantiate = _instantiate;

  static VulpineLinkProgram fromMap(Map<String, dynamic> map) {
    return ensureInitialized().decodeMap<VulpineLinkProgram>(map);
  }

  static VulpineLinkProgram fromJson(String json) {
    return ensureInitialized().decodeJson<VulpineLinkProgram>(json);
  }
}

mixin VulpineLinkProgramMappable {
  String toJson() {
    return VulpineLinkProgramMapper.ensureInitialized()
        .encodeJson<VulpineLinkProgram>(this as VulpineLinkProgram);
  }

  Map<String, dynamic> toMap() {
    return VulpineLinkProgramMapper.ensureInitialized()
        .encodeMap<VulpineLinkProgram>(this as VulpineLinkProgram);
  }

  VulpineLinkProgramCopyWith<
    VulpineLinkProgram,
    VulpineLinkProgram,
    VulpineLinkProgram
  >
  get copyWith =>
      _VulpineLinkProgramCopyWithImpl<VulpineLinkProgram, VulpineLinkProgram>(
        this as VulpineLinkProgram,
        $identity,
        $identity,
      );
  @override
  String toString() {
    return VulpineLinkProgramMapper.ensureInitialized().stringifyValue(
      this as VulpineLinkProgram,
    );
  }

  @override
  bool operator ==(Object other) {
    return VulpineLinkProgramMapper.ensureInitialized().equalsValue(
      this as VulpineLinkProgram,
      other,
    );
  }

  @override
  int get hashCode {
    return VulpineLinkProgramMapper.ensureInitialized().hashValue(
      this as VulpineLinkProgram,
    );
  }
}

extension VulpineLinkProgramValueCopy<$R, $Out>
    on ObjectCopyWith<$R, VulpineLinkProgram, $Out> {
  VulpineLinkProgramCopyWith<$R, VulpineLinkProgram, $Out>
  get $asVulpineLinkProgram => $base.as(
    (v, t, t2) => _VulpineLinkProgramCopyWithImpl<$R, $Out>(v, t, t2),
  );
}

abstract class VulpineLinkProgramCopyWith<
  $R,
  $In extends VulpineLinkProgram,
  $Out
>
    implements VulpineProgramCopyWith<$R, $In, $Out> {
  @override
  $R call({Uri? url});
  VulpineLinkProgramCopyWith<$R2, $In, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  );
}

class _VulpineLinkProgramCopyWithImpl<$R, $Out>
    extends ClassCopyWithBase<$R, VulpineLinkProgram, $Out>
    implements VulpineLinkProgramCopyWith<$R, VulpineLinkProgram, $Out> {
  _VulpineLinkProgramCopyWithImpl(super.value, super.then, super.then2);

  @override
  late final ClassMapperBase<VulpineLinkProgram> $mapper =
      VulpineLinkProgramMapper.ensureInitialized();
  @override
  $R call({Uri? url}) =>
      $apply(FieldCopyWithData({if (url != null) #url: url}));
  @override
  VulpineLinkProgram $make(CopyWithData data) =>
      VulpineLinkProgram(data.get(#url, or: $value.url));

  @override
  VulpineLinkProgramCopyWith<$R2, VulpineLinkProgram, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  ) => _VulpineLinkProgramCopyWithImpl<$R2, $Out2>($value, $cast, t);
}

