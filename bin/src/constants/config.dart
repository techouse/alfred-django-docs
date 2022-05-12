class Config {
  Config._();

  static const String version = '2.0.9';
  static final Uri githubRepositoryUrl =
      Uri.https('github.com', '/techouse/alfred-django-docs');
  static const String algoliaApplicationId = 'WODHKE4WZG';
  static const String algoliaSearchOnlyApiKey =
      '7456cdd91ba8d4f87846549697397759';
  static const String algoliaSearchIndex = 'docs';
  static const List<String> supportedVersions = [
    'v1.8',
    'v1.10',
    'v1.11',
    'v2',
    'v2.1',
    'v2.2',
    'v3',
    'v3.1',
    'v3.2',
    'v4',
  ];
}
