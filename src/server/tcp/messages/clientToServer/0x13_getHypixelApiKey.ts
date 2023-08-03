import Message, { MessageType } from "../base/message";

/**
 * Request global data from server.
 */
export default class C_GetHypixelApiKey extends Message<MessageType.ClientToServer, {}> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetHypixelApiKey", 0x13);
  }

  public parse(data: Buffer): C_GetHypixelApiKey {
    data = this.getHeaderData(data);
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(0);
    return buffer;
  }
}
