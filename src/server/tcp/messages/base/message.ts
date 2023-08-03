import BufferUtil from "../../../../util/bufferUtil";

export default class Message<Type extends MessageType, Format> {
  public type: Type;

  public id: number;
  public name: string;

  public data!: HeaderData & Format;

  constructor(type: Type, name: string, id: number) {
    this.type = type;
    this.name = name;
    this.id = id;
  }

  /**
   * Take a buffer and parse it into a message, setting the data property
   * @param data
   */
  public parse(_data: Buffer): Message<Type, Format> {
    // override this
    throw new Error(`Message[${this.name}].parse() not implemented`);
  }

  /**
   * Take the message in data and serialize it into a buffer.
   * NOTE: This should not include the header, only the data.
   * When sending the message, the header will be added automatically.
   */
  public serialize(): Buffer {
    // override this
    throw new Error(`Message[${this.name}].serialize() not implemented`);
  }

  public toString(): string {
    return `Message[${this.name}]: ${JSON.stringify(this.data)}`;
  }

  public getHeaderData(data: Buffer): Buffer {
    const magic = BufferUtil.readString(data, 0, 3);
    const packetType = data.readUInt8(3);
    const protocolVersion = data.readUInt8(4);
    const packetId = data.readUint32LE(5);

    // @ts-ignore
    this.data = {
      magic,
      packetType,
      protocolVersion,
      packetId,
    };

    return data.subarray(9);
  }
}

export enum MessageType {
  ServerToClient,
  ClientToServer,
}

export type HeaderData = {
  magic: string;
  packetType: number;
  protocolVersion: number;
  packetId: number;
};
