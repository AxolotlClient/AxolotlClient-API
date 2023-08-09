# Get Channel List | Server To Client

## ID 0x0F

Sent in response to [Get Channel List | Client To Server](../clientToServer/0x0F_get_channel_list.md)

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
        <td>Must be <code>0x0F</code></td>
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
        <td>4</td>
        <td>Channel Count <code>[f]</code></td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x0D</td>
        <td>5 * <code>[f]</code></td>
        <td>Channel IDs</td>
        <td>string</td>
        <td></td>
    </tr>
    </tbody>
</table>
