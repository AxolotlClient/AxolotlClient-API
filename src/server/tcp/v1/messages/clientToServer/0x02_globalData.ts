import Message, { MessageType } from "../base/message";

/**
 * Request global data from server.
 */
export default class C_GlobalData extends Message<MessageType.ClientToServer, {}> {
  constructor() {
    super(MessageType.ClientToServer, "C_GlobalData", 0x02);
  }

  public parse(data: Buffer): C_GlobalData {
    data = this.getHeaderData(data);
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(0);
    return buffer;
  }
}
