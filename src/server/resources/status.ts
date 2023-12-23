import { If } from "../../util/utilTypes";
import LanguageStrings from "./lang/languageStrings";
import LanguageValue from "./lang/languageValue";

export default class Status<Type extends StatusType = StatusType.Unknown> {

    public online: If<Type extends StatusType.Online ? true : false, true, undefined>;
    public icon?: string;
    public title: string | LanguageValue
    public description?: string | LanguageValue
    public text?: string | LanguageValue
    public startedAt?: number;

    constructor(online: boolean, icon: string, title: string | LanguageValue, description?: string | LanguageValue , text?: string, startedAt? : number) {
        this.online = online as If<Type extends StatusType.Online ? true : false, true, undefined>;
        this.icon = icon;
        this.title = title;
        this.description = description;
        this.text = text;
        this.startedAt = startedAt || Date.now();
    }

    // Predefined statuses for convenience

    public static readonly Online = new Status<StatusType.Online>(true, "online", LanguageStrings.Status.Online);
    public static readonly Offline = new Status<StatusType.Offline>(false, "offline", LanguageStrings.Status.Offline);
    public static readonly Unknown = new Status<StatusType.Unknown>(false, "unknown", LanguageStrings.Status.Offline);
}

export enum StatusType {
    Unknown,
    Online,
    Offline,
    Away
}
