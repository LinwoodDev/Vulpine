class CommandBlock {
  final String command;
  final List<CommandParameter> parameters;

  CommandBlock({required this.command, this.parameters = const []});
}

sealed class CommandParameter {
  final String description;

  const CommandParameter({this.description = ''});
}

final class CommandParameterString extends CommandParameter {
  final String defaultValue;
  final String validationRegex;
  final String? validationErrorMessage;
  final bool multiline;

  const CommandParameterString({
    this.defaultValue = '',
    this.validationRegex = r'.*',
    this.validationErrorMessage,
    this.multiline = false,
    super.description,
  });
}

final class CommandParameterChoice extends CommandParameter {
  final String choices;
  final String defaultValue;
  final String delimiter;
  final int maxChoices;

  const CommandParameterChoice({
    required this.choices,
    this.defaultValue = '',
    this.delimiter = ',',
    this.maxChoices = 1,
    super.description,
  });
}

final class CommandParameterCheckbox extends CommandParameter {
  final bool? defaultValue;
  final String trueLabel;
  final String falseLabel;
  final String? undeterminedLabel;

  const CommandParameterCheckbox({
    this.defaultValue,
    super.description,
    this.trueLabel = 'true',
    this.falseLabel = 'false',
    this.undeterminedLabel,
  });

  bool get tristate => undeterminedLabel != null;
}

final class CommandParameterNumber extends CommandParameter {
  final String defaultValue;
  final String validationRegex;
  final String? validationErrorMessage;
  final double minValue;
  final double maxValue;
  final int decimalPlaces;
  final String? unitLabel;

  const CommandParameterNumber({
    this.defaultValue = '0',
    this.validationRegex = r'.*',
    this.validationErrorMessage,
    this.minValue = double.negativeInfinity,
    this.maxValue = double.infinity,
    this.decimalPlaces = 0,
    this.unitLabel,
    super.description,
  });
}
