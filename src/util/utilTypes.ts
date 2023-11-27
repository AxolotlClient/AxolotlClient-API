export type If<T, Y, N> = T extends true ? Y : N;
export type FilledCheck<Filled, T> = If<Filled, T, void>;
export type Not<T extends boolean> = If<T, false, true>;

export type NumberToString<N extends number> = `${N}`;

type StringToNumber<T extends string, A extends any[] = []> =
  T extends keyof [0, ...A] ? A['length'] : StringToNumber<T, [0, ...A]>