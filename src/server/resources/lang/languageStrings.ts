import LanguageValue from "./languageValue";

export default class LanguageStrings {

    public static readonly Status = {
        Online: new LanguageValue("status.online"),
        Offline: new LanguageValue("status.offline"),
        Away: new LanguageValue("status.away", "afk"),
    }

}