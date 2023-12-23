import UUID from "../../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to request user information.
 */
export default class C_GetUser extends Message<
  MessageType.ClientToServer,
  {
    uuid: UUID;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_GetUser", 0x05);
  }

  public parse(data: Buffer): C_GetUser {
    data = this.getHeaderData(data);

    const uuid = UUID.fromBuffer(data.subarray(0, 16));
    this.data["uuid"] = uuid;

    return this;
  }

  public serialize(): Buffer {
    return this.data.uuid.toBuffer();
  }
}
