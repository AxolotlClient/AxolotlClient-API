import { FilledCheck } from "../../../../../../util/utilTypes";
import MessagePart from "../messagePart";

/**
 * A message part that represents a buffer.
 */
export default class BufferMessagePart<Filled> extends MessagePart<Buffer, Filled> {
  constructor(length: number, data: FilledCheck<Filled, Buffer>) {
    super("buffer", length, data);
  }
}
