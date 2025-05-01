import 'dart:ui';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';

final class CarouselScrollBehavior extends ScrollBehavior {
  final double itemExtent;
  const CarouselScrollBehavior({required this.itemExtent});
  // Override behavior methods and getters like dragDevices
  @override
  Set<PointerDeviceKind> get dragDevices => {
    PointerDeviceKind.touch,
    PointerDeviceKind.mouse,
    // etc.
  };
}

class VulpineCarouselView extends StatefulWidget {
  final List<Widget> children;
  final double itemExtent;
  final double shrinkExtent;

  const VulpineCarouselView({
    super.key,
    required this.children,
    this.itemExtent = 330,
    this.shrinkExtent = 200,
  });

  @override
  State<VulpineCarouselView> createState() => _VulpineCarouselViewState();
}

class _VulpineCarouselViewState extends State<VulpineCarouselView> {
  final CarouselController _carouselController = CarouselController();
  double _nextOffset = 0;
  double _maxOffset = 0;

  @override
  void initState() {
    super.initState();
    _updateMaxPosition();
  }

  @override
  void dispose() {
    super.dispose();
    _carouselController.dispose();
  }

  void _updateMaxPosition() {
    _maxOffset =
        widget.itemExtent +
        (widget.shrinkExtent * (widget.children.length - 1));
  }

  void _animateToPrevious() =>
      _animateTo(_carouselController.offset - widget.itemExtent);
  void _animateToNext() =>
      _animateTo(_carouselController.offset + widget.itemExtent);

  void animateToNext(int index) {
    double to = 0;
    if (index > 1) {
      to = widget.itemExtent + (widget.shrinkExtent * (index - 1));
    } else if (index == 1) {
      to = widget.itemExtent;
    }
    _animateTo(to);
  }

  void _animateTo(double to) {
    _carouselController.animateTo(
      to,
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
    setState(() {
      _nextOffset = to;
    });
  }

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxHeight: 200),
      child: Stack(
        children: [
          ScrollConfiguration(
            behavior: CarouselScrollBehavior(itemExtent: widget.itemExtent),
            child: CarouselView(
              itemSnapping: false,
              itemExtent: widget.itemExtent,
              shrinkExtent: widget.shrinkExtent,
              controller: _carouselController,
              children: widget.children,
            ),
          ),
          Align(
            alignment: Alignment.centerLeft,
            child: AnimatedScale(
              alignment: Alignment.centerLeft,
              scale: _nextOffset > 0 ? 1 : 0,
              duration: const Duration(milliseconds: 300),
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: IconButton(
                  icon: const Icon(PhosphorIconsLight.arrowLeft),
                  onPressed: _animateToPrevious,
                ),
              ),
            ),
          ),
          Align(
            alignment: Alignment.centerRight,
            child: AnimatedScale(
              alignment: Alignment.centerRight,
              scale: _nextOffset < _maxOffset ? 1 : 0,
              duration: const Duration(milliseconds: 300),
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: IconButton(
                  icon: const Icon(PhosphorIconsLight.arrowRight),
                  onPressed: _animateToNext,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
