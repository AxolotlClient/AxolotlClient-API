export default class LanguageValue<K extends string = string, V extends string[] = string[]> {

    public readonly name: K;
    public readonly values: V;

    constructor(name: K, ...values: V) {
        this.name = name;
        this.values = values;
    }

    public toString(): string {
        return `${this.name}:${this.values.join(":")}`
    }
}