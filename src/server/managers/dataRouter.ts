import fs from "fs";
import path from "path";
import OpenSocket from "../resources/openSocket";
import Message, { MessageType } from "../tcp/v1/messages/base/message";
import EventEmitter from "events";


export default class DataRouter {

    public static messageTypes: Map<string, any> = new Map();
    private static _emitter = new EventEmitter();

    public static loadMessageTypes(version: number = 1): void {
        const clientToServerFiles = fs.readdirSync(path.resolve(`./dist/server/tcp/v${version}/messages/clientToServer`)).filter((file) => file.endsWith(".js")).map((file) => ({ file, type: MessageType.ClientToServer }))
        const serverToClientFiles = fs.readdirSync(path.resolve(`./dist/server/tcp/v${version}/messages/serverToClient`)).filter((file) => file.endsWith(".js")).map((file) => ({ file, type: MessageType.ServerToClient }))

        for (const file of [...clientToServerFiles, ...serverToClientFiles]) {
            const msg = require(path.resolve(`./dist/server/tcp/v${version}/messages/${file.type == MessageType.ClientToServer ? "clientToServer" : "serverToClient"}/${file.file}`)).default
            const msgInstance = new msg() as Message<any, any>;
            DataRouter.messageTypes.set(`${version}:${file.type}:${msgInstance.id}`, msgInstance);
            console.log(`Loaded message type ${version}:${file.type}:${msgInstance.id} (${msgInstance.name})`);
        }
    }

    public static onSocketCreate(socket: OpenSocket): void {
        socket.socket.on("data", (msg) => {
            const magic = msg.toString("ascii", 0, 3);
            const packetType = msg.readUInt8(3);
            const protocolVersion = msg.readUInt8(4);
            const packetId = msg.readUInt32LE(5);

            if (magic != "AXO") {
                console.log(`${socket} sent invalid magic ${magic}`);
                return;
            }

            const msgHandler = DataRouter.getMessageType(protocolVersion, MessageType.ClientToServer, packetId);

            if (!msgHandler) {
                console.log(`Unknown message type ${protocolVersion}:${packetType}:${packetId}`);
                return;
            }

            const message = new msgHandler().parse(msg);

            console.log(`${socket} sent message ${message.name} (${packetId})`);
            DataRouter._emitter.emit(`message:${message.name}`, socket, message);
        })
    }

    public static getMessageType<T = any>(version: number, type: MessageType, id: number): T {
        return DataRouter.messageTypes.get(`${version}:${type}:${id}`) as T;
    }

}


