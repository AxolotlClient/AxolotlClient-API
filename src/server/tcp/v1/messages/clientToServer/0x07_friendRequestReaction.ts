import UUID from "../../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to respond to a friend request.
 */
export default class C_FriendRequestReaction extends Message<
  MessageType.ClientToServer,
  {
    uuid: UUID;
    reaction: FriendRequestReaction;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_FriendRequestReaction", 0x07);
  }

  public parse(data: Buffer): C_FriendRequestReaction {
    data = this.getHeaderData(data);

    const uuid = UUID.fromBuffer(data.subarray(0, 16));
    this.data["uuid"] = uuid;
    this.data.reaction = data.readUInt8(16);

    return this;
  }

  public serialize(): Buffer {
    return Buffer.concat([this.data.uuid.toBuffer(), Buffer.from([this.data.reaction])]);
  }
}

export enum FriendRequestReaction {
  Deny = 0x00,
  Accept = 0x01,
}
