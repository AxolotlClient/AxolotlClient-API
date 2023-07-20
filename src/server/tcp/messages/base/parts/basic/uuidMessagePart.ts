import MessagePart from "../messagePart";
import UUID from "../../../../../../util/uuid";
import { FilledCheck } from "../../../../../../util/utilTypes";

export type UUIDFilledCheck<Filled> = FilledCheck<Filled, UUID>;

/**
 * A message part that represents a UUID.
 */
export default class UUIDMessagePart<Filled> extends MessagePart<Buffer, Filled> {
  constructor(data: UUIDFilledCheck<Filled>) {
    super("uuid", 16, (data ? data.toBuffer() : void 0) as FilledCheck<Filled, Buffer>);
  }

  public get uuid(): FilledCheck<Filled, UUID> {
    if (!this.data) return void 0 as UUIDFilledCheck<Filled>;
    return UUID.fromBuffer(this.data) as UUIDFilledCheck<Filled>;
  }
}
