/**
 * Custom UUID class to make it easier to work with UUIDs.
 */
export default class UUID extends String {
  constructor(uuid: string) {
    super(uuid);
  }

  public static fromBuffer(uuid: Buffer): UUID {
    let string = "";

    for (let i = 0; i < 16; i++) {
      string += uuid[i].toString(16).padStart(2, "0");
      if (i === 3 || i === 5 || i === 7 || i === 9) {
        string += "-";
      }
    }

    return new UUID(string);
  }

  public static fromString(uuid: string): UUID {
    return new UUID(uuid);
  }

  public toBuffer(): Buffer {
    const buffer = Buffer.alloc(16);
    const cleanUUID = this.replace(/-/g, "");

    for (let i = 0; i < 16; i++) {
      buffer[i] = parseInt(cleanUUID.substr(i * 2, 2), 16);
    }

    return buffer;
  }

  public static generate(): UUID {
    const buffer = Buffer.alloc(16);

    for (let i = 0; i < 16; i++) {
      buffer[i] = Math.floor(Math.random() * 256);
    }

    buffer[6] = (buffer[6] & 0x0f) | 0x40;
    buffer[8] = (buffer[8] & 0x3f) | 0x80;

    return UUID.fromBuffer(buffer);
  }
  public toString(dashed = false): string {
    if (dashed) {
      return `${this.substr(0, 8)}-${this.substr(8, 4)}-${this.substr(12, 4)}-${this.substr(
        16,
        4
      )}-${this.substr(20, 12)}`;
    } else {
      return this as unknown as string;
    }
  }
}
