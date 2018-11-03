# encoding: utf-8


class Config(object):
    # Number of results to fetch from API
    RESULT_COUNT = 20
    # How long to cache results for
    CACHE_MAX_AGE = 20  # seconds
    # Icon
    DJANGO_ICON = 'icon.png'
    # supported docs
    SUPPORTED_DJANGO_VERSIONS = {'v2.1', 'v1.11'}
    DEFAULT_DJANGO_VERSION = '2.1'
    # Algolia credentials
    ALGOLIA_APP_ID = 'WODHKE4WZG'
    ALGOLIA_SEARCH_ONLY_API_KEY = '7456cdd91ba8d4f87846549697397759'
    ALGOLIA_SEARCH_INDEX = 'docs'
