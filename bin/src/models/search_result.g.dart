// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'search_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SearchResult _$SearchResultFromJson(Map<String, dynamic> json) => SearchResult(
      objectID: json['objectID'] as String,
      categories: (json['categories'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      content: json['content'] as String,
      id: json['id'] as String,
      permalink: json['permalink'] as String,
      title: json['title'] as String,
      version: json['version'] as num,
    );

Map<String, dynamic> _$SearchResultToJson(SearchResult instance) =>
    <String, dynamic>{
      'objectID': instance.objectID,
      'categories': instance.categories,
      'content': instance.content,
      'id': instance.id,
      'permalink': instance.permalink,
      'title': instance.title,
      'version': instance.version,
    };
