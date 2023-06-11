# Handshake | Client to Server

## ID 0x01

Sent by client to server to initiate handshake. Server will respond with [Handshake | Server to Client](../serverToClient/0x01_handshake.md).

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
        <td></td>
    </tr>
    <tr>
        <td>0x09</td>
        <td>16</td>
        <td>Player UUID</td>
        <td>uuid</td>
        <td></td>
    </tr>
    <tr>
        <td>0x19</td>
        <td>40</td>
        <td>serverId/hash</td>
        <td>string</td>
        <td>https://wiki.vg/Protocol_Encryption </td>
    </tr>
    <tr>
        <td>0x41</td>
        <td>variable, rest of readable bytes</td>
        <td>Player Name</td>
        <td>string</td>
        <td></td>
    </tr>
    </tbody>
</table>
