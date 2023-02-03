var s = Object.defineProperty;
var a = (n, t, e) => t in n ? s(n, t, { enumerable: !0, configurable: !0, writable: !0, value: e }) : n[t] = e;
var i = (n, t, e) => (a(n, typeof t != "symbol" ? t + "" : t, e), e);
class h {
  constructor(t = new ArrayBuffer(0)) {
    i(this, "inner");
    i(this, "pos");
    i(this, "reading");
    i(this, "capacity");
    /**
     * Returns the (immutable) internal Uint8Array.
     */
    i(this, "view", () => this.inner.slice());
    /**
     * Writes a single u8 to the buffer as one byte.
     */
    i(this, "writeUint8", (t) => {
      if (t > 255 || t < 0)
        throw new Error("Value is out of range");
      const e = new Uint8Array(1);
      new DataView(e.buffer).setUint8(0, t), this.write(e);
    });
    /**
     * Writes a u16 to the buffer as two bytes.
     */
    i(this, "writeUint16", (t) => {
      if (t > 65535 || t < 0)
        throw new Error("Value is out of range");
      const e = new Uint8Array(2);
      new DataView(e.buffer).setUint16(0, t), this.write(e);
    });
    /**
     * Writes a u32 to the buffer as four bytes.
     */
    i(this, "writeUint32", (t) => {
      if (t > 4294967295 || t < 0)
        throw new Error("Value is out of range");
      const e = new Uint8Array(4);
      new DataView(e.buffer).setUint32(0, t), this.write(e);
    });
    /**
     * Writes an i8 to the buffer as one byte.
     */
    i(this, "writeInt8", (t) => {
      if (t > 127 || t < -128)
        throw new Error("Value is out of range");
      const e = new Uint8Array(1);
      new DataView(e.buffer).setInt8(0, t), this.write(e);
    });
    /**
     * Writes an i16 to the buffer as two bytes.
     */
    i(this, "writeInt16", (t) => {
      if (t > 32767 || t < -32768)
        throw new Error("Value is out of range");
      const e = new Uint8Array(2);
      new DataView(e.buffer).setInt16(0, t), this.write(e);
    });
    /**
     * Writes an i32 to the buffer as four bytes.
     */
    i(this, "writeInt32", (t) => {
      if (t > 2147483647 || t < -2147483648)
        throw new Error("Value is out of range");
      const e = new Uint8Array(4);
      new DataView(e.buffer).setInt32(0, t), this.write(e);
    });
    /**
     * Writes a bool to the buffer as a single byte.
     */
    i(this, "writeBool", (t) => {
      this.writeUint8(t ? 1 : 0);
    });
    /**
     * Writes a series of bools to the buffer as a single byte. This allows up
     * to 4 bools to be represented as a single byte. The first 4 bits are used
     * as a mask to determine which of the last 4 bits are set.
     */
    i(this, "writePackedBools", (t) => {
      if (t.length > 4)
        throw new Error(
          "Each packed bool can only represent 4 or fewer values"
        );
      let e = 0;
      for (const r of t)
        e <<= 1, r && (e |= 1);
      this.writeUint8(e);
    });
    /**
     * Writes a variable length string to the buffer. It will be prefixed with
     * its length in bytes as a u16 (two bytes).
     */
    i(this, "writeString", (t) => {
      const e = new TextEncoder().encode(t);
      this.writeUint16(e.length), this.write(e);
    });
    /**
     * Writes a string to the buffer without a length prefix.
     */
    i(this, "writeSizedString", (t) => {
      const e = new TextEncoder().encode(t);
      this.write(e);
    });
    /**
     * Writes an array of raw bytes to the buffer. Useful for serializing ///
       distinct structs into byte arrays and appending them to a buffer later.
     */
    i(this, "writeRawBytes", (t) => {
      this.write(t);
    });
    /**
     * Reads a u8 from the buffer.
     */
    i(this, "readUint8", () => {
      this.start_reading();
      const t = new Uint8Array(1);
      return t.set(this.inner.subarray(this.pos, this.pos + 1)), this.pos += 1, new DataView(t.buffer).getUint8(0);
    });
    /**
     * Reads a u16 from the buffer.
     */
    i(this, "readUint16", () => {
      this.start_reading();
      const t = new Uint8Array(2);
      return t.set(this.inner.subarray(this.pos, this.pos + 2)), this.pos += 2, new DataView(t.buffer).getUint16(0);
    });
    /**
     * Reads a u32 from the buffer.
     */
    i(this, "readUint32", () => {
      this.start_reading();
      const t = new Uint8Array(4);
      return t.set(this.inner.subarray(this.pos, this.pos + 4)), this.pos += 4, new DataView(t.buffer).getUint32(0);
    });
    /**
     * Reads an i8 from the buffer.
     */
    i(this, "readInt8", () => {
      this.start_reading();
      const t = new Uint8Array(1);
      return t.set(this.inner.subarray(this.pos, this.pos + 1)), this.pos += 1, new DataView(t.buffer).getInt8(0);
    });
    /**
     * Reads an i16 from the buffer.
     */
    i(this, "readInt16", () => {
      this.start_reading();
      const t = new Uint8Array(2);
      return t.set(this.inner.subarray(this.pos, this.pos + 2)), this.pos += 2, new DataView(t.buffer).getInt16(0);
    });
    /**
     * Reads an i32 from the buffer.
     */
    i(this, "readInt32", () => {
      this.start_reading();
      const t = new Uint8Array(4);
      return t.set(this.inner.subarray(this.pos, this.pos + 4)), this.pos += 4, new DataView(t.buffer).getInt32(0);
    });
    /**
     * Reads a bool from the buffer.
     */
    i(this, "readBool", () => {
      this.start_reading();
      const t = new Uint8Array(1);
      return t.set(this.inner.subarray(this.pos, this.pos + 1)), this.pos += 1, t[0] === 1;
    });
    /**
     *
     */
    i(this, "readPackedBools", () => []);
    /**
     * Reads a variable length string from the buffer.
     */
    i(this, "readString", () => {
      this.start_reading();
      const t = this.readUint16(), e = this.inner.subarray(this.pos, this.pos + t), r = new TextDecoder("utf-8").decode(e);
      return this.pos += t, r;
    });
    /**
     * Reads a sized string from the buffer. You must pass the length of the
     * string in bytes.
     */
    i(this, "readSizedString", (t) => {
      this.start_reading();
      const e = this.inner.subarray(this.pos, this.pos + t), r = new TextDecoder("utf-8").decode(e);
      return this.pos += t, r;
    });
    /**
     * Reads a sized string from the buffer. This will read from the buffers
     * current position until the end of the buffer, so this function should not
     * be used unless you know that the string is the last value in the buffer.
     * This removes the overhead of a length prefix; it is recommended to plan
     * your packets out such that they end with a sized string where possible.
     */
    i(this, "readStringRemaining", () => {
      this.start_reading();
      const t = this.inner.subarray(this.pos, this.inner.length), e = new TextDecoder("utf-8").decode(t);
      return this.pos = this.inner.length, e;
    });
    this.inner = new Uint8Array(t), this.pos = 0, this.reading = !1, this.capacity = 1024;
  }
  /**
   * Writes a byte array to the internal buffer. Returns the numbers of bytes
   * written to the buffer.
   *
   * This should only be called by the library.
   */
  write(t) {
    if (t.length > this.capacity || this.inner.length + t.length > this.capacity)
      throw new Error("Bufferfish is full");
    this.reading = !1;
    const e = new Uint8Array(this.inner.length + t.length);
    e.set(this.inner, 0), e.set(t, this.inner.length), this.inner = e;
    const r = t.length;
    return this.pos += r, r;
  }
  /**
   * Resets the buffer cursor to the start postion when reading after a write.
   *
   * This should only be called by the library.
   */
  start_reading() {
    this.reading || (this.pos = 0, this.reading = !0);
  }
  /**
   * Sets the max capacity (in bytes) for the internal buffer.
   */
  set_max_capacity(t) {
    if (t < 1)
      throw new Error("Max capacity must be at least 1 byte");
    this.capacity = t;
  }
  // public serialize = (obj: object) => {}
  // public serializeNumber = (number: number) => {}
  // public serializeString = (string: string) => {}
  // public serializeBoolean = (bool: boolean) => {}
}
export {
  h as Bufferfish
};
