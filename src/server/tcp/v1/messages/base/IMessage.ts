export default interface IMessage<T> { 
    serialize(): Buffer
    parse(buffer: Buffer): T
}