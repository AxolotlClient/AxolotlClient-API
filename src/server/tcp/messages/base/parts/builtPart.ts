import { MessagePart } from "./messageParts";

export default class BuiltPart<Name extends string, DataType extends MessagePart<any, false>> {
  public type: DataType["type"];
  public length: DataType["length"];
  constructor(public name: Name, public part: DataType, public index: number) {
    this.type = part.type;
    this.length = part.length;
  }
}
