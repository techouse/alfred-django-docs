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

  SearchResult.fromJson(Map<String, dynamic> json)
      : objectID = json['objectID'] as String,
        categories = List<String>.from(json['categories'] as List),
        content = json['content'] as String,
        id = json['id'] as String,
        permalink = json['permalink'] as String,
        title = json['title'] as String,
        version = json['version'] as num;

  Map<String, dynamic> toJson() => {
        'objectID': objectID,
        'categories': categories,
        'content': content,
        'id': id,
        'permalink': permalink,
        'title': title,
        'version': version,
      };
}
