# \CratehandlersadminmoderatorsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_moderator**](CratehandlersadminmoderatorsApi.md#assign_moderator) | **PATCH** /api/admin/moderator/assign | 
[**change_password**](CratehandlersadminmoderatorsApi.md#change_password) | **PATCH** /api/admin/moderator/password | 
[**chat**](CratehandlersadminmoderatorsApi.md#chat) | **PATCH** /api/admin/moderator/chat | 
[**chat_history_admin**](CratehandlersadminmoderatorsApi.md#chat_history_admin) | **GET** /api/admin/moderator/order/{id}/chat/history | 
[**create_moderator**](CratehandlersadminmoderatorsApi.md#create_moderator) | **POST** /api/admin/moderator | 
[**delete_moderator**](CratehandlersadminmoderatorsApi.md#delete_moderator) | **DELETE** /api/admin/moderator/{id} | 
[**history**](CratehandlersadminmoderatorsApi.md#history) | **GET** /api/admin/moderator/chat/{id}/history | 
[**image**](CratehandlersadminmoderatorsApi.md#image) | **GET** /api/user/chat/{id}/image/{id} | 
[**list_moderators**](CratehandlersadminmoderatorsApi.md#list_moderators) | **GET** /api/admin/moderator | 
[**list_moderators_orders**](CratehandlersadminmoderatorsApi.md#list_moderators_orders) | **GET** /api/admin/moderator/orders | 
[**list_unassigned_orders**](CratehandlersadminmoderatorsApi.md#list_unassigned_orders) | **GET** /api/admin/moderator/unassigned-orders | 
[**self_info**](CratehandlersadminmoderatorsApi.md#self_info) | **GET** /api/admin/self | 
[**send_message**](CratehandlersadminmoderatorsApi.md#send_message) | **POST** /api/admin/moderator/chat/{id}/message | 
[**unassign_moderator**](CratehandlersadminmoderatorsApi.md#unassign_moderator) | **PATCH** /api/admin/moderator/unassign | 



## assign_moderator

> assign_moderator(assign_moderator_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assign_moderator_request** | [**AssignModeratorRequest**](AssignModeratorRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_password

> change_password(change_password_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_request** | [**ChangePasswordRequest**](ChangePasswordRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_history_admin

> models::ChatHistory chat_history_admin(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Order id | [required] |

### Return type

[**models::ChatHistory**](ChatHistory.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_moderator

> models::ModeratorResponse create_moderator(moderator_credentials)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moderator_credentials** | [**ModeratorCredentials**](ModeratorCredentials.md) |  | [required] |

### Return type

[**models::ModeratorResponse**](ModeratorResponse.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_moderator

> delete_moderator(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Moderator id | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

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

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image

> image(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Chat id and image id | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_moderators

> Vec<models::ModeratorResponse> list_moderators()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ModeratorResponse>**](ModeratorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_moderators_orders

> Vec<models::Order> list_moderators_orders()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Order>**](Order.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_unassigned_orders

> Vec<models::Order> list_unassigned_orders()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Order>**](Order.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## self_info

> models::ModeratorOrAdminInfo self_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModeratorOrAdminInfo**](ModeratorOrAdminInfo.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

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

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_moderator

> unassign_moderator(unassign_moderator_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unassign_moderator_request** | [**UnassignModeratorRequest**](UnassignModeratorRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

