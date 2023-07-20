import { MikroORM, PostgreSqlDriver, EntityManager } from "@mikro-orm/postgresql";
import { TsMorphMetadataProvider } from "@mikro-orm/reflection";
import Logger from "../util/logger";

export default class Database {
  private orm!: MikroORM;
  private em!: EntityManager<PostgreSqlDriver>;

  constructor() {
    this.init();
  }

  public async init(): Promise<void> {
    const orm = await MikroORM.init<PostgreSqlDriver>({
      entities: ["./dist/database/entities/*.js"],
      type: "postgresql",
      tsNode: true,
      user: process.env.DB_USER,
      password: process.env.DB_PASS,
      dbName: process.env.DB_NAME,
      host: process.env.DB_HOST,
      port: process.env.DB_PORT ? parseInt(process.env.DB_PORT) : 5432,
      metadataProvider: TsMorphMetadataProvider,
    }).catch((err) => {
      Logger.error("Database", "Failed to initialize database");
      Logger.error("Database", err);
      console.error(err);
      process.exit(1);
    });

    this.orm = orm;
    this.em = orm.em;

    Logger.info("Database", "Database initialized");
  }

  public async close(): Promise<void> {
    await this.orm.close(true);
  }

  public getEntityManager(): EntityManager<PostgreSqlDriver> {
    return this.em.fork();
  }

  public getOrm(): MikroORM {
    return this.orm;
  }
}
