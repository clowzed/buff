# \CratehandlersadminblacklistApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**blacklist_user**](CratehandlersadminblacklistApi.md#blacklist_user) | **POST** /api/admin/blacklist | 
[**full_blacklist**](CratehandlersadminblacklistApi.md#full_blacklist) | **GET** /api/admin/blacklist | 
[**unblacklist_user**](CratehandlersadminblacklistApi.md#unblacklist_user) | **DELETE** /api/admin/blacklist | 



## blacklist_user

> blacklist_user(blacklist_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blacklist_user_request** | [**BlacklistUserRequest**](BlacklistUserRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## full_blacklist

> Vec<String> full_blacklist()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unblacklist_user

> unblacklist_user(unblacklist_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unblacklist_user_request** | [**UnblacklistUserRequest**](UnblacklistUserRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

