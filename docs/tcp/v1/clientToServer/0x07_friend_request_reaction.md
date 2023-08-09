# Friend Request Reaction | Client to Server

## ID 0x07

Sent by client to server to create a friend request. Server will respond with [Friend Request Reaction | Server to Client](../serverToClient/0x07_friend_request_reaction.md).

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
        <td>Must be <code>0x07</code></td>
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
        <td>User UUID</td>
        <td>uuid</td>
        <td>The User who sent the friend request</td>
    </tr>
    <tr>
        <td>0x19</td>
        <td>1</td>
        <td>Reaction indicator</td>
        <td>uint8</td>
        <td><code>0x00</code> for denying the request, <code>0x01</code> for accepting it</td>
    </tr>
    </tbody>
</table>

 
