export type AttributeBuffer = {
  type: "single";
  buffer: { offset: number } | { array: Float32Array };
  attribute: { type: "position"; size: 3 } | { type: "momentum"; size: 3 } | {
    type: "mass";
    size: 1;
  };
} | {
  type: "interleaved";
  buffer: { offset: number } | { array: Float32Array };
  attributes: Array<
    {
      attribute:
        | { type: "position"; size: 3 }
        | { type: "momentum"; size: 3 }
        | { type: "mass"; size: 1 };
      offset: number;
      stride: number;
    }
  >;
};
