import 'package:json_annotation/json_annotation.dart';

part 'search_result.g.dart';

@JsonSerializable()
class SearchResult {
  const SearchResult({
    required this.objectID,
    required this.categories,
    required this.content,
    required this.id,
    required this.permalink,
    required this.title,
    required this.version,
  });

  final String objectID;
  final List<String> categories;
  final String content;
  final String id;
  final String permalink;
  final String title;
  final num version;

  String get prettyTitle {
    if (id.startsWith('django.')) return id;
    if (id.startsWith('setting-')) return '$title [setting]';
    if (id.startsWith('templatefilter-')) return '$title [template filter]';
    if (id.startsWith('fieldlookup-')) return '$title [field lookup]';
    if (id.startsWith('templatetag-')) return '$title [template tag]';
    if (id.startsWith('cmdoption-')) return '$title [cmd option]';
    if (id.startsWith('envvar-')) return '$title [env var]';

    return title;
  }

  static const List<String> attributesToRetrieve = [
    'categories',
    'content',
    'id',
    'permalink',
    'title',
    'version',
  ];

  factory SearchResult.fromJson(Map<String, dynamic> json) =>
      _$SearchResultFromJson(json);

  Map<String, dynamic> toJson() => _$SearchResultToJson(this);
}
