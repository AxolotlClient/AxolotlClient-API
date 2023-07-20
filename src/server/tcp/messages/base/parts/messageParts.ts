import { FilledCheck } from "../../../../../util/utilTypes";
import UUID from "../../../../../util/uuid";

export class MessagePart<DataType, Filled = false> {
  public type: string;
  public length: number;
  public data: FilledCheck<Filled, DataType>;

  constructor(type: string, length: number, data: FilledCheck<Filled, DataType>) {
    this.type = type;
    this.length = length;
    this.data = data;
  }
}
  

/**
 * 
 * A message part that represents a number.
 */
export class NumberMessagePart<Filled> extends MessagePart<number, Filled> {
  constructor(type: string, length: number, data: FilledCheck<Filled, number>) {
    super(type, length, data);
  }

  public parse(buffer: Buffer): number {
    return buffer.readUIntLE(0, this.length);
  }
}

/**
 * A message part that represents a buffer.
 */
export class BufferMessagePart<Filled> extends MessagePart<Buffer, Filled> {
  constructor(length: number, data: FilledCheck<Filled, Buffer>) {
    super("buffer", length, data);
  }
}

/**
 * A message part that represents a string.
 */
export class StringMessagePart<Filled> extends MessagePart<string, Filled> {
  constructor(type: string = "string", length: number, data: FilledCheck<Filled, string>) {
    super(type, length, data);
  }
}

export class Uint8MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint8", 1, data);
  }
}

/**
 * A message part that represents an unsigned 16-bit integer.
 */
export class Uint16MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint16", 2, data);
  }
}

/**
 * A message part that represents an unsigned 32-bit integer.
 */
export class Uint32MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint32", 4, data);
  }
}

/**
 * A message part that represents an unsigned 64-bit integer.
 */
export class Uint64MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint64", 8, data);
  }
}

export type UUIDFilledCheck<Filled> = FilledCheck<Filled, UUID>;
/**
 * A message part that represents a UUID.
 */
export class UUIDMessagePart<Filled> extends MessagePart<Buffer, Filled> {
  constructor(data: UUIDFilledCheck<Filled>) {
    super("uuid", 16, (data ? data.toBuffer() : void 0) as FilledCheck<Filled, Buffer>);
  }

  public get uuid(): FilledCheck<Filled, UUID> {
    if (!this.data) return void 0 as UUIDFilledCheck<Filled>;
    return UUID.fromBuffer(this.data) as UUIDFilledCheck<Filled>;
  }
}


