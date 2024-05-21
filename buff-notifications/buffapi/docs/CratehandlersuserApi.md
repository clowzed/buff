# \CratehandlersuserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**avatar**](CratehandlersuserApi.md#avatar) | **GET** /api/user/avatar/{id} | 
[**chat**](CratehandlersuserApi.md#chat) | **PATCH** /api/user/chat | 
[**get_top**](CratehandlersuserApi.md#get_top) | **GET** /api/user/top | 
[**get_user**](CratehandlersuserApi.md#get_user) | **GET** /api/user | 
[**history**](CratehandlersuserApi.md#history) | **GET** /api/user/chat/{id}/history | 
[**send_message**](CratehandlersuserApi.md#send_message) | **POST** /api/user/chat/{id}/message | 
[**set_email**](CratehandlersuserApi.md#set_email) | **PATCH** /api/user/email | 
[**set_trade_url**](CratehandlersuserApi.md#set_trade_url) | **PATCH** /api/user/trade-url | 
[**username**](CratehandlersuserApi.md#username) | **GET** /api/user/username/{id} | 



## avatar

> String avatar(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | SteamId | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat

> models::ChatResponse chat(get_chat_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_chat_request** | [**GetChatRequest**](GetChatRequest.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top

> Vec<models::TopUser> get_top(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i64** |  | [required] |
**offset** | **i64** |  | [required] |

### Return type

[**Vec<models::TopUser>**](TopUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::User get_user()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history

> models::ChatHistory history(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Chat id | [required] |

### Return type

[**models::ChatHistory**](ChatHistory.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_message

> models::SendMessageResponse send_message(id, upload_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Chat id | [required] |
**upload_data** | [**UploadData**](UploadData.md) |  | [required] |

### Return type

[**models::SendMessageResponse**](SendMessageResponse.md)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_email

> set_email(email_form)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_form** | [**EmailForm**](EmailForm.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_trade_url

> set_trade_url(trade_url_form)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trade_url_form** | [**TradeUrlForm**](TradeUrlForm.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_user](../README.md#jwt_user)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## username

> String username(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | SteamId | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

