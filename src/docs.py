#!/usr/bin/python
# encoding: utf-8

from __future__ import print_function, unicode_literals, absolute_import

import functools
import re
import sys
from HTMLParser import HTMLParser

from algoliasearch.search_client import SearchClient
from config import Config
from workflow import Workflow, ICON_WARNING, ICON_INFO

# h.unescape() turns HTML escapes back into real characters
h = HTMLParser()

# Algolia client
client = SearchClient.create(Config.ALGOLIA_APP_ID, Config.ALGOLIA_SEARCH_ONLY_API_KEY)
index = client.init_index(Config.ALGOLIA_SEARCH_INDEX)

# log
log = None


def cache_key(query, version=Config.DEFAULT_DJANGO_VERSION):
    """Make filesystem-friendly cache key"""
    key = query + "_" + version
    key = key.lower()
    key = re.sub(r"[^a-z0-9-_;.]", "-", key)
    key = re.sub(r"-+", "-", key)
    log.debug("Cache key : {!r} {!r} -> {!r}".format(query, version, key))
    return key


def handle_result(api_dict):
    """Extract relevant info from API result"""
    result = {}

    for key in {"version", "id", "title", "permalink" "categories", "content"}:
        result[key] = h.unescape(api_dict[key])

    return result


def search(
    query=None, version=Config.DEFAULT_DJANGO_VERSION, limit=Config.RESULT_COUNT
):
    if query:
        results = index.search(
            query, {"filters": "version=" + version, "page": 0, "hitsPerPage": limit}
        )
        if results is not None and "hits" in results:
            return results["hits"]
    return []


def main(wf):
    if wf.update_available:
        # Add a notification to top of Script Filter results
        wf.add_item(
            "New version available",
            "Action this item to install the update",
            autocomplete="workflow:update",
            icon=ICON_INFO,
        )

    query = wf.args[0].strip()

    # Tag prefix only. Treat as blank query
    if query == "v":
        query = ""

    log.debug("query : {!r}".format(query))

    if not query:
        wf.add_item("Search the Django documentation")
        wf.send_feedback()
        return 0

    # Parse query into query string and tags
    words = query.split(" ")

    query = []
    version = Config.DEFAULT_DJANGO_VERSION

    for word in words:
        if word in Config.SUPPORTED_DJANGO_VERSIONS:
            version = word.replace("v", "")
        else:
            query.append(word)

    query = " ".join(query)

    key = cache_key(query, version)

    results = wf.cached_data(
        key, functools.partial(search, query, version), max_age=Config.CACHE_MAX_AGE
    )

    log.debug("{} results for {!r}, version {!r}".format(len(results), query, version))
    # Show results
    if not results:
        wf.add_item(
            "No matching answers found", "Try a different query", icon=ICON_WARNING
        )

    for result in results:
        if result["id"].startswith("django."):
            title = result["id"]
        elif result["id"].startswith("setting-"):
            title = "{} [setting]".format(result["title"])
        elif result["id"].startswith("templatefilter-"):
            title = "{} [template filter]".format(result["title"])
        elif result["id"].startswith("fieldlookup-"):
            title = "{} [field lookup]".format(result["title"])
        elif result["id"].startswith("templatetag-"):
            title = "{} [template tag]".format(result["title"])
        elif result["id"].startswith("cmdoption-"):
            title = "{} [cmd option]".format(result["title"])
        elif result["id"].startswith("envvar-"):
            title = "{} [env var]".format(result["title"])
        else:
            title = result["title"]

        wf.add_item(
            uid=result["id"],
            title=title,
            subtitle=result["content"] if len(result["content"]) else result["id"],
            arg=result["permalink"],
            valid=True,
            largetext=result["id"],
            copytext=result["id"],
            quicklookurl=result["permalink"],
            icon=Config.DJANGO_ICON,
        )
        # log.debug(result)

    wf.send_feedback()


if __name__ == "__main__":
    wf = Workflow(
        update_settings={"github_slug": "techouse/alfred-django-docs", "frequency": 7}
    )
    log = wf.logger
    sys.exit(wf.run(main))
