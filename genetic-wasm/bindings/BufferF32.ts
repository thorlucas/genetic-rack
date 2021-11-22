import type { Component } from "./Component";
import type { PtrBufF32 } from "./PtrBufF32";

export interface BufferF32 {
  ptr: PtrBufF32;
  items: number;
  component: Component;
}
