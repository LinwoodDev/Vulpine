// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
// ignore_for_file: type=lint
// ignore_for_file: unused_element, unnecessary_cast, override_on_non_overriding_member
// ignore_for_file: strict_raw_type, inference_failure_on_untyped_parameter

part of 'view.dart';

class ViewVulpineMapper extends ClassMapperBase<ViewVulpine> {
  ViewVulpineMapper._();

  static ViewVulpineMapper? _instance;
  static ViewVulpineMapper ensureInitialized() {
    if (_instance == null) {
      MapperContainer.globals.use(_instance = ViewVulpineMapper._());
      ViewVulpineTileMapper.ensureInitialized();
    }
    return _instance!;
  }

  @override
  final String id = 'ViewVulpine';

  static String _$description(ViewVulpine v) => v.description;
  static const Field<ViewVulpine, String> _f$description = Field(
    'description',
    _$description,
    opt: true,
    def: '',
  );
  static List<ViewVulpineTile> _$tiles(ViewVulpine v) => v.tiles;
  static const Field<ViewVulpine, List<ViewVulpineTile>> _f$tiles = Field(
    'tiles',
    _$tiles,
    opt: true,
    def: const [],
  );

  @override
  final MappableFields<ViewVulpine> fields = const {
    #description: _f$description,
    #tiles: _f$tiles,
  };

  static ViewVulpine _instantiate(DecodingData data) {
    return ViewVulpine(
      description: data.dec(_f$description),
      tiles: data.dec(_f$tiles),
    );
  }

  @override
  final Function instantiate = _instantiate;

  static ViewVulpine fromMap(Map<String, dynamic> map) {
    return ensureInitialized().decodeMap<ViewVulpine>(map);
  }

  static ViewVulpine fromJson(String json) {
    return ensureInitialized().decodeJson<ViewVulpine>(json);
  }
}

mixin ViewVulpineMappable {
  String toJson() {
    return ViewVulpineMapper.ensureInitialized().encodeJson<ViewVulpine>(
      this as ViewVulpine,
    );
  }

  Map<String, dynamic> toMap() {
    return ViewVulpineMapper.ensureInitialized().encodeMap<ViewVulpine>(
      this as ViewVulpine,
    );
  }

  ViewVulpineCopyWith<ViewVulpine, ViewVulpine, ViewVulpine> get copyWith =>
      _ViewVulpineCopyWithImpl<ViewVulpine, ViewVulpine>(
        this as ViewVulpine,
        $identity,
        $identity,
      );
  @override
  String toString() {
    return ViewVulpineMapper.ensureInitialized().stringifyValue(
      this as ViewVulpine,
    );
  }

  @override
  bool operator ==(Object other) {
    return ViewVulpineMapper.ensureInitialized().equalsValue(
      this as ViewVulpine,
      other,
    );
  }

  @override
  int get hashCode {
    return ViewVulpineMapper.ensureInitialized().hashValue(this as ViewVulpine);
  }
}

extension ViewVulpineValueCopy<$R, $Out>
    on ObjectCopyWith<$R, ViewVulpine, $Out> {
  ViewVulpineCopyWith<$R, ViewVulpine, $Out> get $asViewVulpine =>
      $base.as((v, t, t2) => _ViewVulpineCopyWithImpl<$R, $Out>(v, t, t2));
}

abstract class ViewVulpineCopyWith<$R, $In extends ViewVulpine, $Out>
    implements ClassCopyWith<$R, $In, $Out> {
  ListCopyWith<
    $R,
    ViewVulpineTile,
    ViewVulpineTileCopyWith<$R, ViewVulpineTile, ViewVulpineTile>
  >
  get tiles;
  $R call({String? description, List<ViewVulpineTile>? tiles});
  ViewVulpineCopyWith<$R2, $In, $Out2> $chain<$R2, $Out2>(Then<$Out2, $R2> t);
}

class _ViewVulpineCopyWithImpl<$R, $Out>
    extends ClassCopyWithBase<$R, ViewVulpine, $Out>
    implements ViewVulpineCopyWith<$R, ViewVulpine, $Out> {
  _ViewVulpineCopyWithImpl(super.value, super.then, super.then2);

  @override
  late final ClassMapperBase<ViewVulpine> $mapper =
      ViewVulpineMapper.ensureInitialized();
  @override
  ListCopyWith<
    $R,
    ViewVulpineTile,
    ViewVulpineTileCopyWith<$R, ViewVulpineTile, ViewVulpineTile>
  >
  get tiles => ListCopyWith(
    $value.tiles,
    (v, t) => v.copyWith.$chain(t),
    (v) => call(tiles: v),
  );
  @override
  $R call({String? description, List<ViewVulpineTile>? tiles}) => $apply(
    FieldCopyWithData({
      if (description != null) #description: description,
      if (tiles != null) #tiles: tiles,
    }),
  );
  @override
  ViewVulpine $make(CopyWithData data) => ViewVulpine(
    description: data.get(#description, or: $value.description),
    tiles: data.get(#tiles, or: $value.tiles),
  );

  @override
  ViewVulpineCopyWith<$R2, ViewVulpine, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  ) => _ViewVulpineCopyWithImpl<$R2, $Out2>($value, $cast, t);
}

class ViewVulpineTileMapper extends ClassMapperBase<ViewVulpineTile> {
  ViewVulpineTileMapper._();

  static ViewVulpineTileMapper? _instance;
  static ViewVulpineTileMapper ensureInitialized() {
    if (_instance == null) {
      MapperContainer.globals.use(_instance = ViewVulpineTileMapper._());
      VulpineProgramMapper.ensureInitialized();
    }
    return _instance!;
  }

  @override
  final String id = 'ViewVulpineTile';

  static int _$x(ViewVulpineTile v) => v.x;
  static const Field<ViewVulpineTile, int> _f$x = Field('x', _$x);
  static int _$y(ViewVulpineTile v) => v.y;
  static const Field<ViewVulpineTile, int> _f$y = Field('y', _$y);
  static VulpineProgram? _$program(ViewVulpineTile v) => v.program;
  static const Field<ViewVulpineTile, VulpineProgram> _f$program = Field(
    'program',
    _$program,
    opt: true,
  );
  static int _$width(ViewVulpineTile v) => v.width;
  static const Field<ViewVulpineTile, int> _f$width = Field(
    'width',
    _$width,
    opt: true,
    def: 1,
  );
  static int _$height(ViewVulpineTile v) => v.height;
  static const Field<ViewVulpineTile, int> _f$height = Field(
    'height',
    _$height,
    opt: true,
    def: 1,
  );
  static Uri? _$icon(ViewVulpineTile v) => v.icon;
  static const Field<ViewVulpineTile, Uri> _f$icon = Field(
    'icon',
    _$icon,
    opt: true,
  );
  static String _$label(ViewVulpineTile v) => v.label;
  static const Field<ViewVulpineTile, String> _f$label = Field(
    'label',
    _$label,
    opt: true,
    def: '',
  );
  static String _$description(ViewVulpineTile v) => v.description;
  static const Field<ViewVulpineTile, String> _f$description = Field(
    'description',
    _$description,
    opt: true,
    def: '',
  );

  @override
  final MappableFields<ViewVulpineTile> fields = const {
    #x: _f$x,
    #y: _f$y,
    #program: _f$program,
    #width: _f$width,
    #height: _f$height,
    #icon: _f$icon,
    #label: _f$label,
    #description: _f$description,
  };

  static ViewVulpineTile _instantiate(DecodingData data) {
    return ViewVulpineTile(
      x: data.dec(_f$x),
      y: data.dec(_f$y),
      program: data.dec(_f$program),
      width: data.dec(_f$width),
      height: data.dec(_f$height),
      icon: data.dec(_f$icon),
      label: data.dec(_f$label),
      description: data.dec(_f$description),
    );
  }

  @override
  final Function instantiate = _instantiate;

  static ViewVulpineTile fromMap(Map<String, dynamic> map) {
    return ensureInitialized().decodeMap<ViewVulpineTile>(map);
  }

  static ViewVulpineTile fromJson(String json) {
    return ensureInitialized().decodeJson<ViewVulpineTile>(json);
  }
}

mixin ViewVulpineTileMappable {
  String toJson() {
    return ViewVulpineTileMapper.ensureInitialized()
        .encodeJson<ViewVulpineTile>(this as ViewVulpineTile);
  }

  Map<String, dynamic> toMap() {
    return ViewVulpineTileMapper.ensureInitialized().encodeMap<ViewVulpineTile>(
      this as ViewVulpineTile,
    );
  }

  ViewVulpineTileCopyWith<ViewVulpineTile, ViewVulpineTile, ViewVulpineTile>
  get copyWith =>
      _ViewVulpineTileCopyWithImpl<ViewVulpineTile, ViewVulpineTile>(
        this as ViewVulpineTile,
        $identity,
        $identity,
      );
  @override
  String toString() {
    return ViewVulpineTileMapper.ensureInitialized().stringifyValue(
      this as ViewVulpineTile,
    );
  }

  @override
  bool operator ==(Object other) {
    return ViewVulpineTileMapper.ensureInitialized().equalsValue(
      this as ViewVulpineTile,
      other,
    );
  }

  @override
  int get hashCode {
    return ViewVulpineTileMapper.ensureInitialized().hashValue(
      this as ViewVulpineTile,
    );
  }
}

extension ViewVulpineTileValueCopy<$R, $Out>
    on ObjectCopyWith<$R, ViewVulpineTile, $Out> {
  ViewVulpineTileCopyWith<$R, ViewVulpineTile, $Out> get $asViewVulpineTile =>
      $base.as((v, t, t2) => _ViewVulpineTileCopyWithImpl<$R, $Out>(v, t, t2));
}

abstract class ViewVulpineTileCopyWith<$R, $In extends ViewVulpineTile, $Out>
    implements ClassCopyWith<$R, $In, $Out> {
  $R call({
    int? x,
    int? y,
    VulpineProgram? program,
    int? width,
    int? height,
    Uri? icon,
    String? label,
    String? description,
  });
  ViewVulpineTileCopyWith<$R2, $In, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  );
}

class _ViewVulpineTileCopyWithImpl<$R, $Out>
    extends ClassCopyWithBase<$R, ViewVulpineTile, $Out>
    implements ViewVulpineTileCopyWith<$R, ViewVulpineTile, $Out> {
  _ViewVulpineTileCopyWithImpl(super.value, super.then, super.then2);

  @override
  late final ClassMapperBase<ViewVulpineTile> $mapper =
      ViewVulpineTileMapper.ensureInitialized();
  @override
  $R call({
    int? x,
    int? y,
    Object? program = $none,
    int? width,
    int? height,
    Object? icon = $none,
    String? label,
    String? description,
  }) => $apply(
    FieldCopyWithData({
      if (x != null) #x: x,
      if (y != null) #y: y,
      if (program != $none) #program: program,
      if (width != null) #width: width,
      if (height != null) #height: height,
      if (icon != $none) #icon: icon,
      if (label != null) #label: label,
      if (description != null) #description: description,
    }),
  );
  @override
  ViewVulpineTile $make(CopyWithData data) => ViewVulpineTile(
    x: data.get(#x, or: $value.x),
    y: data.get(#y, or: $value.y),
    program: data.get(#program, or: $value.program),
    width: data.get(#width, or: $value.width),
    height: data.get(#height, or: $value.height),
    icon: data.get(#icon, or: $value.icon),
    label: data.get(#label, or: $value.label),
    description: data.get(#description, or: $value.description),
  );

  @override
  ViewVulpineTileCopyWith<$R2, ViewVulpineTile, $Out2> $chain<$R2, $Out2>(
    Then<$Out2, $R2> t,
  ) => _ViewVulpineTileCopyWithImpl<$R2, $Out2>($value, $cast, t);
}

