import 'dart:convert';
import 'dart:io';

Future<void> main(List<String> args) async {
  final result =
      await Process.start('ls -la', [], mode: ProcessStartMode.normal);
  await for (var line in result.stdout.transform(const Utf8Decoder())) {
    print(line);
  }
  await for (var line in result.stderr.transform(const Utf8Decoder())) {
    print(line);
  }
  final exitCode = await result.exitCode;
  if (exitCode != 0) {
    print('Process exited with code $exitCode');
  } else {
    print('Process completed successfully');
  }
}
