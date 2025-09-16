import 'package:dart_leap/dart_leap.dart';
import 'package:dart_mappable/dart_mappable.dart';
import 'package:vulpine_api/models/program.dart';

part 'view.mapper.dart';

@MappableClass()
final class ViewVulpine with ViewVulpineMappable {
  final String description;
  final List<ViewVulpineTile> tiles;

  ViewVulpine({this.description = '', this.tiles = const []});
}

@MappableClass()
final class ViewVulpineTile with ViewVulpineTileMappable {
  final VulpineProgram? program;
  final int x, y, width, height;
  final Uri? icon;
  final String label;
  final SRGBColor? color;
  final String description;

  ViewVulpineTile({
    required this.x,
    required this.y,
    this.program,
    this.width = 1,
    this.height = 1,
    this.icon,
    this.label = '',
    this.description = '',
    this.color,
  });
}
