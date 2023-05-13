# Status Update | Client to Server

## ID 0x0B

Sent by client to server to update the status of the user and as a heartbeat. No server response.

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
        <td>64</td>
        <td>Status Title</td>
        <td>string</td>
        <td>Padded at end with null bytes <code>0x00</code></td>
    </tr>
    <tr>
        <td>0x49</td>
        <td>64</td>
        <td>Status Description</td>
        <td>string</td>
        <td>Padded at end with null bytes <code>0x00</code> |  Uses the <a href="../../../formats/keywords.md">Keywords</a> format</td>
    </tr>
    <tr>
        <td>0x89</td>
        <td>32</td>
        <td>Status Icon Path</td>
        <td>string</td>
        <td>Padded at end with null bytes <code>0x00</code> | Uses the <a href="../../../formats/keywords.md">Keywords</a> format</td></td>
    </tr>
    </tbody>
</table>
