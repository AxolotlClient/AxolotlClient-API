# AxolotlClient API Documentation

The API is not currently used in production, however there is a development instance available at
`https://astralchroma.dev/axolotlclient-api/dev/`

## Data Types
- Nullable values are indicated with a `?`, example: `string?`. Fields that are set to null may be absent in responses.
- Arrays are indicated with `[]`, example: `[string]`.
- `Uuid`s are represented as strings, they may or may not be hyphenated.
- `Timestamp`s are represented as strings, they conform to the RFC 3339 format.

## Errors
Errors will return json content with the following:
- `status_code`: `number` - Http status Code
- `error_code`: `number` - Api error code, generally the same as the Http status code, but sometimes used to
                           differentiate errors that use the same Http status code on an endpoint.
- `description`: `string` - Human readable description of the error

If the endpoint is tagged with `Authenticated`, then the following errors are possible:
- HTTP `401` API `1000` - Access Token not provided
- HTTP `401` API `1001` - Access Token is corrupt
- HTTP `401` API `1002` - Access Token is expired or revoked

The following errors are always possible:
- HTTP `500` Internal Server Error

## Endpoints
### `GET` `/authenticate?<username>&<server_id>`
The client should first make a request to `https://sessionserver.mojang.com/session/minecraft/join` as outlined on
[wiki.vg](https://wiki.vg/Protocol_Encryption#Client) except with a secret random string `server_id` that is later given
to the server. This difference is because in Minecraft's protocol the `server_id` is derived from information exchanged
in order to set up encryption, this is unnecessary due to the use of Https. 

#### Query Fields
- `username`: `string`
- `server_id`: `string` - Server Id used to validate authentication with Mojang, this should be a secret random string

#### Response
HTTP `200` Ok
- `username`: `string`
- `uuid`: `Uuid` 
- `access_token`: `string` - Access Token used to authenticate future requests, this is valid for 24 hours from last
                             request, no guarantees are made as to the length or format.

#### Errors
- HTTP `401` Unauthorized

### `GET` `/user/<uuid>`
#### Path Fields
- `uuid`: `Uuid`

#### Response
HTTP `200` Ok
- `uuid`: `Uuid`
- `username`: `string`
- `registered`: `Timestamp?`
- `last_activity`: `Timestamp?`
- `old_usernames`: `[string]`

#### Errors
- HTTP `404` Not Found

### `GET` `/gateway` [Authenticated](#Errors)
See [Gateway](#gateway)

#### Response
HTTP `101` Switching Protocols - *Switch to WebSocket*

#### Errors
- HTTP `409` Conflict - A gateway connection is already open

### `GET` `/account` [Authenticated](#Errors)
#### Response
HTTP `200` Ok
- `uuid`: `Uuid`
- `username`: `string`
- `registered`: `Timestamp`
- `last_activity`: `Timestamp`
- `old_usernames`: `[OldUsername]`

#### OldUsername
- `username`: `string`
- `public`: `boolean`

### `DELETE` `/account` [Authenticated](#Errors)
Immediately and irrecoverably deletes the users account and associated data.

### `GET` `/account/data` [Authenticated](#Errors)
Returns user data in a Json format. Access tokens are not included.

#### Response
HTTP `204` No Content

### `GET` `/account/settings` [Authenticated](#Errors)
#### Response
HTTP `200` Ok
- `show_registered`: `boolean`
- `show_last_activity`: `boolean`
- `retain_usernames`: `boolean`

### `PATCH` `/account/settings` [Authenticated](#Errors)
#### Body Fields
- `show_registered`: `boolean`
- `show_last_activity`: `boolean`
- `retain_usernames`: `boolean`

#### Response
HTTP `204` No Content

### `POST` `/account/username/<username>?<public>` [Authenticated](#Errors)
#### Query Fields
- `public`: `boolean`

#### Response
HTTP `204` No Content

#### Errors
- HTTP `404` Not Found

### `DELETE` `/account/username/<username>` [Authenticated](#Errors)
#### Response
HTTP `204` No Content

### `GET` `POST` `/brew_coffee`
RFC 2324 joke. Serves no purpose.

#### Response
HTTP `418` I'm a teapot

## Gateway
Currently used so the server knows the client is online. Messages aren't actually sent, just a keep alive connection.

### Ping Pong
- Server will respond to any pings from client.
- Server will ping the client if there has been no communication for 10 seconds.
- Server will disconnect if there has been no communication for 10 seconds after the ping.
- The client does not *need* to respond with a pong, but it should, at a minimum it just needs to communicate.

### Closing Reasons
- `0` Closed
- `1` Internal Error
- `2` Invalid Data
- `3` Timed Out - See [Ping Pong](#ping-pong)
- `4` Unknown Error
