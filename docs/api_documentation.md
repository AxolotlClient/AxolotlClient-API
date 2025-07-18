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

- `401` Unauthorized - Access Token is corrupt, expired, missing, or revoked

The following errors are always possible:

- `500` Internal Server Error

The following errors are possible whenever body or query data is required:

- `400` Bad Request - Data is malformed, may have additional possible meanings dependent on endpoint

## Endpoints

### `GET` `/global_data`

Get global data about the backend and mod.

#### Response

- `total_players`: `number` - Total number of players known to the backend. Updated every other minute.
- `ontline_players`: `number` - Number of currently online players. Updated every other minute.
- `latest_version`: `string` - The latest version of the mod, does not include a game version. Cached for 1 day, fetched from modrinth.
- `notes`: `string?` - Misc notes, f.e. updates, maintenance notices, ...

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

`101` Switching Protocols - _Switch to WebSocket_

#### Errors

- `409` Conflict - A gateway connection is already open

### `GET` `/user/<uuid>`

#### Path Fields

- `uuid`: `Uuid`

#### Response

`200` Ok

- `uuid`: `Uuid`
- `username`: `string`
- `relation`: `string` - either: `blocked`, `none`, `request`, or `friend`. Will be absent if request is made unauthenticated.
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
- `metadata`: `Metadata?`

##### Metadata

- `type`: `string`
- `attributes`: opaque json object with additional attributes for this activity

#### Errors

- `404` Not Found

### `POST` `/user/<uuid>?<relation>` [Authenticated](#Errors)

#### Path Fields

- `uuid`: `Uuid`

#### Query Fields

- `relation`: `string` - either: `blocked`, `none`, `request`, or `friend`

#### Response

`204` No Content

A successful result may be sent even when no change has occured under these conditions:

- The requested relation is already set
- A friend request was sent even though the other user is already a friend

Additionally, the relation will be set to friend if the authenticated user is trying to send a friend request to a user who has already sent a friend request.

#### Errors

- `400` Bad Request - If the authenticated user and the queried user are the same
- `403` Forbidden:
  - If the authenticated user is trying to friend a user who has not sent a friend request
  - If the authenticated user is trying to send a friend request to a user who has blocked them
- `404` Not Found - If the queried user isn't known to the database

### `GET` `/user/<uuid>/images` [Authenticated](#Errors)

Get the ids of images a user has uploaded.

#### Path Fields

- `uuid`: `Uuid`

#### Response

- `[number]` - image ids

#### Errors

- `403` Forbidden:
  - The authenticated user isn't friends with the queried user

### `GET` `/channels` [Authenticated](#Errors)

Get a list of all channel ids the authenticated user participates in (owner + participant)

#### Response

`200` Ok

- `[number]` - json array of channel ids

### `GET` `/channel/<id>` [Authenticated](#Errors)

#### Path Fields

- `id` - channel id

#### Response

`200` Ok

- `id`: `number` - channel id
- `name`: `string` - channel name
- `owner`: `uuid` - uuid of the channel's owner
- `persistence`: `Persistence`
- `participants`: `[uuid]` - List of participants

#### Errors

- `400` Bad Request:

  - The authenticated user does not own or participate in the given channel
  - The given channel does not exist

  While this may seem odd this is a deliberate choice for privacy as otherwise it would be possible
  for bad actors to find channels through brute-force measures

### `POST` `/channel` [Authenticated](#Errors)

#### Body Parameters

- `name`: `string` - length between 1 and 32, not unique
- `persistence`: `Persistence`
- `participants`: `[uuid]` - List of UUIDs of other users that should participate in the newly created channel. Friends of the authenticated user are added immediately, otherwise a channel invite is sent.

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

### `PATCH` `/channel/<id>` [Authenticated](#Errors)

Update channel settings. Fields that shouldn't be changed can be left out.

#### Path Fields

- `id` - channel id

#### Body Fields

- `name`: `string?` - length between 1 and 32, not unique. Updated value, if desired to be changed
- `persistence`: `Persistence?` - Updated persistence of the channel
- `participants`: `[uuid]?` - List of UUIDs of other users that should be added to the channel.
  Friends of the authenticated user are added immediately, otherwise a channel invite is sent.

#### Response

`204` No Content

#### Errors

- `400` Bad request:
  - The channel does not exist
  - The authenticated user doesn't own the specified channel
  - The given body fields are invalid or malformed

### `POST` `/channel/<id>` [Authenticated](#Errors)

Send a message to a channel

#### Path Fields

- `id` - channel id

#### Body Fields

- `content`: `string` - The message, max. 2000 characters.
- `display_name`: `string` - The name under which to display this message, max. 179 characters. Used for proxying with PluralKit

#### Response

The id of the new message, in plain text

#### Errors

- `400` Bad request:
  - The channel does not exist
  - The authenticated user does not participate in or own the given channel

### `DELETE` `/channel/<id>` [Authenticated](#Errors)

Leave or delete a channel. If the authenticated user is participating, they leave the channel.
If they own the channel, it will be deleted as it is currently not possible to transfer a channel to a new owner.

#### Path Fields

- `id` - channel id

#### Response

`200` Ok

#### Errors

- `400` Bad request:
  - The channel does not exist
  - The authenticated user does not participate in or own the given channel

### `GET` `/channel/<id>/messages?<before?>` [Authenticated](#Errors)

Get up to 50 messages from a channel.

#### Path Fields

- `id` - channel id

#### Query Fields

- `before` - timestamp, used for pagination (optional)

#### Response

`[Message]`

##### Message

- `id`: `number` - The message id
- `channel_id`: `number` - The channel id
- `sender`: `uuid` - The sender's uuid
- `sender_name`: `string` - The sender's display name
- `content`: `string` - The message content
- `timestamp`: `Timestamp` - The timestamp of the message

#### Errors

- `400` Bad request:
  - The channel does not exist
  - The authenticated user does not participate in or own the given channel

### `POST` `/channel/<id>/remove?<user>` [Authenticated](#Errors)

Remove (kick) a user from a channel.

#### Path Fields

- `id` - channel id

#### Query Fields

- `user` - the uuid of the user to remove

#### Response

`200` Ok

#### Errors

- `400` Bad request:
  - The channel does not exists
  - The authenticated user does not own the specified channel

### `GET` `/channels/invites` [Authenticated](#Errors)

Get channel invites for the current user

#### Response

`[Invite]` - The list of channel invites

##### Invite

- `id`: `number` - The channel id
- `channel_name`: `string` - The name of the channel
- `from`: `uuid` - The user who sent the invite

### `POST` `/channels/invites?<id>&<accept>` [Authenticated](#Errors)

Accept/Ignore a channel invite

#### Query Fields

- `id` - The channel id
- `accept` - Whether to accept the channel invite, if false it is just ignored instead

#### Response

`200` Ok

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

Returns user data in Json format. Access tokens are not included.

#### Response

`200` Ok

- `user`:
  - `uuid`: `uuid`
  - `username`: `string`
  - `registered`: `Timestamp`
  - `last_online`: `Timestamp?`
  - `previous_usernames`: `[OldUsername]`
- `settings`:
  - `show_registered`: `boolean`
  - `retain_usernames`: `boolean`
  - `show_last_online`: `boolean`
  - `show_activity`: `boolean`
  - `allow_friends_image_access`: `boolean`
- `relations` - Map of Minecraft UUID to relation status
- `channels`: `[Channel]`
- `channel_invites`: `[ChannelInvite]`
- `images`: `[Image]`

##### OldUsername

- `username`: `string`
- `public`: `boolean`

##### Channel

- `id`: `number`
- `name`: `string`
- `settings`: `ChannelSettings?` - The settings of this channel, only present if this user owns this channel
- `participants`: `[Uuid]?`, - The participants of this channel, only present if this user owns this channel
- `messages`: `[Message]` - The messages the authenticated user has sent in this channel

##### ChannelSettings

- `created`: `Timestamp`
- `last_updated`: `Timestamp`
- `last_message`: `Timestamp`
- `persistence` - The channel persistence (c.f. above)
  - `type`: `string` - either: `channel`, `duration`, `count`, or `count_and_duration`
  - `count`: `number?` - only present if type is `count` or `count_and_duration`
  - `duration`: `number?` - seconds, only present if type is `duration` or `count_and_duration`

##### Message

- `id`: `number`
- `sender_name`: `string`
- `content`: `string`
- `send_time`: `Timestamp`

##### ChannelInvite

- `channel`: `number`
- `from`: `uuid`

##### Image

- `id`: `number`
- `filename`: `string` - The original file name of the image
- `file`: `string` - The image file, encoded with base64
- `timestamp`: `Timestamp` - The upload timestamp

### `POST` `/account/activity` [Authenticated](#Errors)

#### Body Fields

- `title`: `string`
- `description`: `string`
- `started`: `Timestamp`

#### Response

`200` Ok

### `GET` `/account/settings` [Authenticated](#Errors)

#### Response

`200` Ok

- `show_registered`: `boolean`
- `retain_usernames`: `boolean`
- `show_last_online`: `boolean`
- `show_activity`: `boolean`
- `allow_friends_image_access`: `boolean`

### `PATCH` `/account/settings` [Authenticated](#Errors)

#### Body Fields

- `show_registered`: `boolean?`
- `retain_usernames`: `boolean?`
- `show_last_online`: `boolean?`
- `show_activity`: `boolean?`
- `allow_friends_image_access`: `boolean?`

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

### `GET` `/account/relations/friends` [Authenticated](#Errors)

Get the list of friends for the currently authenticated user

#### Response

- `[uuid]` - json array of uuids

### `GET` `/account/relations/blocked` [Authenticated](#Errors)

Get the list of people the currently authenticated user has blocked

#### Response

- `[uuid]` - json array of uuids

### `GET` `/account/relations/requests` [Authenticated](#Errors)

Get the list of either incoming or outgoing friend requests for the currently authenticated user

#### Response

- `out`: `[uuid]` - json array of uuids of outgoing requests
- `in`: `[uuid]` - json array of uuids of incoming requests

### `GET` `/image/<id>`

Fetch a shared image (usually screenshots) with metadata.

#### Path Fields

`id` - The id of the image

#### Response

- `uploader`: `uuid` - The uuid of the player that shared this image
- `filename`: `string` - The name of the image file
- `file`: `string` - The file content, encoded with standard base64
- `shared_at`: `Timestamp`

#### Errors

- `404` - The provided image does not exist or has expired

### `GET` `/image/<id>/raw`

Fetch a raw image

#### Path Fields

`id` - The id of the image

#### Response

The raw bytes of the image

#### Errors

- `404` - The provided image does not exist or has expired

### `GET` `/image/<id>/view`

View an image in a browser.

#### Path Fields

`id` - The id of the image

#### Response

Html page in the style of the AxolotlClient website to display an image, with embedding support.

#### Errors

- `404` - The provided image does not exist or has expired

### `GET` `/image/<id>/oembed?<format>`

Get oEmbed information for an image. See https://oembed.com.

#### Path Fields

`id` - The id of the image

#### Query Fields

`format` - The oEmbed format. Only `json` is supported currently.

#### Response

- `version`: `string` - The oEmbed version, `1.0`
- `type`: `string` - The oEmbed type, `photo`
- `title`: `string` - The name of the image file
- `url`: `string` - The raw url of the image
- `width`: `number` - The image width, in pixels
- `height`: `number` - The image height, in pixels
- `provider_name`: `string` - The oEmbed provider name, `AxolotlClient`
- `provider_url`: `string` - The oEmbed provider url, `https://axolotlclient.com`

#### Errors

- `404` - The provided image does not exist or has expired

### `POST` `/image/<filename>` [Authenticated](#Errors)

Share an image in PNG format

#### Path Fields

`filename` - The name of the file to share

#### Body Fields

The image data, in raw bytes

#### Response

The image id, in plain text.

#### Errors

- `400` - The png file is malformed
- `413` - The image is over 8MiB in size

### `GET` `/hypixel` [Authenticated](#Errors)

Query cached values from the public hypixel API

#### Body Fields

- `request_type`: `string` - One of `network_level`, `bedwars_level`, `skywars_experience` or `bedwars_data`
- `target_player`: `uuid` - The player whose data should be fetched

#### Response

Response varies for each `request_type`:

`network_level`:

- `network_level`: `number` - The hypixel network level

`bedwars_level`:

- `bedwars_level`: `number`

`skywars_experience`:

- `skywars_experience`: `number`

`bedwars_data`:

- `final_kills_bedwars`: `number`
- `beds_broken_bedwars`: `number`
- `deaths_bedwars`: `number`
- `kills_bedwars`: `number`
- `losses_bedwars`: `number`
- `wins_bedwars`: `number`
- `winstreak`: `number`

`player_data`

- `name`: `string` - The player's name, as stored by Hypixel
- `bedwars`: `BedwarsData`
- `skywars`: `SkywarsData`
- `duels`: `DuelsData`
- `rank`: `string`
- `rank_formatted`: `string`
- `level`: `number`
- `karma`: `number`

##### BedwarsData

- `level`: `number`
- `all`: `BedwarsGameData` - data of all modes summed up
- `core`: `CombinedBedwarsGameData` - data of solo, doubles, trios and fours
- `solo`: `BedwarsGameData`
- `doubles`: `BedwarsGameData`
- `trios`: `BedwarsGameData`
- `fours`: `BedwarsGameData`
- `four_v_four`: `BedwarsGameData`
- `dreams`: `CombinedBedwarsGameData` - data of all dream modes (below) summed up
- `castle`: `BedwarsGameData`
- `doubles_lucky`: `BedwarsGameData`
- `fours_lucky`: `BedwarsGameData`
- `doubles_ultimate`: `BedwarsGameData`
- `fours_ultimate`: `BedwarsGameData`
- `doubles_armed`: `BedwarsGameData`
- `fours_armed`: `BedwarsGameData`
- `doubles_rush`: `BedwarsGameData`
- `fours_rush`: `BedwarsGameData`
- `doubles_swap`: `BedwarsGameData`
- `fours_swap`: `BedwarsGameData`

##### BedwarsGameData

- `kills`: `number`
- `deaths`: `number`
- `wins`: `number`
- `losses`: `number`
- `winstreak`: `number`
- `final_kills`: `number`
- `final_deaths`: `number`
- `beds_broken`: `number`
- `beds_lost`: `number`

##### CombinedBedwarsGameData

_BedwarsGameData without winstreak_

- `kills`: `number`
- `deaths`: `number`
- `wins`: `number`
- `losses`: `number`
- `final_kills`: `number`
- `final_deaths`: `number`
- `beds_broken`: `number`
- `beds_lost`: `number`

##### SkywarsData

- `level`: `string` - formatted level as provided by hypixel
- `exp`: `number`
- `all`: `SkywarsGameData` - data of all modes summed up
- `core`: `SkywarsGameData` - data of all solo + team modes summed up (there are no 'official' core modes in skywars)
- `solo`: `SkywarsModeData`
- `team`: `SkywarsModeData`
- `mega`: `SkywarsMegaModeData`
- `ranked`: `SkywarsGameData`
- `winstreak`: `number`

##### SkywarsModeData

- `normal`: `SkywarsGameData`
- `insane`: `SkywarsGameData`

##### SkywarsMegaModeData

- `normal`: `SkywarsGameData`
- `doubles`: `SkywarsGameData`

##### SkywarsGameData

- `kills`: `number`
- `deaths`: `number`
- `wins`: `number`
- `losses`: `number`

##### DuelsData

- `modes` - `Map<String, DuelsGameData>`

##### DuelsGameData

- `kills`: `number`
- `deaths`: `number`
- `wins`: `number`
- `losses`: `number`
- `winstreak`: `number`

### `POST` `/report/<message>` [Authenticated](#Errors)

Report a message.

#### Path Fields

- `message` - The id of the message to report

#### Response

`200` Ok

### `GET` `POST` `/brew_coffee`

RFC 2324 joke. Serves no purpose.

#### Response

`418` I'm a teapot

## Gateway

The gateway is used so the server knows the client is online. Messages are only sent from server to
client, the client continues to use normal http(s) requests to request data. A sequence of ping-pong
messages is used to keep the connection alive.

### Ping Pong

- Server will respond to any pings from client.
- Server will ping the client if there has been no communication for 10 seconds.
- Server will disconnect if there has been no communication for 10 seconds after the ping.
- The client does not _need_ to respond with a pong, but it should, at a minimum it just needs to communicate.

The websocket connection embodied by the gateway is only used for communication
from the server to the client. Currently, this is used for chat messages and
friend requests.

```json
{
  "target": "",
  ...
}
```

#### Currently implemented targets

- `friend_request`
  - body fields: `from`: `uuid` - The uuid of the player who sent the friend request
- `friend_request_accept`
  - body fields: `from`: `uuid` - The uuid of the player who accepted the friend request
- `friend_request_deny`
  - body fields: `from`: `uuid` - The uuid of the player who denied the friend request
- `chat_message`
  - body fields:
    - `channel`: `number` - channel id
    - `id`: `number` - message id
    - `sender`: `uuid` - The uuid of the sender
    - `sender_name`: `string` - The display name of the sender
    - `content`: `string` - The message content
- `activity_update`
  - body fields:
    - `user`: `uuid` - The user whose status has changed
    - `activity`: `Activity` - The new activity (as defined previously)
- `channel_invite`
  - body fields:
    - `channel`: `number` - channel id
    - `channel_name`: `string` - channel name
    - `from`: `uuid` - The sender of the invite
- `channel_invite_reaction`
  - body fields:
    - `channel`: `number` - channel id
    - `channel_name`: `string` - The channel name
    - `player`: `uuid` - The player who accepted or denied the channel invite
    - `accepted`: `boolean` - Whether the invite was accepted

### Closing Reasons

- `1000` Closed
- `1007` Invalid Data
- `1011` Error
- `1014` Timed Out - See [Ping Pong](#ping-pong)
