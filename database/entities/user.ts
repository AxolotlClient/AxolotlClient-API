import { Collection, Entity, ManyToOne, OneToMany, PrimaryKey, Property } from "@mikro-orm/core";

@Entity()
export class User {
  @PrimaryKey()
  uuid!: string;

  @Property()
  username!: string;

  @Property()
  friends: string[] = [];

  @Property()
  blcoked: string[] = [];

  @Property()
  createdAt = new Date();

  @Property()
  lastSeen = new Date();
}
