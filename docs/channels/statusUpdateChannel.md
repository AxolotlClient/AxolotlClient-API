# Status Update Channel

Used to send Discord "Rich Presence"-like updates to the Discord client.

```
id: statusUpdate
```

## Server To Client

```json
{
    "type": "statusUpdate",
    "data": {
        "icon": "statusGreenCircle", // pre-defined icon from some kind of list
        "uuid": "6392bacb8b00436682b33aaa134e83ac1", // The UUID of the user sending the update
        "title": "On play.hypixel.net", // Limited to 32 characters
        "description": "Playing Skywars | In Game", // Limited to 64 characters
        "text": "3:31 elapsed | 1/8 players", // Limited to 64 characters
    },
    "timestamp": 1234567890
}
```

## Client To Server

```json
{
    "type": "statusUpdate",
    "data": {
        "updateType": "online", // pre-defined type list
        "uuid": "6392bacb8b00436682b33aaa134e83ac1", // The UUID of the user sending the update
        "update": {} // update data, depends on updateType
    },
    "timestamp": 1234567890
}
```
#
## Update Types

- `online`

    User started the game, but isn't in game yet.

    ```json
    {
        "location": "MAIN_MENU" // Menu ID
    }
    ```

    ### Menu IDs

    - `MAIN_MENU`
    - `SERVER_LIST`
    - `SETTINGS`
    
    Can add more in the future

- `inGame`

    User is in game. Used for supported servers.

    ```json
    {
        "server": "HYPIXEL", // Supported server ID
        "gameType": "SKYWARS", // Game ID
        "gameMode": "Normal", // Game Mode text
        "map": "SkyWars", // Map name
        "players": 1, // Number of players in the game
        "maxPlayers": 8, // Max number of players in the game
        "startedAt": 1234567890 // Unix timestamp of when the game started
    }
    ```

    ### Game IDs

    - `SKYWARS`
    - `BEDWARS`
    - `SURVIVAL_GAMES`
    - `MURDER_MYSTERY`
    - `BUILD_BATTLE`
    - `DUELS`
    - `UHC_CHAMPIONS`
    - `ARCADE`
    - `ARENA`
    - `LEGACY`
    - `PROTOTYPE`
    - `HOUSING`
    - `SKYBLOCK`
    - `SUPER_SMASH`
    - `TNTGAMES`
    - `VAMPIREZ`
    - `WALLS`
    - `GINGERBREAD`
    - `QUAKECRAFT`
    - `HOUSING_BETA`
    - `MCGO`
    - `BATTLEGROUND`
    - `SPEED_UHC`
    - `SKYCLASH`

    Note: copilot made this list, no idea if it's accurate I don't PVP
     
    ### Supported Server IDs

    None yet, update this later

- `inGameUnknown`

    User is in game, but the server is not supported.

    ```json
    {
        "server": "play.hypixel.net", // Server IP, or "localhost" if singleplayer
        "worldType": "overworld", // World type, "overworld", "nether", or "end"
        "worldName": "world", // World name text
        "gamemode": "survival", // Gamemode type "survival", "creative", "adventure", or "spectator"
        "startedAt": 1234567890 // Unix timestamp of when the game started
    }
    ```

- `offline`

    User logged out of the game.

    ```json
    {}
    ```
