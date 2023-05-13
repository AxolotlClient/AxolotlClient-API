# Send Message | Server To Client

## ID 0x10

Sent to all online participants of the channel someone sent a message to, except the original sender.

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
        <td>16</td>
        <td>Sender UUID</td>
        <td>uuid</td>
        <td></td>
    </tr>
    <tr>
        <td>0x19</td>
        <td>8</td>
        <td>Message Timestamp</td>
        <td>uint64</td>
        <td>Timestamp in UNIX epoch seconds</td>
    </tr>
    <tr>
        <td>0x21</td>
        <td>1</td>
        <td>Message Type</td>
        <td>uint8</td>
        <td></td>
    </tr>
    <tr>
        <td>0x22</td>
        <td>4</td>
        <td>Content length <code>[f]</code></td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x26</td>
        <td><code>[f]</code></td>
        <td>Message Content</td>
        <td>string</td>
        <td></td>
    </tr>
    </tbody>
</table>