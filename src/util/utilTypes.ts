export type If<T, Y, N> = T extends true ? Y : N;
export type FilledCheck<Filled, T> = If<Filled, T, void>;
export type Not<T extends boolean> = If<T, false, true>;