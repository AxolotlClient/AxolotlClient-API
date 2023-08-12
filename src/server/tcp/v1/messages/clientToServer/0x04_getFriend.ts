import UUID from "../../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to request friend information.
 */
export default class C_GetFriend extends Message<
  MessageType.ClientToServer,
  {
    uuid: UUID;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetFriend", 0x04);
  }

  public parse(data: Buffer): C_GetFriend {
    data = this.getHeaderData(data);

    const uuid = UUID.fromBuffer(data.subarray(0, 16));
    this.data["uuid"] = uuid;

    return this;
  }

  public serialize(): Buffer {
    return this.data.uuid.toBuffer();
  }
}
