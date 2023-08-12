import { Socket } from "net";
import OpenSocket from "../openSocket";
import Status, { StatusType } from "../status";

/**
 * A user that is currently connected to the server.
 */
export default class User {
  public uuid: string;
  public username: string;
  public socket: OpenSocket;
  public status: Status<StatusType>

  constructor(socket: OpenSocket | Socket, uuid: string, username: string) {
    this.uuid = uuid;
    this.username = username;
    this.socket = socket instanceof OpenSocket ? socket : new OpenSocket(socket);
    this.status = Status.Offline
  }

}

export enum UserStatus {
  Temp,
  Handshake,
  Online,
}
