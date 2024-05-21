# \CratehandlersadminusersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**registrations_in_period**](CratehandlersadminusersApi.md#registrations_in_period) | **GET** /api/admin/users/registrations-in-period | 



## registrations_in_period

> models::RegistrationsInPeriodResponse registrations_in_period(start_datetime, end_datetime)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_datetime** | **String** |  | [required] |
**end_datetime** | **String** |  | [required] |

### Return type

[**models::RegistrationsInPeriodResponse**](RegistrationsInPeriodResponse.md)

### Authorization

[jwt_admin](../README.md#jwt_admin)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

