# Django Docs Workflow for Alfred

![GitHub release](https://img.shields.io/github/release/techouse/alfred-django-docs.svg)
![GitHub All Releases](https://img.shields.io/github/downloads/techouse/alfred-django-docs/total.svg)
![GitHub](https://img.shields.io/github/license/techouse/alfred-django-docs.svg)

Search the [Django documentation](https://docs.djangoproject.com/en/2.1/) using [Alfred](https://www.alfredapp.com/). 

![demo](demo.gif)

## Installation

1. [Download the latest version](https://github.com/techouse/alfred-django-docs/releases/latest)
2. Install the workflow by double-clicking the `.alfredworkflow` file
3. You can add the workflow to a category, then click "Import" to finish importing. You'll now see the workflow listed in the left sidebar of your Workflows preferences pane.

## Usage

Just type `dj` followed by your search query.

```
dj FormView
```

Either press `⌘Y` to Quick Look the result, or press `<enter>` to open it in your web browser.

## Changing Branches

The workflow supports searching the documentation of all the currently officially supported branches `2.2`, `2.1` and `1.11`. 
By default it searches the `2.2` branch. To search branch `2.1` or `1.11` simply type `v2.1` or `v1.11` anywhere in your query, like so:

```
dj v1.11 as_p
```

### Note

Built using [Alfred-Workflow](https://github.com/deanishe/alfred-workflow).
The lightning fast search is powered by [Algolia](https://www.algolia.com) which was generous enough to hand me a big 
enough plan to fit all the indices for the officially supported Django documentation versions.
A big thank you to [@redox](https://github.com/redox) from [@algolia](https://github.com/algolia) :innocent: :beers: :heart:

The index for Algolia was compiled from the zipped Django HTML documentation using a [simple Python script](https://github.com/techouse/django-docs-parser) I wrote.