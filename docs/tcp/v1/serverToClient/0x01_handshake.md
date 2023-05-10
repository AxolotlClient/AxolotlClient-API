# Handshake | Server to Client

## ID 0x01

Sent after client sends handshake. See [Handshake | Client to Server](../clientToServer/0x01_handshake.md) for more info.

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
    </tr>
    <tr>
        <td>0x05</td>
        <td>1</td>
        <td>Handshake Status</td>
        <td>uint8</td>
        <td><code>0x00</code> = Success, <code>0x01</code> = Failure</td>
    </tbody>
</table>
