export default class EncodedVersion {
  public static readonly VersionRegex = /^(\d+)\.(\d+)\.(\d+)(?:-(\w+))?$/;

  constructor(
    public readonly major: number,
    public readonly minor: number,
    public readonly patch: number,
    public readonly flags: VersionFlags = VersionFlags.None
  ) {}

  public static parse(version: VersionString) {
    const match = version.match(EncodedVersion.VersionRegex);
    if (!match) throw new Error(`Invalid version string: ${version}`);
    const [, major, minor, patch, flag] = match;

    return new EncodedVersion(
      Number(major),
      Number(minor),
      Number(patch),
      flag ? getFlagValue(flag) : VersionFlags.None
    );
  }

  public toString() {
    return `${this.major}.${this.minor}.${this.patch}${this.flags ? `-${VersionFlagNames[this.flags]}` : ""}`;
  }
}

export type VersionString = `${number}.${number}.${number}-${VersionFlags}` | `${number}.${number}.${number}`;

export enum VersionFlags {
  None = 0x0,
  Snapshot = 0x1,
  PreRelease = 0x2,
  Release = 0x4,
  Debug = 0x8,
  Experimental = 0x10,
  Custom = 0x20,
}

export const VersionFlagNames = {
  [VersionFlags.None]: "",
  [VersionFlags.Snapshot]: "SNAPSHOT",
  [VersionFlags.PreRelease]: "PRE-RELEASE",
  [VersionFlags.Release]: "RELEASE",
  [VersionFlags.Debug]: "DEBUG",
  [VersionFlags.Experimental]: "EXPERIMENTAL",
  [VersionFlags.Custom]: "CUSTOM",
} as const;

export const VersionFlagValues = {
  "": VersionFlags.None,
  SNAPSHOT: VersionFlags.Snapshot,
  "PRE-RELEASE": VersionFlags.PreRelease,
  RELEASE: VersionFlags.Release,
  DEBUG: VersionFlags.Debug,
  EXPERIMENTAL: VersionFlags.Experimental,
  CUSTOM: VersionFlags.Custom,
} as const;

export function getFlagValue(flag: string): VersionFlags {
  const value = VersionFlagValues[flag.toUpperCase() as keyof typeof VersionFlagValues];
  if (value === undefined) throw new Error(`Invalid version flag: ${flag.toUpperCase()}`);
  return value;
}
