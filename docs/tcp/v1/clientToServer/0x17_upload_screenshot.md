# Upload Screenshot | Client To Server

## ID 0x17

Sent to the server after triggering a screenshot upload. The Server should respond with [Upload Screenshot | Server to Client](../serverToClient/0x17_upload_screenshot.md).

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
        <td>4</td>
        <td>Screenshot Name length <code>[f]</code></td>
        <td>uint32</td>
        <td>the length of the screenshot name</td>
    </tr>
    <tr>
        <td>0x0C</td>
        <td><code>[f]</code></td>
        <td>Screenshot name</td>
        <td>string</td>
        <td></td>
    </tr>
    <tr>
        <td>0x18</td>
        <td>rest of readable bytes</td>
        <td>Screenshot Image Data</td>
        <td>string</td>
        <td></td>
    </tr>
    </tbody>
</table>
