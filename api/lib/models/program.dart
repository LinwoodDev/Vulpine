import 'package:dart_mappable/dart_mappable.dart';

part 'program.mapper.dart';

@MappableClass()
sealed class VulpineProgram with VulpineProgramMappable {
  const VulpineProgram();
}

@MappableClass()
final class VulpineLinkProgram extends VulpineProgram
    with VulpineLinkProgramMappable {
  final Uri url;

  const VulpineLinkProgram(this.url);
}
