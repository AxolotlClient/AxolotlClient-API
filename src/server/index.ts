import express from "express";
import http from "http";
import Logger from "../util/logger";
import UserManager from "./managers/userManager";

import v1 from "./api/v1";
import WebsocketServer from "./ws";
import expressWs from "express-ws";
import { WebSocket } from "ws";
import PreSocketConnection from "./ws/resources/preSocketConnection";
import path from "path";

export const userManager = new UserManager();

const appNoSocket = express();
export const server = http.createServer(appNoSocket);
const ews = expressWs(appNoSocket, server);
const app = ews.app;

app.use(express.json());

app.use((req, res, next) => {

  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization");

  next();

})

app.use("/api/v1", v1);
app.use("/assets", express.static(path.resolve("./data/client")))

app.get("/", (req, res) => {
  res.sendFile(path.resolve("./data/client/pages/index.html"))
})

app.get("/bg", (req, res) => {
  res.sendFile(path.resolve("./data/client/pages/bgtest.html"))
})

export const socketServer = new WebsocketServer(ews.getWss());
socketServer.register();

app.ws("/api/ws", (ws: WebSocket, req: http.IncomingMessage) => {
  const preConnectionInstance = new PreSocketConnection(ws);
  socketServer.preConnectionInstances.set(preConnectionInstance.id, preConnectionInstance);

  Logger.debug("Server", `Created pre-connection instance with ID ${preConnectionInstance.id}`);

  ws.on("message", (message: string) => {
    socketServer.connectionManager.onMessage(socketServer, ws, message);
  })
});

server.listen(parseInt(process.env.PORT || "8080"), () => {
  Logger.info("Server", `Server started on port ${process.env.PORT || "8080"}`);
});
