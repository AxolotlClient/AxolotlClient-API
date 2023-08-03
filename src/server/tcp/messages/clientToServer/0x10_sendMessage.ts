import BufferUtil from "../../../../util/bufferUtil";
import Message, { MessageType } from "../base/message";

/**
 * Sent to the server when loading messages for a channel.
 */
export default class C_SendMessage extends Message<
  MessageType.ClientToServer,
  {
    channelId: string;
    timestamp: Date;
    contentLength: number;
    content: string;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_SendMessage", 0x10);
  }

  public parse(data: Buffer): C_SendMessage {
    data = this.getHeaderData(data);

    const channelId = BufferUtil.readString(data, 0, 5);
    const timestamp = data.readBigUint64LE(6);
    const contentLength = data.readUInt8(14); 
    const content = BufferUtil.readString(data, 15, contentLength);

    this.data["channelId"] = channelId;
    this.data["timestamp"] = new Date(Number(timestamp));
    this.data["contentLength"] = contentLength;
    this.data["content"] = content;

    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(15);
    BufferUtil.writeString(buffer, this.data.channelId, 0);
    buffer.writeBigUint64LE(BigInt(this.data.timestamp.getTime()), 6);
    buffer.writeUInt8(this.data.contentLength, 14);
    BufferUtil.writeString(buffer, this.data.content, 15);

    return buffer;
  }
}

