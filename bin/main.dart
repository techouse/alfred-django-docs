import 'dart:io' show exitCode, stdout;

import 'package:alfred_workflow/alfred_workflow.dart';
import 'package:algoliasearch/src/model/hit.dart';
import 'package:algoliasearch/src/model/search_response.dart';
import 'package:args/args.dart' show ArgParser, ArgResults;
import 'package:cli_script/cli_script.dart';

import 'src/env/env.dart';
import 'src/extensions/string_helpers.dart';
import 'src/models/search_result.dart' show SearchResult;
import 'src/models/user_config_key.dart' show UserConfigKey;
import 'src/services/algolia_search.dart' show AlgoliaSearch;

part 'main_helpers.dart';

bool _verbose = false;
bool _update = false;

void main(List<String> arguments) {
  wrapMain(() async {
    try {
      exitCode = 0;

      _workflow.clearItems();

      final ArgParser parser = ArgParser()
        ..addOption('query', abbr: 'q', defaultsTo: '')
        ..addFlag('verbose', abbr: 'v', defaultsTo: false)
        ..addFlag('update', abbr: 'u', defaultsTo: false);
      final ArgResults args = parser.parse(arguments);

      _update = args['update'];
      if (_update) {
        stdout.writeln('Updating workflow...');

        return await _updater.update();
      }

      _verbose = args['verbose'];

      final Map<String, AlfredUserConfiguration>? userDefaults =
          await _workflow.getUserDefaults();

      final AlfredUserConfigurationSelect? djangoVersion =
          userDefaults?[UserConfigKey.djangoVersion.toString()]
              as AlfredUserConfigurationSelect?;

      final AlfredUserConfigurationCheckBox? useFileCache =
          userDefaults?[UserConfigKey.useFileCache.toString()]
              as AlfredUserConfigurationCheckBox?;

      final AlfredUserConfigurationNumberSlider? fileCacheMaxEntries =
          userDefaults?[UserConfigKey.fileCacheMaxEntries.toString()]
              as AlfredUserConfigurationNumberSlider?;

      final AlfredUserConfigurationCheckBox? useAlfredCache =
          userDefaults?[UserConfigKey.useAlfredCache.toString()]
              as AlfredUserConfigurationCheckBox?;

      final AlfredUserConfigurationNumberSlider? cacheTimeToLive =
          userDefaults?[UserConfigKey.cacheTtl.toString()]
              as AlfredUserConfigurationNumberSlider?;

      if (djangoVersion == null) {
        throw Exception('django_version not set!');
      }

      List<String> query =
          args['query'].replaceAll(RegExp(r'\s+'), ' ').trim().split(' ');
      query.removeWhere((str) => str == djangoVersion.value);
      final String queryString = query.join(' ').trim().toLowerCase();

      if (_verbose) stdout.writeln('Query: "$queryString"');

      if (useAlfredCache?.value ?? false) {
        _workflow.useAutomaticCache = true;
      } else if (useFileCache?.value ?? false) {
        _workflow.maxCacheEntries =
            fileCacheMaxEntries?.value ?? fileCacheMaxEntries?.defaultValue;
      }

      _workflow.cacheTimeToLive = cacheTimeToLive?.value;

      if (queryString.isEmpty) {
        _showPlaceholder();
      } else {
        if (useFileCache?.value ?? false) {
          _workflow.cacheKey = '${queryString}_${djangoVersion.value}';
        }
        if ((await _workflow.getItems()).isEmpty) {
          await _performSearch(queryString, version: djangoVersion.value);
        }
      }
    } on FormatException catch (err) {
      exitCode = 2;
      _workflow.addItem(AlfredItem(title: err.toString()));
    } catch (err) {
      exitCode = 1;
      _workflow.addItem(AlfredItem(title: err.toString()));
      if (_verbose) rethrow;
    } finally {
      if (!_update) {
        if (await _updater.updateAvailable()) {
          _workflow.run(addToBeginning: _updateItem);
        } else {
          _workflow.run();
        }
      }
    }
  });
}
