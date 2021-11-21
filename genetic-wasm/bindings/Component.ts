import type { Point } from "./Point";
import type { Source } from "./Source";

export type Component = { type: "point" } & Point | { type: "source" } & Source;
