import fetch from "node-fetch";
import EncodedVersion, { VersionString } from "./encodedVersion";
import Logger from "../logger";

export default async function modrinthGetLatestVersion(): Promise<EncodedVersion> {
  const res = await fetch("https://api.modrinth.com/v2/project/axolotlclient/version", {
    headers: {
      "User-Agent": `AxolotlClient-Server ${process.env.npm_package_version}`,
      "Content-Type": "application/json",
    },
  });

  Logger.info("Modrinth", `Got response code ${res.status} from modrinth`);

  const json = await res.json();

  const versionString = json[0].version_number.split("+")[0];
  const versionType = json[0].version_type;

  const version = EncodedVersion.parse(`${versionString}-${versionType}` as VersionString);

  return version;
}
