import Logger from "../../../util/logger";
import { ErrorServerToClient, Status, StatusUpdateClientToServer, StatusUpdateServerToClient } from "../../types";
import WebsocketConnection from "../resources/socketConnection";
import WebsocketChannel from "../resources/websocketChannel";

export default class StatusUpdateChannel extends WebsocketChannel<StatusUpdateServerToClient, StatusUpdateClientToServer> {
  /**
   *
   *  StatusUpdateChannel
   *  discord "Rich Presence" like status updates
   *
   */

  constructor() {
    super("statusUpdate");
  }

  public onMessage(socket: WebsocketConnection, message: StatusUpdateClientToServer): void {

    if (!message.data.hasOwnProperty("updateType")) {
      Logger.error("StatusUpdateChannel", `Received message from ${socket.uuid}, ID: ${socket.connectionId}, but message does not have property "updateType"!`);
      
      socket.send<ErrorServerToClient>({
        type: "error",
        data: {
          message: `MALFORMED_PACKET:Message missing property "updateType"!`
        },
        timestamp: Date.now(),
        id: message.id
      })
    }

    switch (message.data.updateType) {
    case "online":
    
    break;
    case "inGame":
    
    break;
    case "inGameUnknown":
        
    break;

    }
  }
}
