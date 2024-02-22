# AxolotlClient API Documentation

The API is not currently used in production, however there is a development instance available at
`https://astralchroma.dev/axolotlclient-api/dev/`

## Errors
Errors will return json content with the following:
- `status_code`: `number` - Http status Code
- `error_code`: `number` - Api error code
- `description`: `string` - Human readable description of the error

If the endpoint is tagged with `Authenticated`, then the following errors are possible
- HTTP `401` API `1001` - Access Token not provided
- HTTP `401` API `1002` - Access Token is corrupt
- HTTP `401` API `1003` - Access Token is expired or revoked

## Endpoints
### `GET` `/authenticate`
The client should first make a request to `https://sessionserver.mojang.com/session/minecraft/join` as outlined on
[wiki.vg](https://wiki.vg/Protocol_Encryption#Client) except with a secret random string `server_id` that is later given
to the server. This difference is because in Minecraft's protocol the `server_id` is derived from information exchanged
in order to set up encryption, this is unnecessary due to the use of Https. 

#### Query Parameters
- `username` - Username of the authenticating player
- `server_id` - Server Id used to validate authentication with Mojang, this should be a secret random string

#### Response
- `username`: `string` - Username as returned by Mojang's Api
- `uuid`: `string` - Hyphenated Minecraft Player Uuid 
- `access_token`: `string` - Access Token used to authenticate future requests, this is valid for 24 hours from last
request, no guarantees are made as to the length or format.

#### Errors
- HTTP `401` API `1000` - Authentication Failed

### `GET` `/user/<uuid>`
#### Path Parameters
- `uuid` - Minecraft Uuid of user

#### Response
- `uuid`: `string`
- `username`: `string`
- `registered`: `string or null` - RFC 3339 Format, null if hidden by user
- `last_activity`: `string or null` - RFC 3339 Format, null if hidden by user

#### Errors
- HTTP `404` API `1100` - User is not registered

### `DELETE` `/account` [Authenticated](#Errors)
Immediately and irrecoverably deletes the users account and associated data.

#### Response
HTTP `204` - No Content

### `GET` `/account/user` [Authenticated](#Errors)
#### Response
- `uuid`: `string`
- `username`: `string`
- `registered`: `string` - RFC 3339 Format
- `last_activity`: `string` - RFC 3339 Format

### `GET` `/account/settings` [Authenticated](#Errors)
#### Response
- `show_registered`: `boolean`
- `show_last_activity`: `boolean`
- `retain_usernames`: `boolean`

### `PATCH` `/account/settings` [Authenticated](#Errors)
#### Fields
- `show_registered`: `boolean`
- `show_last_activity`: `boolean`
- `retain_usernames`: `boolean`

#### Response
HTTP `204` - No Content

### `GET` `/account/data` [Authenticated](#Errors)
Returns user data in a Json format. Access tokens are not included. 

### `GET` `POST` `/brew_coffee`
RFC 2324 joke. Serves no purpose.

#### Errors
- HTTP `418` API `418` - I'm a teapot