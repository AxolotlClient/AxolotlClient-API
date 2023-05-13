# Get Friend Requests | Server to Client

## ID 0x08

Sent after client requests global data. See [Global Data | Client to Server](../clientToServer/0x08_get_friend_requests.md) for more info.

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
        <td>Must be <code>0x08</code></td>
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
        <td>0x0A</td>
        <td>4</td>
        <td>Incoming Friend Request Count <code>[f]</code></td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x0E</td>
        <td>16 * <code>f</code></td>
        <td>Incoming Friend Request UUIDs</td>
        <td>uuid</td>
        <td>The UUIDs of the users sending friend requests</td>
    </tr>
    <tr>
        <td>0x1E + 16 * <code>f</code></td>
        <td>4</td>
        <td>Outgoing Friend Request Count <code>[g]</code></td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x23 + 16 * <code>f</code></td>
        <td>16 * <code>g</code></td>
        <td>Outgoing Friend Request UUIDs</td>
        <td>uuid</td>
        <td>The UUIDs of the users this user sent friend requests to</td>
    </tr>
    </tbody>
</table>
