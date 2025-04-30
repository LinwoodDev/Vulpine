import 'package:vulpine_api/models/parser.dart';

final class VulpineDSLRunResult {
  final String executable;
  final List<String> arguments;
  final Map<String, String> environment;
  final String? workingDirectory;

  VulpineDSLRunResult({
    required this.executable,
    required this.arguments,
    required this.environment,
    required this.workingDirectory,
  });
}

final class VulpineDSLResult {
  final List<VulpineDSLRunResult> commands;

  VulpineDSLResult({this.commands = const []});
}

VulpineDSLResult evaluate(
  VulpineDSL dsl, {
  Map<String, String> parameters = const {},
}) {
  final List<VulpineDSLRunResult> commands = [];

  String evaluateString(VulpineDSLString string) {
    final value = string.parts
        .map((part) {
          return switch (part) {
            VulpineDSLRawStringPart() => part.value,
            VulpineDSLVariableStringPart() => parameters[part.variable] ?? '',
          };
        })
        .join('');
    if (!string.calcLength) return value;
    return value.length.toString();
  }

  Map<String, String> env = {};
  List<String> args = [];
  String? currentExec;
  String? currentWorkingDirectory;

  void addCurrentExec() {
    if (currentExec == null) return;
    commands.add(
      VulpineDSLRunResult(
        executable: currentExec!,
        arguments: args,
        environment: env,
        workingDirectory: currentWorkingDirectory,
      ),
    );
    currentExec = null;
    args = [];
    env = {};
    currentWorkingDirectory = null;
  }

  bool evaluateCondition(VulpineDSLCondition condition) {
    switch (condition) {
      case VulpineDSLNotCondition():
        return !evaluateCondition(condition.condition);
      case VulpineDSLLogicalCondition():
        return switch (condition.operator) {
          LogicalOperator.and => condition.conditions.every(evaluateCondition),
          LogicalOperator.or => condition.conditions.any(evaluateCondition),
        };
      case VulpineDSLComparisonCondition():
        final left = parameters[condition.left] ?? '';
        final right = parameters[condition.right] ?? '';
        return switch (condition.operator) {
          EqualityOperator.equal => left == right,
          EqualityOperator.notEqual => left != right,
          EqualityOperator.greaterThan =>
            (int.tryParse(left) ?? 0) > (int.tryParse(right) ?? 0),
          EqualityOperator.lessThan => left.compareTo(right) < 0,
          EqualityOperator.greaterThanOrEqual => left.compareTo(right) >= 0,
          EqualityOperator.lessThanOrEqual => left.compareTo(right) <= 0,
        };
    }
  }

  void runCommand() {
    for (final command in dsl.commands) {
      switch (command) {
        case VulpineDSLSetCommand():
          parameters[command.variable] = evaluateString(command.value);
        case VulpineDSLEnvCommand():
          env[command.variable] = evaluateString(command.value);
        case VulpineDSLExecCommand():
          addCurrentExec();
          currentExec = command.executable;
        case VulpineDSLArgCommand():
          args.add(evaluateString(command.argument));
        case VulpineDSLIfCommand():
          if (evaluateCondition(command.condition)) {
            runCommand();
          }
      }
    }
  }

  runCommand();

  return VulpineDSLResult(commands: commands);
}
