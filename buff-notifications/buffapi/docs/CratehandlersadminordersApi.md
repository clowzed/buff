# \CratehandlersadminordersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_in_period**](CratehandlersadminordersApi.md#all_in_period) | **POST** /api/admin/order/all-in-period | 
[**cancel_order_by_id**](CratehandlersadminordersApi.md#cancel_order_by_id) | **PATCH** /api/admin/order/{id}/cancel | 
[**finish_order_by_id**](CratehandlersadminordersApi.md#finish_order_by_id) | **PATCH** /api/admin/order/{id}/success | 



## all_in_period

> Vec<models::Order> all_in_period(time_bounds)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_bounds** | [**TimeBounds**](TimeBounds.md) |  | [required] |

### Return type

[**Vec<models::Order>**](Order.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_order_by_id

> cancel_order_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Order id | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## finish_order_by_id

> finish_order_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Order id | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

