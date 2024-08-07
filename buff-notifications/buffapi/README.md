# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1.0
- Package version: 0.1.0
- Generator version: 7.5.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CratehandlersadminblacklistApi* | [**blacklist_user**](docs/CratehandlersadminblacklistApi.md#blacklist_user) | **POST** /api/admin/blacklist | 
*CratehandlersadminblacklistApi* | [**full_blacklist**](docs/CratehandlersadminblacklistApi.md#full_blacklist) | **GET** /api/admin/blacklist | 
*CratehandlersadminblacklistApi* | [**unblacklist_user**](docs/CratehandlersadminblacklistApi.md#unblacklist_user) | **DELETE** /api/admin/blacklist | 
*CratehandlersadmincurrencyApi* | [**create_currency**](docs/CratehandlersadmincurrencyApi.md#create_currency) | **POST** /api/admin/currency | 
*CratehandlersadmincurrencyApi* | [**delete_currency_rate_by_id**](docs/CratehandlersadmincurrencyApi.md#delete_currency_rate_by_id) | **DELETE** /api/admin/currency/{id} | 
*CratehandlersadmincurrencyApi* | [**set_currency_rate_by_id**](docs/CratehandlersadmincurrencyApi.md#set_currency_rate_by_id) | **PATCH** /api/admin/currency/{id} | 
*CratehandlersadminmoderatorsApi* | [**assign_moderator**](docs/CratehandlersadminmoderatorsApi.md#assign_moderator) | **PATCH** /api/admin/moderator/assign | 
*CratehandlersadminmoderatorsApi* | [**change_password**](docs/CratehandlersadminmoderatorsApi.md#change_password) | **PATCH** /api/admin/moderator/password | 
*CratehandlersadminmoderatorsApi* | [**chat**](docs/CratehandlersadminmoderatorsApi.md#chat) | **PATCH** /api/admin/moderator/chat | 
*CratehandlersadminmoderatorsApi* | [**chat_history_admin**](docs/CratehandlersadminmoderatorsApi.md#chat_history_admin) | **GET** /api/admin/moderator/order/{id}/chat/history | 
*CratehandlersadminmoderatorsApi* | [**create_moderator**](docs/CratehandlersadminmoderatorsApi.md#create_moderator) | **POST** /api/admin/moderator | 
*CratehandlersadminmoderatorsApi* | [**delete_moderator**](docs/CratehandlersadminmoderatorsApi.md#delete_moderator) | **DELETE** /api/admin/moderator/{id} | 
*CratehandlersadminmoderatorsApi* | [**history**](docs/CratehandlersadminmoderatorsApi.md#history) | **GET** /api/admin/moderator/chat/{id}/history | 
*CratehandlersadminmoderatorsApi* | [**image**](docs/CratehandlersadminmoderatorsApi.md#image) | **GET** /api/user/chat/{id}/image/{id} | 
*CratehandlersadminmoderatorsApi* | [**list_moderators**](docs/CratehandlersadminmoderatorsApi.md#list_moderators) | **GET** /api/admin/moderator | 
*CratehandlersadminmoderatorsApi* | [**list_moderators_orders**](docs/CratehandlersadminmoderatorsApi.md#list_moderators_orders) | **GET** /api/admin/moderator/orders | 
*CratehandlersadminmoderatorsApi* | [**list_unassigned_orders**](docs/CratehandlersadminmoderatorsApi.md#list_unassigned_orders) | **GET** /api/admin/moderator/unassigned-orders | 
*CratehandlersadminmoderatorsApi* | [**self_info**](docs/CratehandlersadminmoderatorsApi.md#self_info) | **GET** /api/admin/self | 
*CratehandlersadminmoderatorsApi* | [**send_message**](docs/CratehandlersadminmoderatorsApi.md#send_message) | **POST** /api/admin/moderator/chat/{id}/message | 
*CratehandlersadminmoderatorsApi* | [**unassign_moderator**](docs/CratehandlersadminmoderatorsApi.md#unassign_moderator) | **PATCH** /api/admin/moderator/unassign | 
*CratehandlersadminordersApi* | [**all_in_period**](docs/CratehandlersadminordersApi.md#all_in_period) | **POST** /api/admin/order/all-in-period | 
*CratehandlersadminordersApi* | [**cancel_order_by_id**](docs/CratehandlersadminordersApi.md#cancel_order_by_id) | **PATCH** /api/admin/order/{id}/cancel | 
*CratehandlersadminordersApi* | [**finish_order_by_id**](docs/CratehandlersadminordersApi.md#finish_order_by_id) | **PATCH** /api/admin/order/{id}/success | 
*CratehandlersadminrequisitesApi* | [**set_data**](docs/CratehandlersadminrequisitesApi.md#set_data) | **PATCH** /api/admin/requisites | 
*CratehandlersadminreviewsApi* | [**add_video_review**](docs/CratehandlersadminreviewsApi.md#add_video_review) | **POST** /api/admin/review/video | 
*CratehandlersadminreviewsApi* | [**remove_review**](docs/CratehandlersadminreviewsApi.md#remove_review) | **DELETE** /api/admin/review | 
*CratehandlersadminreviewsApi* | [**remove_video_review**](docs/CratehandlersadminreviewsApi.md#remove_video_review) | **DELETE** /api/admin/review/video | 
*CratehandlersadminreviewsApi* | [**update_video_review**](docs/CratehandlersadminreviewsApi.md#update_video_review) | **PATCH** /api/admin/review/video | 
*CratehandlersadminsocialApi* | [**set_url**](docs/CratehandlersadminsocialApi.md#set_url) | **PATCH** /api/admin/social | 
*CratehandlersadminusersApi* | [**registrations_in_period**](docs/CratehandlersadminusersApi.md#registrations_in_period) | **GET** /api/admin/users/registrations-in-period | 
*CratehandlersauthadminsApi* | [**login**](docs/CratehandlersauthadminsApi.md#login) | **POST** /api/auth/admin/login | 
*CratehandlersauthusersApi* | [**login**](docs/CratehandlersauthusersApi.md#login) | **GET** /api/auth/user/callback | 
*CratehandlersauthusersApi* | [**login_link**](docs/CratehandlersauthusersApi.md#login_link) | **GET** /api/auth/user/link | 
*CratehandlerscurrencyApi* | [**get_currency_rate_by_id**](docs/CratehandlerscurrencyApi.md#get_currency_rate_by_id) | **GET** /api/currency/{id} | 
*CratehandlerscurrencyApi* | [**get_currency_rates**](docs/CratehandlerscurrencyApi.md#get_currency_rates) | **GET** /api/currency | 
*CratehandlersordersApi* | [**all_in_period**](docs/CratehandlersordersApi.md#all_in_period) | **POST** /api/user/order/all-in-period | 
*CratehandlersordersApi* | [**cancel_order**](docs/CratehandlersordersApi.md#cancel_order) | **PATCH** /api/user/order/{id}/cancel | 
*CratehandlersordersApi* | [**create_order**](docs/CratehandlersordersApi.md#create_order) | **POST** /api/user/order | 
*CratehandlersordersApi* | [**get_order**](docs/CratehandlersordersApi.md#get_order) | **GET** /api/user/order/{id} | 
*CratehandlersordersApi* | [**list_orders**](docs/CratehandlersordersApi.md#list_orders) | **GET** /api/user/order | 
*CratehandlersordersApi* | [**set_order_maybepayed**](docs/CratehandlersordersApi.md#set_order_maybepayed) | **PATCH** /api/user/order/{id}/maybepayed | 
*CratehandlersrequisitesApi* | [**requisites**](docs/CratehandlersrequisitesApi.md#requisites) | **GET** /api/requisites | 
*CratehandlersreviewsusersApi* | [**add_users_review**](docs/CratehandlersreviewsusersApi.md#add_users_review) | **POST** /api/review | 
*CratehandlersreviewsusersApi* | [**all_users_reviews**](docs/CratehandlersreviewsusersApi.md#all_users_reviews) | **GET** /api/review | 
*CratehandlersreviewsusersApi* | [**all_video_reviews**](docs/CratehandlersreviewsusersApi.md#all_video_reviews) | **GET** /api/review/video | 
*CratehandlersreviewsusersApi* | [**count_reviews**](docs/CratehandlersreviewsusersApi.md#count_reviews) | **GET** /api/review/count | 
*CratehandlersreviewsusersApi* | [**five_stars**](docs/CratehandlersreviewsusersApi.md#five_stars) | **GET** /api/review/five-stars | 
*CratehandlerssocialApi* | [**socials**](docs/CratehandlerssocialApi.md#socials) | **GET** /api/socials | 
*CratehandlersstatususersApi* | [**fetch_status**](docs/CratehandlersstatususersApi.md#fetch_status) | **GET** /api/status/user | 
*CratehandlersstatususersApi* | [**refresh_status**](docs/CratehandlersstatususersApi.md#refresh_status) | **PATCH** /api/status/user | 
*CratehandlersuserApi* | [**avatar**](docs/CratehandlersuserApi.md#avatar) | **GET** /api/user/avatar/{id} | 
*CratehandlersuserApi* | [**chat**](docs/CratehandlersuserApi.md#chat) | **PATCH** /api/user/chat | 
*CratehandlersuserApi* | [**get_top**](docs/CratehandlersuserApi.md#get_top) | **GET** /api/user/top | 
*CratehandlersuserApi* | [**get_user**](docs/CratehandlersuserApi.md#get_user) | **GET** /api/user | 
*CratehandlersuserApi* | [**history**](docs/CratehandlersuserApi.md#history) | **GET** /api/user/chat/{id}/history | 
*CratehandlersuserApi* | [**send_message**](docs/CratehandlersuserApi.md#send_message) | **POST** /api/user/chat/{id}/message | 
*CratehandlersuserApi* | [**set_email**](docs/CratehandlersuserApi.md#set_email) | **PATCH** /api/user/email | 
*CratehandlersuserApi* | [**set_trade_url**](docs/CratehandlersuserApi.md#set_trade_url) | **PATCH** /api/user/trade-url | 
*CratehandlersuserApi* | [**username**](docs/CratehandlersuserApi.md#username) | **GET** /api/user/username/{id} | 


## Documentation For Models

 - [AddReviewRequest](docs/AddReviewRequest.md)
 - [AddVideoReviewRequest](docs/AddVideoReviewRequest.md)
 - [AdminLoginResponse](docs/AdminLoginResponse.md)
 - [AssignModeratorRequest](docs/AssignModeratorRequest.md)
 - [BlacklistUserRequest](docs/BlacklistUserRequest.md)
 - [Bounds](docs/Bounds.md)
 - [ChangePasswordRequest](docs/ChangePasswordRequest.md)
 - [ChatHistory](docs/ChatHistory.md)
 - [ChatHistoryMessagesInnerInner](docs/ChatHistoryMessagesInnerInner.md)
 - [ChatResponse](docs/ChatResponse.md)
 - [CreateCurrencyRequest](docs/CreateCurrencyRequest.md)
 - [CreateOrderRequest](docs/CreateOrderRequest.md)
 - [Credentials](docs/Credentials.md)
 - [Currency](docs/Currency.md)
 - [Details](docs/Details.md)
 - [EmailForm](docs/EmailForm.md)
 - [GetChatRequest](docs/GetChatRequest.md)
 - [JwtResponse](docs/JwtResponse.md)
 - [LoginLinkResponse](docs/LoginLinkResponse.md)
 - [Message](docs/Message.md)
 - [ModeratorCredentials](docs/ModeratorCredentials.md)
 - [ModeratorOrAdminInfo](docs/ModeratorOrAdminInfo.md)
 - [ModeratorResponse](docs/ModeratorResponse.md)
 - [Order](docs/Order.md)
 - [RegistrationsInPeriodResponse](docs/RegistrationsInPeriodResponse.md)
 - [RemoveReviewRequest](docs/RemoveReviewRequest.md)
 - [RemoveVideoReviewRequest](docs/RemoveVideoReviewRequest.md)
 - [Requisites](docs/Requisites.md)
 - [Review](docs/Review.md)
 - [ReviewCountResponse](docs/ReviewCountResponse.md)
 - [SendMessageResponse](docs/SendMessageResponse.md)
 - [SetRateRequest](docs/SetRateRequest.md)
 - [SetRequisitesDataRequest](docs/SetRequisitesDataRequest.md)
 - [SetSocialUrlRequest](docs/SetSocialUrlRequest.md)
 - [Social](docs/Social.md)
 - [StatusRequest](docs/StatusRequest.md)
 - [StatusResponse](docs/StatusResponse.md)
 - [TimeBounds](docs/TimeBounds.md)
 - [TopUser](docs/TopUser.md)
 - [TradeUrlForm](docs/TradeUrlForm.md)
 - [UnassignModeratorRequest](docs/UnassignModeratorRequest.md)
 - [UnblacklistUserRequest](docs/UnblacklistUserRequest.md)
 - [UpdateVideoReviewRequest](docs/UpdateVideoReviewRequest.md)
 - [UploadData](docs/UploadData.md)
 - [User](docs/User.md)
 - [UserStatus](docs/UserStatus.md)
 - [VerifyForm](docs/VerifyForm.md)
 - [VideoReview](docs/VideoReview.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



