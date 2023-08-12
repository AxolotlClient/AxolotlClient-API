import Message, { MessageType } from "../base/message";

/**
 * Request global data from server.
 */
export default class S_GlobalData extends Message<MessageType.ServerToClient, {
  status: GlobalDataStatus;
  totalPlayers: number;
  playersOnline: number;
  latestVersion: string;
}> {
  constructor() {
    super(MessageType.ServerToClient, "S_GlobalData", 0x02);
  }

  public parse(data: Buffer): S_GlobalData {
    data = this.getHeaderData(data);
    return this;
  }

  public serialize(): Buffer {
    const buffer = Buffer.alloc(0);
    return buffer;
  }
}

export enum GlobalDataStatus {
  Success,
  Failure,
}
