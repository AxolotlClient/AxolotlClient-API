import FadeIn from "./indexResources/fadeIn";

FadeIn.runFadeIn({
  initalDelay: 500,
  spacing: 100,
});

// get user platform (windows, mac, linux) for download links
// @ts-ignore
const platformID: string = navigator.platform.toLowerCase();

let platform: "windows" | "linux" | "mac-x64" | "mac-arm64";

if (platformID.includes("win")) {
  platform = "windows";
} else if (platformID.includes("mac")) {
  if (platformID.includes("arm")) {
    platform = "mac-arm64";
  } else {
    platform = "mac-x64";
  }
} else if (platformID.includes("linux")) {
  platform = "linux";
} else {
  platform = "windows";
}

let platformExtension: string;

if (platform === "windows") {
  platformExtension = "exe";
} else if (platform === "linux") {
  platformExtension = "AppImage";
} else if (platform === "mac-x64") {
  platformExtension = "-x64.dmg";
} else if (platform === "mac-arm64") {
  platformExtension = "-arm64.dmg";
}

let formattedPlatform: string;

if (platform === "windows") {
  formattedPlatform = "Windows";
} else if (platform === "linux") {
  formattedPlatform = "Linux";
} else if (platform === "mac-x64") {
  formattedPlatform = "Mac (Intel)";
} else if (platform === "mac-arm64") {
  formattedPlatform = "Mac (Apple Silicon)";
}

// set download link

// get status data

(async () => {
  // fetch client data
  const clientData = (await fetch("/api/v1/count").then((res) => res.json())) as {
    online: number;
    total: number;
  };

  // set client count
  document.getElementById("users")!.innerHTML = clientData.total.toLocaleString();
  document.getElementById("online")!.innerHTML = clientData.online.toLocaleString();

  // fetch modrinth data
  const modrinthData = (await fetch("https://api.modrinth.com/v2/project/axolotlclient").then((res) =>
    res.json()
  )) as {
    downloads: number;
  };

  // set modrinth download count
  document.getElementById("downloads")!.innerHTML = modrinthData.downloads.toLocaleString();

  // fetch discord data

  const discordData = (await fetch("https://discord.com/api/guilds/872856682567454720/widget.json").then(
    (res) => res.json()
  )) as {
    presence_count: number;
  };

  // set discord member count
  document.getElementById("discord")!.innerHTML = discordData.presence_count.toLocaleString();
})();
