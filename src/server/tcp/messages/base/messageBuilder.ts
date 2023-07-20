import Uint16MessagePart from "./parts/advanced/uint16MessagePart";
import Uint32MessagePart from "./parts/advanced/uint32MessagePart";
import Uint8MessagePart from "./parts/advanced/uint8MessagePart";
import MessagePart from "./parts/messagePart";

export default class MessageBuilder<Message = {}> {
  private message: Message;
  private parts: MessagePart<any>[] = [];
  private index: number = 0;

  constructor(message: Message = {} as Message) {
    this.message = message;
  }

  public parse(data: Buffer): Message {
    
    let offset = 0;

    for (const part of this.parts) {
        part.data = data.slice(offset, offset + part.length) as any;
        offset += part.length;
        }
        return this.message;
  }

  // Message Parts

  private addPart<T extends string, DataType>(
    name: T,
    part: MessagePart<DataType>
  ): MessageBuilder<Message & { [K in T]: DataType }> {
    const newBuilder = new MessageBuilder<Message & { [K in T]: DataType }>({
      ...this.message,
      [name]: {
        type: part.type,
        length: part.length,
        index: this.index++,
      },
    } as Message & { [K in T]: DataType });

    newBuilder.parts = [...this.parts, part];
    return newBuilder;
  }

  public uint8<T extends string>(name: T): MessageBuilder<Message & { [K in T]: number }> {
    return this.addPart(name, new Uint8MessagePart(void 0));
  }

  public uint16<T extends string>(name: T): MessageBuilder<Message & { [K in T]: number }> {
    return this.addPart(name, new Uint16MessagePart(void 0));
  }

  public uint32<T extends string>(name: T): MessageBuilder<Message & { [K in T]: number }> {
    return this.addPart(name, new Uint32MessagePart(void 0));
  }

  public uint64<T extends string>(name: T): MessageBuilder<Message & { [K in T]: number }> {
    return this.addPart(name, new Uint32MessagePart(void 0));
  }
}


const test = new MessageBuilder()
    .uint8("type")
    .uint16("length")
    .uint32("data")

