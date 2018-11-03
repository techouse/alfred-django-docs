# Django Docs Workflow for Alfred

Search the [Django documentation](https://docs.djangoproject.com/en/2.1/) using [Alfred](https://www.alfredapp.com/). 

![demo](demo.gif)

## Installation

1. [Download the latest version](https://github.com/techouse/alfred-django-docs/releases)
2. Install the workflow by double-clicking the .alfredworkflow file
3. You can add the workflow to a category, then click "Import" to finish importing. You'll now see the workflow listed in the left sidebar of your Workflows preferences pane.

## Usage

Just type `dj` followed by your search query.

```
dj FormView
```

Either press `⌘Y` to Quick Look the result, or press `<enter>` to open it in your web browser.

## Changing Branches

The workflow supports searching the documentation of the branches `2.1` and `1.11`. 
By default it searches the `2.1` branch. To search branch `1.11` simply type `v1.11` anywhere in your query, like so:

```
dj v1.11 as_p
```

### Note

The search powered by [Algolia search](https://www.algolia.com) using a __free Community plan__.
Since the community plan only supports __10,000__ records I had to cut it short and support only 2 branches of the documentation.
That is the reason why there is no documentation for the `2.0` branch.

The index for Algolia was compiled from the zipped Django HTML documentation using a [simple Python script](https://github.com/techouse/django-docs-parser) I wrote.