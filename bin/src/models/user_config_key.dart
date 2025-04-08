import 'package:recase/recase.dart';

enum UserConfigKey {
  djangoVersion,
  useAlfredCache,
  useFileCache,
  cacheTtl,
  fileCacheMaxEntries;

  @override
  String toString() => name.snakeCase;
}
