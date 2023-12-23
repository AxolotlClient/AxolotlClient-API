import BufferUtil from "../../../../../util/bufferUtil";
import Message, { MessageType } from "../base/message";

/**
 * Sent by client to server to update the status of the user and as a heartbeat.
 */
export default class C_StatusUpdate extends Message<
  MessageType.ClientToServer,
  {
   statusTitle: string;
   statusDescription: string;
   statusIconPath: string;
  }
> {
  constructor() {
    super(MessageType.ClientToServer, "C_StatusUpdate", 0x0b);
  }

  public parse(data: Buffer): C_StatusUpdate {
    data = this.getHeaderData(data);

    const statusTitle = BufferUtil.readString(data, 0, 64);
    const statusDescription = BufferUtil.readString(data, 64, 64);
    const statusIconPath = BufferUtil.readString(data, 128, 32);

    this.data["statusTitle"] = statusTitle;
    this.data["statusDescription"] = statusDescription;
    this.data["statusIconPath"] = statusIconPath;

    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(160);

    BufferUtil.writeString(buffer, this.data.statusTitle, 0);
    BufferUtil.writeString(buffer, this.data.statusDescription, 64);
    BufferUtil.writeString(buffer, this.data.statusIconPath, 128);

    return buffer;
  }
}
