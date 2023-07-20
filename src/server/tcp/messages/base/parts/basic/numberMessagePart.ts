import { FilledCheck } from "../../../../../../util/utilTypes";
import MessagePart from "../messagePart";

/**
 * A message part that represents a number.
 */
export default class NumberMessagePart<Filled> extends MessagePart<number, Filled> {
  constructor(type: string, length: number, data: FilledCheck<Filled, number>) {
    super(type, length, data);
  }

  public parse(buffer: Buffer): number {
    return buffer.readUIntLE(0, this.length);
  }
}
