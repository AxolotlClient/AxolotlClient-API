# Get Channel | Server To Client

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
        <td>5</td>
        <td>Channel Id</td>
        <td>string</td>
        <td>Random string uniquely identifying this channel</td>
    </tr>
    <tr>
        <td>0x0E</td>
        <td>64</td>
        <td>Channel Name</td>
        <td>string</td>
        <td>The name of this channel. See below for more information.</td>
    </tr>
    <tr>
        <td>0x4E</td>
        <td>4</td>
        <td>Count of UUIDs <code>[f]</code></td>
        <td>uint32</td>
        <td>The count of UUIDs in the next field</td>
    </tr>
    <tr>
        <td>0x53</td>
        <td>16 * <code>[f]</code></td>
        <td>Player UUIDs</td>
        <td>uuid</td>
        <td>The Players who take part in this chat (excluding this one)</td>
    </tr>
    <tr>
        <td>0x09 + 16 * <code>[f]</code></td>
        <td>1</td>
        <td>Count of Messages <code>[g]</code></td>
        <td>uint8</td>
        <td>The count of Messages</td>
    </tr>
    <tr>
        <td>0x0A + 16 * <code>[f]</code></td>
        <td>variable</td>
        <td>Messages</td>
        <td>multiple</td>
        <td>See below for the message format</td>
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
        <td>Message ID</td>
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

Channel Name: The name of the group, or, in case of a DM, the name of the receiving user. Padded at the end with `0x00`