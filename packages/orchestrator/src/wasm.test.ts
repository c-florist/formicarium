import { describe, expect, it } from "vitest";
import { add } from "../../core-rs/pkg/core_rs";

describe("Wasm Module", () => {
  it("should call a core-rs WASM function and get the correct result", () => {
    const result = add(2, 3);
    expect(result).toBe(5);
  });
});
