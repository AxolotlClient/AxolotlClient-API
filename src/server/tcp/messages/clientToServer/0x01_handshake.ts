import BufferUtil from "../../../../util/bufferUtil";
import UUID from "../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to initiate handshake.
 */
export default class C_Handshake extends Message<
  MessageType.ClientToServer,
  {
    uuid: UUID;
    serverId: string;
    playerName: string;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_Handshake", 0x01);
  }

  public parse(data: Buffer): C_Handshake {
    data = this.getHeaderData(data);

    const uuid = UUID.fromBuffer(data.subarray(0, 16));
    const serverId = BufferUtil.readString(data, 16, 40);
    const playerName = BufferUtil.readString(data, 56, 16);

    this.data["uuid"] = uuid;
    this.data["serverId"] = serverId;
    this.data["playerName"] = playerName;

    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(72);
    BufferUtil.writeBuffer(buffer, this.data.uuid.toBuffer(), 0);
    BufferUtil.writeString(buffer, this.data.serverId, 16);
    BufferUtil.writeString(buffer, this.data.playerName, 56);

    return buffer;
  }
}
