import { Entity, ManyToOne, PrimaryKey, Property } from "@mikro-orm/core";
import { randomUUID } from "crypto";
import { User } from "./user";

@Entity()
export class FriendInvite {

    @PrimaryKey()
    id: string = randomUUID();

    @ManyToOne()
    from!: User;

    @ManyToOne()
    to!: User;

    @Property()
    createdAt: Date = new Date();    

}