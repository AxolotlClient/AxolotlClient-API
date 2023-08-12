import express from "express";
import http from "http";
import Logger from "../util/logger";

import UserManager from "./managers/userManager";
import AuthenticationManager from "./managers/authManager";

import v1 from "./api/v1";
import path from "path";

export const userManager = new UserManager();
export const authManager = new AuthenticationManager();

export const app = express();
export const server = http.createServer(app);

app.use(express.json());

app.use((req, res, next) => {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization");

  next();
});

app.use("/api/v1", v1);
app.use("/assets", express.static(path.resolve("./data/client")));

app.get("/", (req, res) => {
  res.sendFile(path.resolve("./data/client/pages/index.html"));
});

app.get("/bg", (req, res) => {
  res.sendFile(path.resolve("./data/client/pages/bgtest.html"));
});

server.listen(parseInt(process.env.PORT || "8080"), () => {
  Logger.info("Server", `Server started on port ${process.env.PORT || "8080"}`);
});
