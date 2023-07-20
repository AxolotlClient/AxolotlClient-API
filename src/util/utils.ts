import fs from "fs";
import path from "path";

export default class Utils {
  public static getColor(str: string) {
    // calculate hash
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }
    // convert to hex
    let color = "#";
    for (let i = 0; i < 3; i++) {
      const value = (hash >> (i * 8)) & 0xff;
      color += ("00" + value.toString(16)).substr(-2);
    }
    return color;
  }

  public static randomKey(length: number) {
    const chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let key = "";
    for (let i = 0; i < length; i++) {
      key += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return key;
  }

  // return only values of array that are in whitelist
  public static onlyAllowWhitelistedArrayValues<T, V extends T>(array: T[], whitelist: V[] ): V[] {
    return array.filter((value) => whitelist.includes(value as V)) as V[];
  }
}