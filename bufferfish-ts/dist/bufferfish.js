// src/bufferfish.ts
class Bufferfish {
  inner;
  pos;
  reading;
  capacity;
  constructor(buf = new ArrayBuffer(0)) {
    this.inner = new Uint8Array(buf);
    this.pos = 0;
    this.reading = false;
    this.capacity = 1024;
  }
  write(buf) {
    if (this.capacity > 0 && (buf.length > this.capacity || this.inner.length + buf.length > this.capacity)) {
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
  view = () => {
    return this.inner.slice();
  };
  startReading() {
    if (this.reading) {
      return;
    }
    this.pos = 0;
    this.reading = true;
  }
  setMaxCapacity(capacity) {
    this.capacity = capacity;
  }
  peek = () => {
    this.startReading();
    if (this.pos >= this.inner.length) {
      return null;
    }
    return this.inner.slice(this.pos, this.pos + 1)[0] ?? null;
  };
  peekN = (n) => {
    this.startReading();
    if (this.pos + n > this.inner.length) {
      return null;
    }
    return this.inner.slice(this.pos, this.pos + n);
  };
  push = (arr) => {
    if (arr instanceof Bufferfish) {
      this.write(arr.view());
    } else if (arr instanceof Uint8Array) {
      this.write(arr);
    } else if (arr instanceof ArrayBuffer) {
      this.write(new Uint8Array(arr));
    } else if (arr instanceof Array) {
      this.write(new Uint8Array(arr));
    } else {
      throw new Error("Invalid type");
    }
  };
  writeUint8 = (value) => {
    if (value > 255 || value < 0) {
      throw new Error("Value is out of range");
    }
    const slice = new Uint8Array(1);
    const view = new DataView(slice.buffer);
    view.setUint8(0, value);
    this.write(slice);
  };
  writeUint16 = (value) => {
    if (value > 65535 || value < 0) {
      throw new Error("Value is out of range");
    }
    const slice = new Uint8Array(2);
    const view = new DataView(slice.buffer);
    view.setUint16(0, value);
    this.write(slice);
  };
  writeUint32 = (value) => {
    if (value > 4294967295 || value < 0) {
      throw new Error("Value is out of range");
    }
    const slice = new Uint8Array(4);
    const view = new DataView(slice.buffer);
    view.setUint32(0, value);
    this.write(slice);
  };
  writeInt8 = (value) => {
    if (value > 127 || value < -128) {
      throw new Error("Value is out of range");
    }
    const slice = new Uint8Array(1);
    const view = new DataView(slice.buffer);
    view.setInt8(0, value);
    this.write(slice);
  };
  writeInt16 = (value) => {
    if (value > 32767 || value < -32768) {
      throw new Error("Value is out of range");
    }
    const slice = new Uint8Array(2);
    const view = new DataView(slice.buffer);
    view.setInt16(0, value);
    this.write(slice);
  };
  writeInt32 = (value) => {
    if (value > 2147483647 || value < -2147483648) {
      throw new Error("Value is out of range");
    }
    const slice = new Uint8Array(4);
    const view = new DataView(slice.buffer);
    view.setInt32(0, value);
    this.write(slice);
  };
  writeBool = (value) => {
    this.writeUint8(value ? 1 : 0);
  };
  writePackedBools = (values) => {
    if (values.length > 4) {
      throw new Error("Each packed bool can only represent 4 or fewer values");
    }
    let packed_value = 0;
    for (const value of values) {
      packed_value <<= 1;
      if (value) {
        packed_value |= 1;
      }
    }
    this.writeUint8(packed_value);
  };
  writeString = (value) => {
    const slice = new TextEncoder().encode(value);
    this.writeUint16(slice.length);
    this.write(slice);
  };
  writeSizedString = (value) => {
    const slice = new TextEncoder().encode(value);
    this.write(slice);
  };
  writeRawBytes = (value) => {
    this.write(value);
  };
  readUint8 = () => {
    this.startReading();
    const buf = new Uint8Array(1);
    buf.set(this.inner.subarray(this.pos, this.pos + 1));
    this.pos += 1;
    return new DataView(buf.buffer).getUint8(0);
  };
  readUint16 = () => {
    this.startReading();
    const buf = new Uint8Array(2);
    buf.set(this.inner.subarray(this.pos, this.pos + 2));
    this.pos += 2;
    return new DataView(buf.buffer).getUint16(0);
  };
  readUint32 = () => {
    this.startReading();
    const buf = new Uint8Array(4);
    buf.set(this.inner.subarray(this.pos, this.pos + 4));
    this.pos += 4;
    return new DataView(buf.buffer).getUint32(0);
  };
  readInt8 = () => {
    this.startReading();
    const buf = new Uint8Array(1);
    buf.set(this.inner.subarray(this.pos, this.pos + 1));
    this.pos += 1;
    return new DataView(buf.buffer).getInt8(0);
  };
  readInt16 = () => {
    this.startReading();
    const buf = new Uint8Array(2);
    buf.set(this.inner.subarray(this.pos, this.pos + 2));
    this.pos += 2;
    return new DataView(buf.buffer).getInt16(0);
  };
  readInt32 = () => {
    this.startReading();
    const buf = new Uint8Array(4);
    buf.set(this.inner.subarray(this.pos, this.pos + 4));
    this.pos += 4;
    return new DataView(buf.buffer).getInt32(0);
  };
  readBool = () => {
    this.startReading();
    const buf = new Uint8Array(1);
    buf.set(this.inner.subarray(this.pos, this.pos + 1));
    this.pos += 1;
    return buf[0] === 1;
  };
  readPackedBools = () => {
    return [];
  };
  readString = () => {
    this.startReading();
    const len = this.readUint16();
    const slice = this.inner.subarray(this.pos, this.pos + len);
    const str = new TextDecoder("utf-8").decode(slice);
    this.pos += len;
    return str;
  };
  readSizedString = (size) => {
    this.startReading();
    const slice = this.inner.subarray(this.pos, this.pos + size);
    const str = new TextDecoder("utf-8").decode(slice);
    this.pos += size;
    return str;
  };
  readStringRemaining = () => {
    this.startReading();
    const slice = this.inner.subarray(this.pos, this.inner.length);
    const str = new TextDecoder("utf-8").decode(slice);
    this.pos = this.inner.length;
    return str;
  };
}
export {
  Bufferfish
};
