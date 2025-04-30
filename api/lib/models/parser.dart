import 'package:petitparser/petitparser.dart';

final class VulpineDSL {
  final List<VulpineDSLCommand> commands;

  VulpineDSL(this.commands);
}

final class VulpineDSLString {
  final bool calcLength;
  final List<VulpineDSLStringPart> parts;

  VulpineDSLString(this.parts, {this.calcLength = false});
}

sealed class VulpineDSLStringPart {
  VulpineDSLStringPart();
}

final class VulpineDSLRawStringPart extends VulpineDSLStringPart {
  final String value;

  VulpineDSLRawStringPart(this.value);
}

final class VulpineDSLVariableStringPart extends VulpineDSLStringPart {
  final String variable;

  VulpineDSLVariableStringPart(this.variable);
}

sealed class VulpineDSLCommand {
  VulpineDSLCommand();
}

final class VulpineDSLSetCommand extends VulpineDSLCommand {
  final String variable;
  final VulpineDSLString value;

  VulpineDSLSetCommand(this.variable, this.value);
}

final class VulpineDSLEnvCommand extends VulpineDSLCommand {
  final String variable;
  final VulpineDSLString value;

  VulpineDSLEnvCommand(this.variable, this.value);
}

final class VulpineDSLExecCommand extends VulpineDSLCommand {
  final String executable;

  VulpineDSLExecCommand(this.executable);
}

final class VulpineDSLArgCommand extends VulpineDSLCommand {
  final VulpineDSLString argument;

  VulpineDSLArgCommand(this.argument);
}

enum EqualityOperator {
  equal('=='),
  notEqual('!='),
  greaterThanOrEqual('>='),
  lessThanOrEqual('<='),
  greaterThan('>'),
  lessThan('<');

  final String symbol;

  const EqualityOperator(this.symbol);
}

enum LogicalOperator {
  and('&&'),
  or('||');

  final String symbol;
  const LogicalOperator(this.symbol);
}

sealed class VulpineDSLCondition {}

final class VulpineDSLComparisonCondition extends VulpineDSLCondition {
  final String left;
  final EqualityOperator operator;
  final String right;

  VulpineDSLComparisonCondition(this.left, this.operator, this.right);
}

final class VulpineDSLLogicalCondition extends VulpineDSLCondition {
  final LogicalOperator operator;
  final List<VulpineDSLCondition> conditions;

  VulpineDSLLogicalCondition(this.operator, this.conditions);
}

final class VulpineDSLNotCondition extends VulpineDSLCondition {
  final VulpineDSLCondition condition;

  VulpineDSLNotCondition(this.condition);
}

final class VulpineDSLElseIfCommand {
  final VulpineDSLCondition condition;
  final List<VulpineDSLCommand> commands;

  VulpineDSLElseIfCommand(this.condition, this.commands);
}

final class VulpineDSLIfCommand extends VulpineDSLCommand {
  final VulpineDSLCondition condition;
  final List<VulpineDSLCommand> commands;
  final List<VulpineDSLElseIfCommand> elseIfCommands;
  final List<VulpineDSLCommand>? elseCommands;

  VulpineDSLIfCommand(
    this.condition,
    this.commands, {
    this.elseIfCommands = const [],
    this.elseCommands,
  });
}

const Map<String, String> jsonEscapeChars = {
  '\\': '\\',
  '/': '/',
  '"': '"',
  'b': '\b',
  'f': '\f',
  'n': '\n',
  'r': '\r',
  't': '\t',
};

/// Extended grammar definition with interpolation support
class DslGrammarDefinition extends GrammarDefinition<VulpineDSL> {
  @override
  Parser<VulpineDSL> start() =>
      ref0(entries).trim().map((values) => VulpineDSL(values));

  Parser<List<VulpineDSLCommand>> entries() => value()
      .starSeparated(whitespace().star())
      .map((values) => values.elements);

  Parser<VulpineDSLCommand> value() =>
      [
        ref0(setCommand),
        ref0(envCommand),
        ref0(execCommand),
        ref0(argCommand),
        ref0(ifCommand),
        failure<VulpineDSLCommand>('value expected'),
      ].toChoiceParser();

  // SET command now uses ref0(interpolatedString)
  Parser<VulpineDSLSetCommand> setCommand() => (string('SET') &
          whitespace() &
          ref0(identifier) &
          whitespace() &
          ref0(interpolatedString))
      .map(
        (values) =>
            VulpineDSLSetCommand(values[2], values[4] as VulpineDSLString),
      );

  Parser<VulpineDSLEnvCommand> envCommand() => (string('ENV') &
          whitespace() &
          ref0(identifier) &
          whitespace() &
          ref0(interpolatedString))
      .map(
        (values) =>
            VulpineDSLEnvCommand(values[2], values[4] as VulpineDSLString),
      );

  Parser<VulpineDSLExecCommand> execCommand() =>
      (string('EXEC') & whitespace() & ref0(identifier)).map(
        (values) => VulpineDSLExecCommand(values[2]),
      );

  Parser<VulpineDSLArgCommand> argCommand() =>
      (string('ARG') & whitespace() & ref0(interpolatedString)).map(
        (values) => VulpineDSLArgCommand(values[2]),
      );

  Parser<String> identifier() => (letter() & word().star()).flatten();

  /// The main entrypoint for quoted strings with {interpolation}.
  Parser<VulpineDSLString> interpolatedString() => seq4(
    char('#').optional(),
    char('"'),
    ref0(stringContent).star(),
    char('"'),
  ).map4(
    (length, _, value, _) =>
        VulpineDSLString(value, calcLength: length != null),
  );

  /// A single piece of the content: either raw text, or a {variable}.
  Parser<VulpineDSLStringPart> stringContent() =>
      [ref0(rawStringPart), ref0(variableStringPart)].toChoiceParser();

  /// Plain text up to the next '{' or '"'
  Parser<VulpineDSLRawStringPart> rawStringPart() => pattern(
    '^{"',
  ).plus().flatten().map((txt) => VulpineDSLRawStringPart(txt));

  /// Matches `{ identifier }` and yields a Variable part
  Parser<VulpineDSLVariableStringPart> variableStringPart() => seq3(
    char('{').trim(),
    ref0(identifier),
    char('}').trim(),
  ).map3((l, name, r) => VulpineDSLVariableStringPart(name));

  Parser<EqualityOperator> equalityOperator() =>
      [
        string('==').map((_) => EqualityOperator.equal),
        string('!=').map((_) => EqualityOperator.notEqual),
        string('>').map((_) => EqualityOperator.greaterThan),
        string('<').map((_) => EqualityOperator.lessThan),
        string('>=').map((_) => EqualityOperator.greaterThanOrEqual),
        string('<=').map((_) => EqualityOperator.lessThanOrEqual),
      ].toChoiceParser();

  Parser<LogicalOperator> logicalOperator() =>
      [
        string('&&').map((_) => LogicalOperator.and),
        string('||').map((_) => LogicalOperator.or),
      ].toChoiceParser();

  Parser<VulpineDSLCondition> condition() => seq3(
    char('('),
    [
      ref0(comparisonCondition),
      ref0(logicalCondition),
      ref0(notCondition),
      failure<VulpineDSLCondition>('condition expected'),
    ].toChoiceParser(),
    char(')'),
  ).map3((_, condition, _) => condition);

  Parser<VulpineDSLComparisonCondition> comparisonCondition() => seq3(
    ref0(identifier).trim(),
    ref0(equalityOperator).trim(),
    ref0(identifier).trim(),
  ).map3(
    (left, operator, right) =>
        VulpineDSLComparisonCondition(left, operator, right),
  );

  Parser<VulpineDSLLogicalCondition> logicalCondition() => seq3(
    ref0(condition).trim(),
    ref0(logicalOperator).trim(),
    ref0(condition).trim(),
  ).map3(
    (left, operator, right) =>
        VulpineDSLLogicalCondition(operator, [left, right]),
  );

  Parser<VulpineDSLNotCondition> notCondition() => seq2(
    string('NOT').trim(),
    ref0(condition).trim(),
  ).map2((_, condition) => VulpineDSLNotCondition(condition));

  Parser<VulpineDSLIfCommand> ifCommand() => seq7(
    string('IF'),
    ref0(condition).trim(),
    string('THEN'),
    ref0(entries),
    seq4(
      string('ELSE IF'),
      ref0(condition).trim(),
      string('THEN'),
      ref0(entries),
    ).star(),
    seq2(string('ELSE'), ref0(entries)).optional(),
    string('ENDIF'),
  ).map((values) {
    return VulpineDSLIfCommand(
      values.$2,
      values.$4,
      elseIfCommands:
          values.$5.map((e) => VulpineDSLElseIfCommand(e.$2, e.$4)).toList(),
      elseCommands: values.$6?.$2,
    );
  });
}
