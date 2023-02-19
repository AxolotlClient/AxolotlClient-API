import { db } from "../..";
import { User } from "../../database/entities/user";

export default class UserManager {

  public onlineUsers: string[] = [];

  constructor() {
    this.onlineUsers = [];
  }

  public isOnline(uuid: string): boolean {
    return this.onlineUsers.includes(uuid);
  }

  public setStatus(uuid: string, status: boolean): void {
    if (status) {
      this.onlineUsers.push(uuid);
    } else {
      this.onlineUsers = this.onlineUsers.filter((user) => user !== uuid);
    }
  }

  public async getCount(): Promise<{
    online: number;
    total: number;
  }> {
    return {
      online: this.onlineUsers.length,
      total: await db.getEntityManager().count(User),
    };
  }
}
