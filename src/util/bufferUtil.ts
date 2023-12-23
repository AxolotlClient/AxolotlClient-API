export default class BufferUtil {
    public static readString(buffer: Buffer, offset: number, length: number): string {
        let str = "";
        for (let i = offset; i < offset + length; i++) {
            str += String.fromCharCode(buffer[i]);
        }
        return str;
    }

    public static writeString(buffer: Buffer, str: string, offset: number): void {
        for (let i = 0; i < str.length; i++) {
            buffer.writeUInt8(str.charCodeAt(i), offset + i);
        }
    }

    public static writeBuffer(buffer: Buffer, data: Buffer, offset: number): void {
        for (let i = 0; i < data.length; i++) {
            buffer.writeUInt8(data.readUInt8(i), offset + i);
        }
    }
}