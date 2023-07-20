import { FilledCheck } from "../../../../../../util/utilTypes";
import NumberMessagePart from "../basic/numberMessagePart";
/**
 * A message part that represents an unsigned 16-bit integer.
 */
export default class Uint16MessagePart<Filled> extends NumberMessagePart<Filled> {
  constructor(data: FilledCheck<Filled, number>) {
    super("uint16", 2, data);
  }
}
