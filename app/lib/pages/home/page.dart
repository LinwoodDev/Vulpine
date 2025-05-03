import 'package:flutter/material.dart';
import 'package:material_leap/widgets.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';
import 'package:vulpine/api/settings.dart';
import 'package:vulpine/cubits/settings.dart';
import 'package:vulpine/main.dart';
import 'package:vulpine/widgets/grid.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: WindowTitleBar<SettingsCubit, VulpineSettings>(
        title: Text(applicationName),
        actions: [
          IconButton(
            icon: const Icon(PhosphorIconsLight.gear),
            onPressed: () => openSettings(context),
          ),
        ],
      ),
      body: ListView(
        children: [
          Align(
            child: GridLayout(
              itemSpacing: const EdgeInsets.all(4),
              children: [
                ...List.generate(
                  10,
                  (index) => GridItem.fromLTWH(
                    child: LayoutCard(
                      color: Colors.primaries[index % Colors.primaries.length]
                          .withValues(alpha: 0.5),
                      child: Center(
                        child: Text(
                          'Item $index',
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 20,
                          ),
                          overflow: TextOverflow.clip,
                          softWrap: false,
                        ),
                      ),
                    ),
                    left: index % 2,
                    top: (index ~/ 2).toDouble(),
                    width: 1,
                    height: 1,
                  ),
                ),
                GridItem.fromLTWH(
                  child: LayoutCard(
                    color: Colors.red,
                    child: Center(
                      child: Text(
                        'Item Big',
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 20,
                        ),
                        overflow: TextOverflow.clip,
                        softWrap: false,
                      ),
                    ),
                  ),
                  left: 2,
                  top: 0,
                  width: 3,
                  height: 1,
                ),
                GridItem.fromLTWH(
                  child: LayoutCard(
                    color: Colors.green,
                    child: Center(
                      child: Text(
                        'Item Big',
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 20,
                        ),
                        overflow: TextOverflow.clip,
                        softWrap: false,
                      ),
                    ),
                  ),
                  left: 2,
                  top: 1,
                  width: 1,
                  height: 3,
                ),
                GridItem.fromLTWH(
                  child: LayoutCard(
                    color: Colors.green,
                    child: Image.asset('images/linwood.png'),
                  ),
                  left: 3,
                  top: 1,
                  width: 2,
                  height: 2,
                ),
                GridItem.fromLTWH(
                  child: LayoutCard(
                    color: Colors.orange,
                    child: Image.asset('images/logo.png'),
                  ),
                  left: 3,
                  top: 3,
                  width: 2,
                  height: 2,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class LayoutCard extends StatefulWidget {
  final Widget child;
  final Color color;

  const LayoutCard({super.key, required this.child, required this.color});

  @override
  State<LayoutCard> createState() => _LayoutCardState();
}

class _LayoutCardState extends State<LayoutCard> {
  final FocusNode _focusNode = FocusNode();
  bool _focused = false, _hovered = false;

  @override
  void dispose() {
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        AnimatedContainer(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(8),
            color: widget.color,
            border:
                _hovered
                    ? Border.all(
                      color: ColorScheme.of(
                        context,
                      ).primary.withValues(alpha: 0.5),
                      width: 6,
                    )
                    : _focused
                    ? Border.all(
                      color: ColorScheme.of(
                        context,
                      ).secondary.withValues(alpha: 0.5),
                      width: 6,
                    )
                    : null,
            boxShadow: [
              BoxShadow(
                color: Colors.black.withValues(alpha: 0.2),
                blurRadius: 4,
                offset: const Offset(0, 2),
              ),
            ],
          ),
          duration: const Duration(milliseconds: 300),
          child: widget.child,
        ),
        Material(
          type: MaterialType.transparency,
          child: InkWell(
            focusNode: _focusNode,
            onTap: () {
              FocusScope.of(context).requestFocus(_focusNode);
            },
            borderRadius: BorderRadius.circular(8),
            onHover: (hovered) {
              setState(() {
                _hovered = hovered;
              });
            },
            onFocusChange: (focused) {
              setState(() {
                _focused = focused;
              });
            },
          ),
        ),
      ],
    );
  }
}
