import { db } from "../../../..";
import { Channel } from "../../../database/entities/channel";
import Logger from "../../../util/logger";
import Utils from "../../../util/utils";
import { ChannelsClientToServer, ChannelsServerToClient, ErrorServerToClient } from "../../types";
import WebsocketConnection from "../resources/socketConnection";
import WebsocketChannel from "../resources/websocketChannel";

export default class ChannelsChannel extends WebsocketChannel<ChannelsServerToClient, ChannelsClientToServer> {

  /**
   *
   *  Channels
   *  Manages channels
   *
   */

    constructor() {
        super("channels");
    }

    public async onMessage(socket: WebsocketConnection, message: ChannelsClientToServer): Promise<void> {
        if (!message.data.hasOwnProperty("method")) {
            Logger.error("ChannelsChannel", `Received message from ${socket.uuid}, ID: ${socket.connectionId}, but message does not have property "method"!`);
            
            socket.send<ErrorServerToClient>({
                type: "error",
                data: {
                    message: `MALFORMED_PACKET:Message missing property "method"!`
                },
                timestamp: Date.now(),
                id: message.id
            })
        }

        switch (message.data.method) {
          case "get": {

            const allowedInclusions = [
                "messages",
                "users"
            ]

            const populate = message.data.include?.filter((inclusion) => allowedInclusions.includes(inclusion)) as any as ("messages" | "users")[]

            const channel = await db.getEntityManager().findOne(Channel, {
                id: message.data.id
            },  {
                populate
            })

            if (!channel) {
                socket.send<ErrorServerToClient>({
                    type: "error",
                    data: {
                        message: `CHANNEL_NOT_FOUND:Channel with ID ${message.data.id} not found!`
                    },
                    timestamp: Date.now(),
                    id: message.id
                })
                return;
            }

           
          }

            break;
    }
    }
}