import Message, { MessageType } from "../base/message";

/**
 * Requests the currently unanswered friend requests of this user. 
 */
export default class C_GetFriendRequests extends Message<MessageType.ClientToServer, {}> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetFriendRequests", 0x08);
  }

  public parse(data: Buffer): C_GetFriendRequests {
    data = this.getHeaderData(data);
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(0);
    return buffer;
  }
}
