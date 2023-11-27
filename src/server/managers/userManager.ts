import { Socket } from "net";
import OnlineUser from "../resources/user/onlineUser";
import TempUser from "../resources/user/tempUser";
import Logger from "../../util/logger";
import DataRouter from "./dataRouter";

export default class UserManager {

  public onlineUsers: OnlineUser[] = [];
  public tempUsers: TempUser[] = [];


  constructor() {
    this.onlineUsers = [];
  }

  public async onSocketCreate(socket: Socket): Promise<void> {
    const tempUser = new TempUser(socket);
    this.tempUsers.push(tempUser);

    Logger.info("UserManager", `Created temp user (index ${this.tempUsers.length}) ${tempUser.socket.remoteAddress}:${tempUser.socket.remotePort}... waiting for authentication.`);
    DataRouter.onSocketCreate(tempUser.socket);
  }

  public getCount(): number {
    return this.onlineUsers.length;
  }

  public isOnline(uuid: string): boolean {
    return this.onlineUsers.some((user) => user.uuid === uuid);
  }
 
}
