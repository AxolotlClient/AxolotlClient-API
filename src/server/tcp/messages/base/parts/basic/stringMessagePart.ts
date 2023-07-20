import { FilledCheck } from "../../../../../../util/utilTypes";
import MessagePart from "../messagePart";

/**
 * A message part that represents a string.
 */
export default class StringMessagePart<Filled> extends MessagePart<string, Filled> {
  constructor(type: string = "string", length: number, data: FilledCheck<Filled, string>) {
    super(type, length, data);
  }
}
