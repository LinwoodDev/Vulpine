class CommandContext {
  final Map<String, EnumType> enums;

  const CommandContext({this.enums = const {}});
}

class EnumType {
  final Set<String> values;

  EnumType({this.values = const {}});
}
