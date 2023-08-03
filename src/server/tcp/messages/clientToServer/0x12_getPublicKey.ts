import Message, { MessageType } from "../base/message";

/**
 * Request global data from server.
 */
export default class C_GetPublicKey extends Message<MessageType.ClientToServer, {}> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetPublicKey", 0x12);
  }

  public parse(data: Buffer): C_GetPublicKey {
    data = this.getHeaderData(data);
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(0);
    return buffer;
  }
}
