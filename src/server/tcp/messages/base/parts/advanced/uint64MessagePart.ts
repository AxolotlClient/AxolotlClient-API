import { FilledCheck } from "../../../../../../util/utilTypes";
import NumberMessagePart from "../basic/numberMessagePart";

/**
 * A message part that represents an unsigned 64-bit integer.
 */
export default class Uint64MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint64", 8, data);
  }
}
