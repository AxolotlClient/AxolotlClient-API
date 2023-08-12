import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to request friends list.
 */
export default class C_FriendsList extends Message<MessageType.ClientToServer, {}> {
  constructor() {
    super(MessageType.ClientToServer, "C_FriendsList", 0x03);
  }

  public parse(data: Buffer): C_FriendsList {
    data = this.getHeaderData(data);
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(0);
    return buffer;
  }
}
