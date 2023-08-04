# Get or create Channel | Client To Server

## ID 0x0D

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
        <td>Must be <code>0x0D</code></td>
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
        <td>1</td>
        <td>Count of UUIDs <code>[f]</code></td>
        <td>uint8</td>
        <td>The count of UUIDs in the next field</td>
    </tr>
    <tr>
        <td>0x0A</td>
        <td>16 * <code>[f]</code></td>
        <td>Player UUIDs</td>
        <td>uuid</td>
        <td>The Players who take part in this chat (excluding this one)</td>
    </tr>
    </tbody>
</table>
