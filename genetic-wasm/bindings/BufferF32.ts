import type { PtrBufF32 } from "./PtrBufF32";
import type { Component } from "./Component";

export interface BufferF32 {
  ptr: PtrBufF32;
  items: number;
  component: Component;
}
