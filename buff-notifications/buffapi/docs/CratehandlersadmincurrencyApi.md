# \CratehandlersadmincurrencyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_currency**](CratehandlersadmincurrencyApi.md#create_currency) | **POST** /api/admin/currency | 
[**delete_currency_rate_by_id**](CratehandlersadmincurrencyApi.md#delete_currency_rate_by_id) | **DELETE** /api/admin/currency/{id} | 
[**set_currency_rate_by_id**](CratehandlersadmincurrencyApi.md#set_currency_rate_by_id) | **PATCH** /api/admin/currency/{id} | 



## create_currency

> models::Order create_currency(create_currency_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_currency_request** | [**CreateCurrencyRequest**](CreateCurrencyRequest.md) |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_currency_rate_by_id

> delete_currency_rate_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Currency rate id | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_currency_rate_by_id

> set_currency_rate_by_id(id, set_rate_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Currency rate id | [required] |
**set_rate_request** | [**SetRateRequest**](SetRateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

