export type If<T, Y, N> = T extends true ? Y : N;
export type FilledCheck<Filled, T> = If<Filled, T, void>;
