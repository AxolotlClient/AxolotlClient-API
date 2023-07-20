import { FilledCheck } from "../../../../../../util/utilTypes";
import NumberMessagePart from "../basic/numberMessagePart";

/**
 * A message part that represents an unsigned 32-bit integer.
 */
export default class Uint32MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint32", 4, data);
  }
}
