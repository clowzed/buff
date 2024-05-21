# \CratehandlersstatususersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_status**](CratehandlersstatususersApi.md#fetch_status) | **GET** /api/status/user | 
[**refresh_status**](CratehandlersstatususersApi.md#refresh_status) | **PATCH** /api/status/user | 



## fetch_status

> Vec<models::StatusResponse> fetch_status(status_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_request** | [**StatusRequest**](StatusRequest.md) |  | [required] |

### Return type

[**Vec<models::StatusResponse>**](StatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_status

> refresh_status()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

