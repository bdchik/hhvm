<?hh

case type CT1 = int | string | bool;
case type CT2 = | int | string | bool;

case type MultiLineCT1 = int | string | bool | nonnull | null | mixed | (function(int, string, bool): void);
case type MultiLineCT2 = | int | string | bool | nonnull | null | mixed | (function(int, string, bool): void);
