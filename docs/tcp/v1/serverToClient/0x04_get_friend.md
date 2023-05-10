# Get Friend | Server to Client

## ID 0x04

Sent by server to client in response to [Get Friend | Client to Server](../clientToServer/0x04_get_friend.md).

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
    </tr>
    <tr>
        <td>0x05</td>
        <td>16</td>
        <td>Player Name</td>
        <td>string</td>
        <td>Padded at end with null bytes <code>0x00</code></td>
    </tr>
    <tr>
        <td>0x15</td>
        <td>4</td>
        <td>Last Seen</td>
        <td>uint64</td>
        <td>Last login time, in epoch seconds</td>
    </tr>
    <tr>
        <td>0x19</td>
        <td>1</td>
        <td>Online</td>
        <td>uint8</td>
        <td>1 if online, 0 if offline</td>
    </tr>
    <tr>
        <td>0x1A</td>
        <td>64</td>
        <td>Status Title</td>
        <td>string</td>
        <td>Padded at end with null bytes <code>0x00</code></td>
    </tr>
    <tr>
        <td>0x5A</td>
        <td>64</td>
        <td>Status Description</td>
        <td>string</td>
        <td>Padded at end with null bytes <code>0x00</code> |  Uses the <a href="../../../formats/keywords.md">Keywords</a> format</td>
    </tr>
    <tr>
        <td>0x9A</td>
        <td>32</td>
        <td>Status Icon Path</td>
        <td>string</td>
        <td>Padded at end with null bytes <code>0x00</code> | Uses the <a href="../../../formats/keywords.md">Keywords</a> format</td></td>
    </tr>
    </tbody>
</table>
