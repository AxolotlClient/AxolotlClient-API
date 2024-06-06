# AxolotlClient API Documentation

The API is not currently used in production, however there is a development instance available at
`https://astralchroma.dev/axolotlclient-api/dev/`

## Data Types

- Nullable values are indicated with a `?`, example: `string?`. Fields that are set to null may be absent in responses.
- Arrays are indicated with `[]`, example: `[string]`.
- `Uuid`s are represented as strings, they may or may not be hyphenated.
- `Timestamp`s are represented as strings, they conform to the RFC 3339 format.
- `Duration`s are seconds represented as numbers.

## Errors

Errors may or may not have a plain text reason in the body.

If the endpoint is tagged with `Authenticated`, then the following errors are possible:

- `403` Forbidden - Access Token is missing or corrupt
- `401` Unauthorized - Access Token is expired or revoked

The following errors are always possible:

- `500` Internal Server Error

The following errors are possible whenever body or query data is required:

- `400` Bad Request

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

`200` Ok

- `username`: `string`
- `uuid`: `Uuid`
- `access_token`: `string` - Access Token used to authenticate future requests, this is valid for 24 hours from last
  request, no guarantees are made as to the length or format.

#### Errors

- `401` Unauthorized

### `GET` `/gateway` [Authenticated](#Errors)

See [Gateway](#gateway)

#### Response

`101` Switching Protocols - *Switch to WebSocket*

#### Errors

- `409` Conflict - A gateway connection is already open

### `GET` `/user/<uuid>`

#### Path Fields

- `uuid`: `Uuid`

#### Response

`200` Ok

- `uuid`: `Uuid`
- `username`: `string`
- `registered`: `Timestamp?`
- `status`: `Status`
- `previous_usernames`: `[string]`

##### Status

- `type`: `string` - either: `online` or `offline`
- `last_online`: `Timestamp?` - only present if type is `offline` and if enabled by the user
- `activity`: `Activity?` - only present if type is `online` and if enabled by the user

##### Activity

- `title`: `string`
- `description`: `string`
- `started`: `Timestamp`

#### Errors

- `404` Not Found

### `POST` `/channel` [Authenticated](#Errors)

#### Body Parameters

- `name`: `string` - length between 1 and 32, not unique
- `persistence`: `Persistence`

##### Persistence

Controls when if ever messages should be automatically deleted. 4 options are provided:

- Channel = Delete messages when the channel is deleted
- Duration = Delete messages X time after they are sent
- Count = Delete all but the latest X messages
- Count and Duration = Delete all but the latest X messages X time after they are sent

The server may change this value, but only lower it, never increase it.

- `type`: `string` - either: `channel`, `duration`, `count`, or `count_and_duration`
- `count`: `number?` - only present if type is `count` or `count_and_duration`
- `duration`: `number?` - seconds, only present if type is `duration` or `count_and_duration`

#### Response

`200` Ok

Channel ID formatted as plain text

### `GET` `/account` [Authenticated](#Errors)

#### Response

`200` Ok

- `uuid`: `Uuid`
- `username`: `string`
- `registered`: `Timestamp`
- `last_online`: `Timestamp?`
- `previous_usernames`: `[OldUsername]`

#### OldUsername

- `username`: `string`
- `public`: `boolean`

### `DELETE` `/account` [Authenticated](#Errors)

Immediately and irrecoverably deletes the users account and associated data.

### `GET` `/account/data` [Authenticated](#Errors)

Returns user data in a Json format. Access tokens are not included.

#### Response

`204` No Content

### `GET` `/account/settings` [Authenticated](#Errors)

#### Response

`200` Ok

- `show_registered`: `boolean`
- `retain_usernames`: `boolean`
- `show_last_online`: `boolean`
- `show_activity`: `boolean`

### `PATCH` `/account/settings` [Authenticated](#Errors)

#### Body Fields

- `show_registered`: `boolean`
- `retain_usernames`: `boolean`
- `show_last_online`: `boolean`
- `show_activity`: `boolean`

#### Response

`204` No Content

### `POST` `/account/username/<username>?<public>` [Authenticated](#Errors)

#### Query Fields

- `public`: `boolean`

#### Response

`204` No Content

#### Errors

- `404` Not Found

### `DELETE` `/account/username/<username>` [Authenticated](#Errors)

#### Response

`204` No Content

### `GET` `POST` `/brew_coffee`

RFC 2324 joke. Serves no purpose.

#### Response

`418` I'm a teapot

## Gateway

Currently used so the server knows the client is online. Messages aren't actually sent, just a keep alive connection.

### Ping Pong

- Server will respond to any pings from client.
- Server will ping the client if there has been no communication for 10 seconds.
- Server will disconnect if there has been no communication for 10 seconds after the ping.
- The client does not *need* to respond with a pong, but it should, at a minimum it just needs to communicate.

### Closing Reasons

- `1000` Closed
- `1006` Timed Out - See [Ping Pong](#ping-pong)
- `1007` Invalid Data
- `1011` Error
