import Message, { MessageType } from "../base/message";

/**
 * Get the list of channels that the user has access to.
 */
export default class C_GetChannelList extends Message<MessageType.ClientToServer, {}> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetChannelList", 0x0f);
  }

  public parse(data: Buffer): C_GetChannelList {
    data = this.getHeaderData(data);
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(0);
    return buffer;
  }
}
