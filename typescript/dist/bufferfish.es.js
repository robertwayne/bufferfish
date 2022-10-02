var __defProp = Object.defineProperty;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => {
  __defNormalProp(obj, typeof key !== "symbol" ? key + "" : key, value);
  return value;
};
class Bufferfish {
  constructor(buf = new ArrayBuffer(0)) {
    __publicField(this, "inner");
    __publicField(this, "pos");
    __publicField(this, "reading");
    __publicField(this, "capacity");
    __publicField(this, "view", () => {
      return this.inner.slice();
    });
    __publicField(this, "writeUint8", (value) => {
      if (value > 255 || value < 0) {
        throw new Error("Value is out of range");
      }
      this.write(new Uint8Array([value]));
    });
    __publicField(this, "writeUint16", (value) => {
      if (value > 65535 || value < 0) {
        throw new Error("Value is out of range");
      }
      this.write(new Uint8Array([value >> 8, value & 255]));
    });
    __publicField(this, "writeUint32", (value) => {
      if (value > 4294967295 || value < 0) {
        throw new Error("Value is out of range");
      }
      this.write(
        new Uint8Array([
          value >> 24,
          value >> 16 & 255,
          value >> 8 & 255,
          value & 255
        ])
      );
    });
    __publicField(this, "writeInt8", (value) => {
      if (value > 127 || value < -128) {
        throw new Error("Value is out of range");
      }
      this.write(new Uint8Array([value]));
    });
    __publicField(this, "writeInt16", (value) => {
      if (value > 32767 || value < -32768) {
        throw new Error("Value is out of range");
      }
      this.write(new Uint8Array([value >> 8, value & 255]));
    });
    __publicField(this, "writeInt32", (value) => {
      if (value > 2147483647 || value < -2147483648) {
        throw new Error("Value is out of range");
      }
      this.write(
        new Uint8Array([
          value >> 24,
          value >> 16 & 255,
          value >> 8 & 255,
          value & 255
        ])
      );
    });
    __publicField(this, "writeBool", (value) => {
      this.writeUint8(value ? 1 : 0);
    });
    __publicField(this, "writePackedBools", (values) => {
      if (values.length > 4) {
        throw new Error(
          "Each packed bool can only represent 4 or fewer values"
        );
      }
      let packed_value = 0;
      for (const value of values) {
        packed_value <<= 1;
        if (value) {
          packed_value |= 1;
        }
      }
      this.writeUint8(packed_value);
    });
    __publicField(this, "writeString", (value) => {
      const slice = new TextEncoder().encode(value);
      this.writeUint16(slice.length);
      this.write(slice);
    });
    __publicField(this, "writeSizedString", (value) => {
      const slice = new TextEncoder().encode(value);
      this.write(slice);
    });
    __publicField(this, "readUint8", () => {
      this.start_reading();
      const buf = new Uint8Array(1);
      buf.set(this.inner.subarray(this.pos, this.pos + 1));
      this.pos += 1;
      return buf[0];
    });
    __publicField(this, "readUint16", () => {
      this.start_reading();
      const buf = new Uint8Array(2);
      buf.set(this.inner.subarray(this.pos, this.pos + 2));
      this.pos += 2;
      return buf[0] << 8 | buf[1];
    });
    __publicField(this, "readUint32", () => {
      this.start_reading();
      const buf = new Uint8Array(4);
      buf.set(this.inner.subarray(this.pos, this.pos + 4));
      this.pos += 4;
      return (buf[0] << 24 | buf[1] << 16 | buf[2] << 8 | buf[3]) >>> 0;
    });
    __publicField(this, "readInt8", () => {
      this.start_reading();
      const buf = new Uint8Array(1);
      buf.set(this.inner.subarray(this.pos, this.pos + 1));
      this.pos += 1;
      const value = buf[0];
      return buf[0] & 128 ? -value : value;
    });
    __publicField(this, "readInt16", () => {
      this.start_reading();
      const buf = new Uint8Array(2);
      buf.set(this.inner.subarray(this.pos, this.pos + 2));
      this.pos += 2;
      const value = buf[0] << 8 | buf[1];
      return buf[0] & 128 ? -value : value;
    });
    __publicField(this, "readInt32", () => {
      this.start_reading();
      const buf = new Uint8Array(4);
      buf.set(this.inner.subarray(this.pos, this.pos + 4));
      this.pos += 4;
      const value = (buf[0] << 24 | buf[1] << 16 | buf[2] << 8 | buf[3]) >>> 0;
      return buf[0] & 128 ? -value : value;
    });
    __publicField(this, "readBool", () => {
      this.start_reading();
      const buf = new Uint8Array(1);
      buf.set(this.inner.subarray(this.pos, this.pos + 1));
      this.pos += 1;
      return buf[0] === 1;
    });
    __publicField(this, "readPackedBools", () => {
      return [];
    });
    __publicField(this, "readString", () => {
      this.start_reading();
      const len = this.readUint16();
      const slice = this.inner.subarray(this.pos, this.pos + len);
      const str = new TextDecoder("utf-8").decode(slice);
      this.pos += len;
      return str;
    });
    __publicField(this, "readSizedString", (size) => {
      this.start_reading();
      const slice = this.inner.subarray(this.pos, this.pos + size);
      const str = new TextDecoder("utf-8").decode(slice);
      this.pos += size;
      return str;
    });
    __publicField(this, "readStringRemaining", () => {
      this.start_reading();
      const slice = this.inner.subarray(this.pos, this.inner.length);
      const str = new TextDecoder("utf-8").decode(slice);
      this.pos = this.inner.length;
      return str;
    });
    __publicField(this, "serialize", (obj) => {
    });
    __publicField(this, "serializeNumber", (number) => {
    });
    __publicField(this, "serializeString", (string) => {
    });
    __publicField(this, "serializeBoolean", (bool) => {
    });
    this.inner = new Uint8Array(buf);
    this.pos = 0;
    this.reading = false;
    this.capacity = 1024;
  }
  write(buf) {
    if (buf.length > this.capacity || this.inner.length + buf.length > this.capacity) {
      throw new Error("Bufferfish is full");
    }
    this.reading = false;
    const tmp = new Uint8Array(this.inner.length + buf.length);
    tmp.set(this.inner, 0);
    tmp.set(buf, this.inner.length);
    this.inner = tmp;
    const bytesWritten = buf.length;
    this.pos += bytesWritten;
    return bytesWritten;
  }
  start_reading() {
    if (this.reading) {
      return;
    }
    this.pos = 0;
    this.reading = true;
  }
  set_max_capacity(capacity) {
    if (capacity < 1) {
      throw new Error("Max capacity must be at least 1 byte");
    }
    this.capacity = capacity;
  }
}
export {
  Bufferfish
};
