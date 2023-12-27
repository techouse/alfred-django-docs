import 'package:algoliasearch/algoliasearch_lite.dart';

import '../env/env.dart';
import '../models/search_result.dart';

class AlgoliaSearch {
  AlgoliaSearch._();

  static final SearchClient _client = SearchClient(
    appId: Env.algoliaApplicationId,
    apiKey: Env.algoliaSearchOnlyApiKey,
  );

  static Future<SearchResponse> query(
    String queryString, {
    required String version,
  }) =>
      _client.searchIndex(
        request: SearchForHits(
          indexName: Env.algoliaSearchIndex,
          query: queryString,
          facetFilters: [
            'version:${version.replaceAll('v', '')}',
          ],
          attributesToRetrieve: SearchResult.attributesToRetrieve,
          page: 0,
          hitsPerPage: 20,
        ),
      );

  static dispose() => _client.dispose();
}
