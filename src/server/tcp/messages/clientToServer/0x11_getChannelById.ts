import BufferUtil from "../../../../util/bufferUtil";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to get a channel by ID.
 */
export default class C_GetChannelById extends Message<
  MessageType.ClientToServer,
  {
    channelId: string;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetChannelById", 0x11);
  }

  public parse(data: Buffer): C_GetChannelById {
    data = this.getHeaderData(data);
    const channelId = BufferUtil.readString(data, 0, 5);
    this.data["channelId"] = channelId;
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(5);
    BufferUtil.writeString(buffer, this.data.channelId, 0);
    return buffer;
  }
}
