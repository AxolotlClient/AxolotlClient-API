import BuiltPart from "./parts/builtPart";
import {
  BufferMessagePart,
  MessagePart,
  StringMessagePart,
  UUIDMessagePart,
  Uint16MessagePart,
  Uint32MessagePart,
  Uint64MessagePart,
  Uint8MessagePart,
} from "./parts/messageParts";

export default class MessageBuilder<Message = {}> {
  private message: Message;
  private parts: MessagePart<any>[] = [];
  private index: number = 0;

  constructor(message: Message = {} as Message) {
    this.message = message;
  }

  public build(): Message {
    return this.message;
  }

  // Message Parts

  private addPart<Name extends string, DataType extends MessagePart<any, false>>(
    name: Name,
    part: MessagePart<DataType>
  ): MessageBuilder<
    {
      [K in Name]: BuiltPart<Name, DataType>;
    } & Message
  > {
    const newBuilder = new MessageBuilder<{ [K in Name]: BuiltPart<Name, DataType> } & Message>({
      ...this.message,
      [name]: new BuiltPart(name, part, this.index++),
    } as Message & { [K in Name]: BuiltPart<Name, DataType> });

    newBuilder.parts = [...this.parts, part];
    return newBuilder;
  }

  public uint8<T extends string>(
    name: T
  ): MessageBuilder<
    {
      [K in T]: BuiltPart<T, Uint8MessagePart<false>>;
    } & Message
  > {
    return this.addPart(name, new Uint8MessagePart(void 0));
  }

  public uint16<T extends string>(
    name: T
  ): MessageBuilder<
    {
      [K in T]: BuiltPart<T, Uint16MessagePart<false>>;
    } & Message
  > {
    return this.addPart(name, new Uint16MessagePart(void 0));
  }

  public uint32<T extends string>(
    name: T
  ): MessageBuilder<
    {
      [K in T]: BuiltPart<T, Uint32MessagePart<false>>;
    } & Message
  > {
    return this.addPart(name, new Uint32MessagePart(void 0));
  }

  public uint64<T extends string>(
    name: T
  ): MessageBuilder<
    {
      [K in T]: BuiltPart<T, Uint64MessagePart<false>>;
    } & Message
  > {
    return this.addPart(name, new Uint64MessagePart(void 0));
  }

  public buffer<T extends string>(
    name: T,
    length: number
  ): MessageBuilder<
    {
      [K in T]: BuiltPart<T, BufferMessagePart<false>>;
    } & Message
  > {
    return this.addPart(name, new BufferMessagePart(length, void 0));
  }

  public string<T extends string>(
    name: T,
    length: number
  ): MessageBuilder<
    {
      [K in T]: BuiltPart<T, StringMessagePart<false>>;
    } & Message
  > {
    return this.addPart(name, new BufferMessagePart(length, void 0));
  }

  public uuid<T extends string>(
    name: T
  ): MessageBuilder<
    {
      [K in T]: BuiltPart<T, UUIDMessagePart<false>>;
    } & Message
  > {
    return this.addPart(name, new BufferMessagePart(16, void 0));
  }
}

const test = new MessageBuilder()
  .uint8("type")
  .uint16("length")
  .uint32("data")
  .uuid("uuid")
  .buffer("buffer", 16)
  .string("string", 16)
  .uint64("uint64")
  .build();
