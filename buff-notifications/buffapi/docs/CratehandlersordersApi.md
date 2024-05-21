# \CratehandlersordersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**all_in_period**](CratehandlersordersApi.md#all_in_period) | **POST** /api/user/order/all-in-period | 
[**cancel_order**](CratehandlersordersApi.md#cancel_order) | **PATCH** /api/user/order/{id}/cancel | 
[**create_order**](CratehandlersordersApi.md#create_order) | **POST** /api/user/order | 
[**get_order**](CratehandlersordersApi.md#get_order) | **GET** /api/user/order/{id} | 
[**list_orders**](CratehandlersordersApi.md#list_orders) | **GET** /api/user/order | 
[**set_order_maybepayed**](CratehandlersordersApi.md#set_order_maybepayed) | **PATCH** /api/user/order/{id}/maybepayed | 



## all_in_period

> Vec<models::Order> all_in_period(time_bounds)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_bounds** | [**TimeBounds**](TimeBounds.md) |  | [required] |

### Return type

[**Vec<models::Order>**](Order.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_order

> cancel_order(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Order id | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order

> models::Order create_order(create_order_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_request** | [**CreateOrderRequest**](CreateOrderRequest.md) |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> models::Order get_order(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Order id | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_orders

> Vec<models::Order> list_orders()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Order>**](Order.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_order_maybepayed

> set_order_maybepayed(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Order id | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

