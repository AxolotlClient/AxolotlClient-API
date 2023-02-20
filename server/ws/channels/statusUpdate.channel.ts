import { Status, StatusUpdateClientToServer, StatusUpdateServerToClient } from "../../types";
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
    switch (message.data.updateType) {
    case "online":
    
    break;
    case "offline":

    break;
    case "inGame":
    
    break;
    case "inGameUnknown":
        
    break;

    }
  }
}
