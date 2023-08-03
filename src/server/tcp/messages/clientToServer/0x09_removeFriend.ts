import UUID from "../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to remove a friend connection between two users.
 */
export default class C_RemoveFriend extends Message<
  MessageType.ClientToServer,
  {
    uuid: UUID;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_RemoveFriend", 0x09);
  }

  public parse(data: Buffer): C_RemoveFriend {
    data = this.getHeaderData(data);

    const uuid = UUID.fromBuffer(data.subarray(0, 16));
    this.data["uuid"] = uuid;

    return this;
  }

  public serialize(): Buffer {
    return this.data.uuid.toBuffer();
  }
}
