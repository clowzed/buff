# \CratehandlersadminreviewsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_video_review**](CratehandlersadminreviewsApi.md#add_video_review) | **POST** /api/admin/review/video | 
[**remove_review**](CratehandlersadminreviewsApi.md#remove_review) | **DELETE** /api/admin/review | 
[**remove_video_review**](CratehandlersadminreviewsApi.md#remove_video_review) | **DELETE** /api/admin/review/video | 
[**update_video_review**](CratehandlersadminreviewsApi.md#update_video_review) | **PATCH** /api/admin/review/video | 



## add_video_review

> add_video_review(add_video_review_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_video_review_request** | [**AddVideoReviewRequest**](AddVideoReviewRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_review

> remove_review(remove_review_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_review_request** | [**RemoveReviewRequest**](RemoveReviewRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_video_review

> remove_video_review(remove_video_review_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_video_review_request** | [**RemoveVideoReviewRequest**](RemoveVideoReviewRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_video_review

> update_video_review(update_video_review_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_video_review_request** | [**UpdateVideoReviewRequest**](UpdateVideoReviewRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

