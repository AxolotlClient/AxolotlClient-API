# Send Message | Client To Server

## ID 0x10

Sent to the server after sending a message from a chat window. The server should send the message to all online members of this channel.

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
        <td>Must be <code>0x10</code></td>
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
        <td>0x0C</td>
        <td>8</td>
        <td>Message Timestamp</td>
        <td>uint64</td>
        <td>Timestamp in UNIX epoch seconds</td>
    </tr>
    <tr>
        <td>0x14</td>
        <td>4</td>
        <td>Sender name length <code>[f]</code></td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x18</td>
        <td><code>[f]</code></td>
        <td>Sender name</td>
        <td>string</td>
        <td></td>
    </tr>
    <tr>
        <td>0x18+<code>[f]</code></td>
        <td>4</td>
        <td>Content length <code>[g]</code></td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x1C+<code>[f]</code></td>
        <td><code>[g]</code></td>
        <td>Message Content</td>
        <td>string</td>
        <td></td>
    </tr>
    </tbody>
</table>

The server should add this message to the database with this user's uuid as the sender.
