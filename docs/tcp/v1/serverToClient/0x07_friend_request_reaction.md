# Friend Request Reaction | Server to Client

## ID 0x07

Sent to the recipient after a client reacts to a friend request (from the recipient). See [Create Friend Request | Client to Server](../clientToServer/0x06_create_friend.md) for more info.

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
        <td>Must be <code>0x06</code></td>
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
        <td>16</td>
        <td>Player UUID</td>
        <td>uuid</td>
        <td>The user who accepted the request</td>
    </tr>
    <tr>
        <td>0x19</td>
        <td>1</td>
        <td>Friend Status</td>
        <td>uint8</td>
        <td><code>0</code> when the request has been denied, <code>1</code> when it has been accepted</td>
    </tr>
    </tbody>
</table>

If there is no friend request from the provided user to react to, an [Error response](0xFF_error.md) should be replied (to the user sending the react request) instead.


