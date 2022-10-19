var n = Object.defineProperty;
var o = (e, t, r) => t in e ? n(e, t, { enumerable: !0, configurable: !0, writable: !0, value: r }) : e[t] = r;
var i = (e, t, r) => (o(e, typeof t != "symbol" ? t + "" : t, r), r);
class a {
  constructor(t = new ArrayBuffer(0)) {
    i(this, "inner");
    i(this, "pos");
    i(this, "reading");
    i(this, "capacity");
    i(this, "view", () => this.inner.slice());
    i(this, "writeUint8", (t) => {
      if (t > 255 || t < 0)
        throw new Error("Value is out of range");
      this.write(new Uint8Array([t]));
    });
    i(this, "writeUint16", (t) => {
      if (t > 65535 || t < 0)
        throw new Error("Value is out of range");
      this.write(new Uint8Array([t >> 8, t & 255]));
    });
    i(this, "writeUint32", (t) => {
      if (t > 4294967295 || t < 0)
        throw new Error("Value is out of range");
      this.write(
        new Uint8Array([
          t >> 24,
          t >> 16 & 255,
          t >> 8 & 255,
          t & 255
        ])
      );
    });
    i(this, "writeInt8", (t) => {
      if (t > 127 || t < -128)
        throw new Error("Value is out of range");
      this.write(new Uint8Array([t]));
    });
    i(this, "writeInt16", (t) => {
      if (t > 32767 || t < -32768)
        throw new Error("Value is out of range");
      this.write(new Uint8Array([t >> 8, t & 255]));
    });
    i(this, "writeInt32", (t) => {
      if (t > 2147483647 || t < -2147483648)
        throw new Error("Value is out of range");
      this.write(
        new Uint8Array([
          t >> 24,
          t >> 16 & 255,
          t >> 8 & 255,
          t & 255
        ])
      );
    });
    i(this, "writeBool", (t) => {
      this.writeUint8(t ? 1 : 0);
    });
    i(this, "writePackedBools", (t) => {
      if (t.length > 4)
        throw new Error(
          "Each packed bool can only represent 4 or fewer values"
        );
      let r = 0;
      for (const s of t)
        r <<= 1, s && (r |= 1);
      this.writeUint8(r);
    });
    i(this, "writeString", (t) => {
      const r = new TextEncoder().encode(t);
      this.writeUint16(r.length), this.write(r);
    });
    i(this, "writeSizedString", (t) => {
      const r = new TextEncoder().encode(t);
      this.write(r);
    });
    i(this, "readUint8", () => {
      this.start_reading();
      const t = new Uint8Array(1);
      return t.set(this.inner.subarray(this.pos, this.pos + 1)), this.pos += 1, t[0];
    });
    i(this, "readUint16", () => {
      this.start_reading();
      const t = new Uint8Array(2);
      return t.set(this.inner.subarray(this.pos, this.pos + 2)), this.pos += 2, t[0] << 8 | t[1];
    });
    i(this, "readUint32", () => {
      this.start_reading();
      const t = new Uint8Array(4);
      return t.set(this.inner.subarray(this.pos, this.pos + 4)), this.pos += 4, (t[0] << 24 | t[1] << 16 | t[2] << 8 | t[3]) >>> 0;
    });
    i(this, "readInt8", () => {
      this.start_reading();
      const t = new Uint8Array(1);
      t.set(this.inner.subarray(this.pos, this.pos + 1)), this.pos += 1;
      const r = t[0];
      return t[0] & 128 ? -r : r;
    });
    i(this, "readInt16", () => {
      this.start_reading();
      const t = new Uint8Array(2);
      t.set(this.inner.subarray(this.pos, this.pos + 2)), this.pos += 2;
      const r = t[0] << 8 | t[1];
      return t[0] & 128 ? -r : r;
    });
    i(this, "readInt32", () => {
      this.start_reading();
      const t = new Uint8Array(4);
      t.set(this.inner.subarray(this.pos, this.pos + 4)), this.pos += 4;
      const r = (t[0] << 24 | t[1] << 16 | t[2] << 8 | t[3]) >>> 0;
      return t[0] & 128 ? -r : r;
    });
    i(this, "readBool", () => {
      this.start_reading();
      const t = new Uint8Array(1);
      return t.set(this.inner.subarray(this.pos, this.pos + 1)), this.pos += 1, t[0] === 1;
    });
    i(this, "readPackedBools", () => []);
    i(this, "readString", () => {
      this.start_reading();
      const t = this.readUint16(), r = this.inner.subarray(this.pos, this.pos + t), s = new TextDecoder("utf-8").decode(r);
      return this.pos += t, s;
    });
    i(this, "readSizedString", (t) => {
      this.start_reading();
      const r = this.inner.subarray(this.pos, this.pos + t), s = new TextDecoder("utf-8").decode(r);
      return this.pos += t, s;
    });
    i(this, "readStringRemaining", () => {
      this.start_reading();
      const t = this.inner.subarray(this.pos, this.inner.length), r = new TextDecoder("utf-8").decode(t);
      return this.pos = this.inner.length, r;
    });
    i(this, "serialize", (t) => {
    });
    i(this, "serializeNumber", (t) => {
    });
    i(this, "serializeString", (t) => {
    });
    i(this, "serializeBoolean", (t) => {
    });
    this.inner = new Uint8Array(t), this.pos = 0, this.reading = !1, this.capacity = 1024;
  }
  write(t) {
    if (t.length > this.capacity || this.inner.length + t.length > this.capacity)
      throw new Error("Bufferfish is full");
    this.reading = !1;
    const r = new Uint8Array(this.inner.length + t.length);
    r.set(this.inner, 0), r.set(t, this.inner.length), this.inner = r;
    const s = t.length;
    return this.pos += s, s;
  }
  start_reading() {
    this.reading || (this.pos = 0, this.reading = !0);
  }
  set_max_capacity(t) {
    if (t < 1)
      throw new Error("Max capacity must be at least 1 byte");
    this.capacity = t;
  }
}
export {
  a as Bufferfish
};
