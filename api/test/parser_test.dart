import 'package:petitparser/petitparser.dart';
import 'package:test/test.dart';
import 'package:vulpine_api/models/parser.dart';

void main() {
  final parser = DslGrammarDefinition().build();

  group('DSL Parser Basics', () {
    test('SET directive parsing', () {
      final result = parser.parse('SET foo "Hello {person}!"\n');
      expect(result is Success, isTrue);
    });

    test('EXEC and ARG directives', () {
      final script = 'EXEC docker\nARG compose\nARG up\n';
      final result = parser.parse(script);
      expect(result is Success, isTrue);
    });
  });
}
