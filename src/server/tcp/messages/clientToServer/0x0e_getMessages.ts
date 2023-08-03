import BufferUtil from "../../../../util/bufferUtil";
import Message, { MessageType } from "../base/message";

/**
 * Sent to the server when loading messages for a channel.
 */
export default class C_GetMessages extends Message<
  MessageType.ClientToServer,
  {
    channelId: string;
    messageCount: number;
    timestamp: Date;
    mode: MessageLoadMode;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetMessages", 0x0e);
  }

  public parse(data: Buffer): C_GetMessages {
    data = this.getHeaderData(data);

    const channelId = BufferUtil.readString(data, 0, 5);
    const messageCount = data.readUInt8(5);
    const timestamp = data.readBigUint64LE(6);
    const mode = data.readUInt8(14);

    this.data["channelId"] = channelId;
    this.data["messageCount"] = messageCount;
    this.data["timestamp"] = new Date(Number(timestamp));
    this.data["mode"] = mode;

    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(15);
    BufferUtil.writeString(buffer, this.data.channelId, 0);
    buffer.writeUInt8(this.data.messageCount, 5);
    buffer.writeBigUint64LE(BigInt(this.data.timestamp.getTime()), 6);
    buffer.writeUInt8(this.data.mode, 14);

    return buffer;
  }
}

/**
 * How to load messages.
 */
export enum MessageLoadMode {
  /**
   * Load messages before the given timestamp.
   */
  Before,
  /**
   * Load messages after the given timestamp.
   */
  After,
}
