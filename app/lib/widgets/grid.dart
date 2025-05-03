import 'package:flutter/material.dart';

final class GridItem {
  final Widget child;
  final Rect rect;

  GridItem({required this.child, required this.rect});

  GridItem.fromLTRB({
    required Widget child,
    required double left,
    required double top,
    required double right,
    required double bottom,
  }) : this(child: child, rect: Rect.fromLTRB(left, top, right, bottom));

  GridItem.fromLTWH({
    required Widget child,
    required double left,
    required double top,
    required double width,
    required double height,
  }) : this(child: child, rect: Rect.fromLTWH(left, top, width, height));
}

final class GridLayout extends StatelessWidget {
  final List<GridItem> children;
  const GridLayout({
    super.key,
    required this.children,
    this.itemSpacing = EdgeInsets.zero,
    this.itemHeight = 200,
    this.itemWidth = 200,
  });
  final EdgeInsets itemSpacing;
  final double itemHeight;
  final double itemWidth;

  @override
  Widget build(BuildContext context) {
    final maxX = children
        .map((item) => item.rect.right)
        .reduce((a, b) => a > b ? a : b);
    final maxY = children
        .map((item) => item.rect.bottom)
        .reduce((a, b) => a > b ? a : b);
    final width = maxX * itemWidth + ((maxX + 1) * itemSpacing.horizontal);
    final height = maxY * itemHeight + ((maxY + 1) * itemSpacing.vertical);
    return SizedBox(
      width: width,
      height: height,
      child: Stack(
        children:
            children.map((item) {
              // Calculate top, left, right, bottom
              // based on itemHeight, itemWidth, and itemSpacing
              final itemRect = item.rect;
              final left =
                  itemRect.left * itemWidth +
                  ((itemRect.left + 1) * itemSpacing.horizontal);
              final top =
                  itemRect.top * itemHeight +
                  ((itemRect.top + 1) * itemSpacing.vertical);
              final width =
                  itemRect.width * itemWidth +
                  ((itemRect.width - 1) * itemSpacing.horizontal);
              final height =
                  itemRect.height * itemHeight +
                  ((itemRect.height - 1) * itemSpacing.vertical);
              return Positioned.fromRect(
                rect: Rect.fromLTWH(left, top, width, height),
                child: item.child,
              );
            }).toList(),
      ),
    );
  }
}
