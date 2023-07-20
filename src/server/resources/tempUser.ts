import { Socket } from "net";
import UUIDUtil from "../../util/uuid";
import UUID from "../../util/uuid";

/**
 * A temporary user is a user that has connected to the server but has not yet executed a handshake.
 */
export default class TempUser {

  public uuid: UUID;
  public createdAt = new Date();
  public socket: Socket;

  constructor(socket: Socket) {
    this.socket = socket;
    this.uuid = UUIDUtil.generate();
  }
}
