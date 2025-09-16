import 'package:url_launcher/url_launcher.dart';
import 'package:vulpine_api/models/program.dart';

Future<void> runProgram(VulpineProgram program) async {
  switch (program) {
    case VulpineLinkProgram():
      launchUrl(program.url);
  }
}
