import { FilledCheck } from "../../../../../../util/utilTypes";
import NumberMessagePart from "../basic/numberMessagePart";

/**
 * A message part that represents an unsigned 8-bit integer.
 */
export default class Uint8MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint8", 1, data);
  }
}
