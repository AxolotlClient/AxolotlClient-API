# Global Data | Server to Client

## ID 0x02

Sent after client requests global data. See [Global Data | Client to Server](../clientToServer/0x02_global_data.md) for more info.

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
        <td>Must be <code>0x02</code></td>
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
        <td>Global Data Status</td>
        <td>uint8</td>
        <td><code>0x00</code> = Success, <code>0x01</code> = Failure</td>
    </tbody>
    <tr>
        <td>0x0A</td>
        <td>4</td>
        <td>Total Players</td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x0E</td>
        <td>4</td>
        <td>Players Online</td>
        <td>uint32</td>
        <td></td>
    </tr>
    <tr>
        <td>0x12</td>
        <td>4</td>
        <td>Latest Mod Version</td>
        <td>4x uint8</td>
        <td>Endoded version, see [Encoded Version](#encoded-version) </td>
    </tr>
    <tr>
        <td>0x16</td>
        <td>4</td>
        <td>Misc text length</td>
        <td>uint64</td>
        <td>Length of next field</td>
    </tr>
    <tr>
        <td>0x1A</td>
        <td>4</td>
        <td>Misc text</td>
        <td>string</td>
        <td>Text to display on the main menu, can be anything, like patch notes, or news.</td>
    </tr>
        
</table>


### Encoded Version

    byte 0: major version
    byte 1: minor version
    byte 2: patch version
    byte 3: flags

    flags:
    0x01: snapshot
    0x02: prerelease
    0x04: release
    0x08: debug
    0x10: experimental
    0x20: custom

example: **1.2.3-SNAPSHOT**

    byte 0: 0x01
    byte 1: 0x02
    byte 2: 0x03
    byte 3: 0x01
