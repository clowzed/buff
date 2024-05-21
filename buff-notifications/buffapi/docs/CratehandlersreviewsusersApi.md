# \CratehandlersreviewsusersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_users_review**](CratehandlersreviewsusersApi.md#add_users_review) | **POST** /api/review | 
[**all_users_reviews**](CratehandlersreviewsusersApi.md#all_users_reviews) | **GET** /api/review | 
[**all_video_reviews**](CratehandlersreviewsusersApi.md#all_video_reviews) | **GET** /api/review/video | 
[**count_reviews**](CratehandlersreviewsusersApi.md#count_reviews) | **GET** /api/review/count | 
[**five_stars**](CratehandlersreviewsusersApi.md#five_stars) | **GET** /api/review/five-stars | 



## add_users_review

> add_users_review(add_review_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_review_request** | [**AddReviewRequest**](AddReviewRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_users_reviews

> Vec<models::Review> all_users_reviews(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **String** |  | [required] |
**offset** | **String** |  | [required] |

### Return type

[**Vec<models::Review>**](Review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_video_reviews

> Vec<models::VideoReview> all_video_reviews()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::VideoReview>**](VideoReview.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## count_reviews

> models::ReviewCountResponse count_reviews()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReviewCountResponse**](ReviewCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## five_stars

> Vec<models::Review> five_stars()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Review>**](Review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

