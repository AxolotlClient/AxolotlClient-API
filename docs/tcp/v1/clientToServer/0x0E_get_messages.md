# Get Messages | Client To Server

## ID 0x0E

Sent to the server when loading .

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
        <td>Must be <code>0x0E</code></td>
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
        <td>5</td>
        <td>Channel ID</td>
        <td>string</td>
        <td></td>
    </tr>
    <tr>
        <td>0x0D</td>
        <td>1</td>
        <td>Count of Messages <code>[f]</code></td>
        <td>uint8</td>
        <td>The count of Messages to load from the server</td>
    </tr>
    <tr>
        <td>0x0E</td>
        <td>8</td>
        <td>Timestamp to load</td>
        <td>uint64</td>
        <td></td>
    </tr>
    <tr>
        <td>0x17</td>
        <td>1</td>
        <td>Load mode</td>
        <td>uint8</td>
        <td><code>0x00</code> to load messages <strong>before</strong> the timestamp, <code>0x01</code> to load messages <strong>after</strong> the timestamp</td>
    </tr>
    </tbody>
</table>

## Message Format
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
    <tr>
        <td>0x00</td>
        <td>16</td>
        <td>Sender UUID</td>
        <td>uuid</td>
        <td></td>
    </tr>
    <tr>
        <td>0x10</td>
        <td>8</td>
        <td>Message Timestamp</td>
        <td>uint64</td>
        <td>Timestamp in UNIX epoch seconds</td>
    </tr>
    <tr>
        <td>0x18</td>
        <td>1</td>
        <td>Message Flags</td>
        <td>uint8</td>
        <td></td>
    </tr>
    <tr>
        <td>0x19</td>
        <td>4</td>
        <td>Content length <code>[f]</code></td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x1D</td>
        <td><code>[f]</code></td>
        <td>Message Content</td>
        <td>string</td>
        <td></td>
    </tr>
    <tbody>
    </tbody>
</table>
