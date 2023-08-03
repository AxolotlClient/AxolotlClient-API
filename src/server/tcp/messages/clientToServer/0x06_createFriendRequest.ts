import UUID from "../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to create a friend request.
 */
export default class C_CreateFriendRequest extends Message<
  MessageType.ClientToServer,
  {
    uuid: UUID;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_CreateFriendRequest", 0x06);
  }

  public parse(data: Buffer): C_CreateFriendRequest {
    data = this.getHeaderData(data);

    const uuid = UUID.fromBuffer(data.subarray(0, 16));
    this.data["uuid"] = uuid;

    return this;
  }

  public serialize(): Buffer {
    return this.data.uuid.toBuffer();
  }
}
