# \CratehandlersauthusersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**login**](CratehandlersauthusersApi.md#login) | **GET** /api/auth/user/callback | 
[**login_link**](CratehandlersauthusersApi.md#login_link) | **GET** /api/auth/user/link | 



## login

> models::JwtResponse login(openid_period_ns, openid_period_mode, openid_period_op_endpoint, openid_period_claimed_id, openid_period_return_to, openid_period_response_nonce, openid_period_assoc_handle, openid_period_signed, openid_period_sig, openid_period_identity, openid_period_invalidate_handle)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**openid_period_ns** | **String** |  | [required] |
**openid_period_mode** | **String** |  | [required] |
**openid_period_op_endpoint** | **String** |  | [required] |
**openid_period_claimed_id** | **String** |  | [required] |
**openid_period_return_to** | **String** |  | [required] |
**openid_period_response_nonce** | **String** |  | [required] |
**openid_period_assoc_handle** | **String** |  | [required] |
**openid_period_signed** | **String** |  | [required] |
**openid_period_sig** | **String** |  | [required] |
**openid_period_identity** | Option<**String**> |  |  |
**openid_period_invalidate_handle** | Option<**String**> |  |  |

### Return type

[**models::JwtResponse**](JwtResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_link

> models::LoginLinkResponse login_link()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LoginLinkResponse**](LoginLinkResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

