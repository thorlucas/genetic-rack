export interface InterleavedAttributeDescriptor {
  attribute: { type: "position"; size: 3 } | { type: "momentum"; size: 3 } | {
    type: "mass";
    size: 1;
  };
  offset: number;
  stride: number;
}
