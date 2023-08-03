import BufferUtil from "../../../../util/bufferUtil";
import UUID from "../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to get a channel by members.
 */
export default class C_GetChannelByMembers extends Message<
  MessageType.ClientToServer,
  {
    userCount: number;
    users: UUID[];
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetChannelByMembers", 0x0d);
  }

  public parse(data: Buffer): C_GetChannelByMembers {
    data = this.getHeaderData(data);

    const userCount = data.readInt8(0);
    const users: UUID[] = [];

    for (let i = 0; i < userCount; i++) {
      // add 1 to skip the userCount byte
      const uuid = UUID.fromBuffer(data.subarray(1 + i * 16, 17 + i * 16));
      users.push(uuid);
    }

    this.data["userCount"] = userCount;
    this.data["users"] = users;

    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(1 + this.data.users.length * 16);
    buffer.writeInt8(this.data.users.length, 0);

    for (let i = 0; i < this.data.users.length; i++) {
      BufferUtil.writeBuffer(buffer, this.data.users[i].toBuffer(), 1 + i * 16);
    }

    return buffer;
  }
}
