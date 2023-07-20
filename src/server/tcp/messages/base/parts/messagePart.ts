import { FilledCheck } from "../../../../../util/utilTypes";

/**
 * Generic message part, extended by all other message parts.
 */
export default class MessagePart<DataType, Filled = false> {
  public type: string;
  public length: number;
  public data: FilledCheck<Filled, DataType>

  constructor(type: string, length: number, data: FilledCheck<Filled, DataType>) {
    this.type = type;
    this.length = length;
    this.data = data;
  }
}
