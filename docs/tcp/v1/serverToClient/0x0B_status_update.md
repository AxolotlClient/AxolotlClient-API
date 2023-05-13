# Status Update | Server To Client

## ID 0x0B

Sent by the server to indicate to the client that a friend's status has changed.

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
        <td>Must be <code>0x01</code></td>
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
        <td>Must be <code>0</code></td>
    </tr>
    <tr>
        <td>0x09</td>
        <td>16</td>
        <td>Player UUID</td>
        <td>uuid</td>
        <td>The UUID of the player whose status has changed</td>
    </tr>
    <tr>
        <td>0x19</td>
        <td>1</td>
        <td>Status Update Type</td>
        <td>uint8</td>
        <td>See below for more info</td>
    </tr>
    </tbody>
</table>

## Status Update types

0x1 - online\
0x2 - offline\
0x3 - in game\
0x4 - in game (unknown game)
