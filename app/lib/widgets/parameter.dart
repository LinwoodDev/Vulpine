import 'package:flutter/material.dart';
import 'package:vulpine_api/api.dart';

class CommandParameterView extends StatefulWidget {
  final CommandContext context;
  final CommandParameter parameter;
  final String? value;
  final ValueChanged<String?> onChanged;

  const CommandParameterView({
    super.key,
    this.value,
    required this.parameter,
    required this.onChanged,
    required this.context,
  });

  @override
  State<CommandParameterView> createState() => _CommandParameterViewState();
}

class _CommandParameterViewState extends State<CommandParameterView> {
  @override
  Widget build(BuildContext context) {
    final value = widget.value;
    return switch (widget.parameter) {
      CommandParameterString p => TextField(
        controller: TextEditingController(text: widget.value),
        onChanged: widget.onChanged,
        decoration: InputDecoration(labelText: p.description),
      ),
      CommandParameterChoice p => DropdownButton<String>(
        value: value,
        items:
            widget.context.enums[p.choices]?.values.map((String choice) {
              return DropdownMenuItem<String>(
                value: choice,
                child: Text(choice),
              );
            }).toList() ??
            [],
        onChanged: (value) => widget.onChanged(value),
      ),
      CommandParameterCheckbox p => CheckboxListTile(
        title: Text(widget.parameter.description),
        value:
            p.tristate
                ? (value == p.trueLabel
                    ? true
                    : value == p.falseLabel
                    ? false
                    : null)
                : (value == p.trueLabel),
        tristate: p.tristate,
        onChanged: (value) => widget.onChanged(value.toString()),
      ),
      CommandParameterNumber p => TextField(
        controller: TextEditingController(text: widget.value),
        onChanged: widget.onChanged,
        decoration: InputDecoration(labelText: p.description),
        keyboardType: TextInputType.number,
      ),
    };
  }
}
