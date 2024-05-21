# \CratehandlerscurrencyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_currency_rate_by_id**](CratehandlerscurrencyApi.md#get_currency_rate_by_id) | **GET** /api/currency/{id} | 
[**get_currency_rates**](CratehandlerscurrencyApi.md#get_currency_rates) | **GET** /api/currency | 



## get_currency_rate_by_id

> models::Currency get_currency_rate_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Currency rate id | [required] |

### Return type

[**models::Currency**](Currency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_currency_rates

> Vec<models::Currency> get_currency_rates()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Currency>**](Currency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

