import BufferUtil from "../../../../../util/bufferUtil";
import UUID from "../../../../../util/uuid";
import Message, { MessageType } from "../base/message";

/**
 * Sent by server to client to respond to handshake.
 */
export default class S_Handshake extends Message<
  MessageType.ServerToClient,
  {
    handShakeStatus: HandshakeStatus;
  }
> {
  constructor() {
    super(MessageType.ServerToClient, "S_Handshake", 0x01);
  }

  public parse(data: Buffer): S_Handshake {
    data = this.getHeaderData(data);

    const handShakeStatus = data.readUint8(0);
   
    this.data["handShakeStatus"] = handShakeStatus;

    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(1);

    buffer.writeUInt8(this.data.handShakeStatus, 0);

    return buffer;
  }
}

export enum HandshakeStatus {
  Success,
  Failure,
}
