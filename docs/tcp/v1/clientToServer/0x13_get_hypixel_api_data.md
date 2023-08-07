# Get Hypixel API Data | Client to Server

## ID 0x13

Sent by the client to the server to request Data from the Hypixel API. Server will respond with [Get Hypixel API Data | Server to Client](../serverToClient/0x13_get_hypixel_api_key.md).

<table>
    <thead>
        <tr>
            <th>Offset</th>
            <th>Size</th>
            <th>Field Name</th>
            <th>Field Type</th>
            <th>Notes</th>
        </tr>
    </thead>
    <tbody>
    <tr>
        <td>0x00</td>
        <td>3</td>
        <td>Magic</td>
        <td>string</td>
        <td>Must be <code>AXO</code></td>
    </tr>
        <tr>
        <td>0x03</td>
        <td>1</td>
        <td>Packet Type</td>
        <td>uint8</td>
        <td>Must be <code>0x04</code></td>
    </tr>
    <tr>
        <td>0x04</td>
        <td>1</td>
        <td>Protocol Version</td>
        <td>uint8</td>
        <td></td>
    </tr>
    <tr>
        <td>0x05</td>
        <td>4</td>
        <td>Packet Identifier</td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
            <td>0x09</td>
            <td>32</td>
            <td>Player UUID</td>
            <td>string</td>
            <td>The player to request data for</td>
        </tr>
    <tr>
        <td>0x29</td>
        <td>4</td>
        <td>requested Data type, see below for available types</td>
        <td>uint32</td>
        <td></td>
    </tr>
    </tbody>
</table>

### Data types

<table>
    <thead>
        <tr>
            <td>Data Type ID</td>
            <td>Description</td>
            <td>Type</td>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>0x01</td>
            <td>Network Level</td>
            <td>uint32</td>
        </tr>
        <tr>
            <td>0x2</td>
            <td>Bedwars Level</td>
            <td>uint32</td>
        </tr>
        <tr>
            <td>0x3</td>
            <td>Skywars Experience</td>
            <td>uint32</td>
        </tr>
        <tr>
            <td>0x4</td>
            <td>Bedwars Data</td>
            <td>Data object, see below.</td>
        </tr>
    </tbody>
</table>

### Bedwars Data

- final_kills_bedwars
- final_deaths_bedwars
- beds_broken_bedwars
- deaths_bedwars
- kills_bedwars
- losses_bedwars
- wins_bedwars
- winstreak

<table>
    <thead>
        <tr>
            <th>Offset</th>
            <th>Size</th>
            <th>Field Name</th>
            <th>Field Type</th>
            <th>Notes</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>0x00</td>
            <td>4</td>
            <td>Final Kills</td>
            <td>uint32</td>
            <td></td>
        </tr>
        <tr>
            <td>0x04</td>
            <td>4</td>
            <td>Final Deaths</td>
            <td>uint32</td>
            <td></td>
        </tr>
        <tr>
            <td>0x08</td>
            <td>4</td>
            <td>Beds Broken</td>
            <td>uint32</td>
            <td></td>
        </tr>
        <tr>
            <td>0x0B</td>
            <td>4</td>
            <td>Deaths</td>
            <td>uint32</td>
            <td></td>
        </tr>
        <tr>
            <td>0x0F</td>
            <td>4</td>
            <td>Kills</td>
            <td>uint32</td>
            <td></td>
        </tr>
        <tr>
            <td>0x14</td>
            <td>4</td>
            <td>Losses</td>
            <td>uint32</td>
            <td></td>
        </tr>
        <tr>
            <td>0x18</td>
            <td>4</td>
            <td>Wins</td>
            <td>uint32</td>
            <td></td>
        </tr>
        <tr>
            <td>0x1B</td>
            <td>4</td>
            <td>Winstreak</td>
            <td>uint32</td>
            <td></td>
        </tr>
    </tbody>
</table>
