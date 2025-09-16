import 'package:flutter/material.dart';
import 'package:material_leap/material_leap.dart';
import 'package:vulpine/pages/home/page.dart';
import 'package:vulpine/widgets/grid.dart';
import 'package:vulpine_api/api.dart';

GridItem buildTile(BuildContext context, ViewVulpineTile tile) {
  final icon = tile.icon?.data?.contentAsBytes();
  return GridItem.fromLTWH(
    child: LayoutCard(
      color: tile.color?.toColor() ?? ColorScheme.of(context).primaryContainer,
      child: icon == null ? null : Image.memory(icon),
    ),
    left: tile.x.toDouble(),
    top: tile.y.toDouble(),
    width: tile.width.toDouble(),
    height: tile.height.toDouble(),
  );
}

class HomeView extends StatelessWidget {
  final ViewVulpine view;

  const HomeView({super.key, required this.view});

  @override
  Widget build(BuildContext context) {
    return GridLayout(
      itemSpacing: const EdgeInsets.all(4),
      children: [...view.tiles.map((tile) => buildTile(context, tile))],
    );
  }
}
