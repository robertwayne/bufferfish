// src/bufferfish.ts
var OVERFLOW_ERR = "attempted to read past the end of the Bufferfish";

class Bufferfish {
  inner;
  position;
  reading;
  capacity;
  textDecoder;
  textEncoder;
  constructor(bf = new ArrayBuffer(0)) {
    this.inner = new Uint8Array(bf);
    this.position = 0;
    this.reading = false;
    this.capacity = 1024;
    this.textDecoder = undefined;
    this.textEncoder = undefined;
  }
  write(bf) {
    if (this.capacity > 0 && (bf.length > this.capacity || this.inner.length + bf.length > this.capacity)) {
      return new Error(`Bufferfish capacity exceeded (${this.capacity} bytes)`);
    }
    this.reading = false;
    const tmp = new Uint8Array(this.inner.length + bf.length);
    tmp.set(this.inner, 0);
    tmp.set(bf, this.inner.length);
    this.inner = tmp;
    const bytesWritten = bf.length;
    this.position += bytesWritten;
    return bytesWritten;
  }
  view = () => {
    return this.inner.slice();
  };
  startReading() {
    if (this.reading) {
      return;
    }
    this.position = 0;
    this.reading = true;
  }
  setMaxCapacity(capacity) {
    this.capacity = capacity;
  }
  peek = () => {
    this.startReading();
    const value = this.inner.slice(this.position, this.position + 1)[0];
    if (this.position >= this.inner.length || value === undefined) {
      return new Error(`peek of 1 byte exceeds the max capacity of ${this.capacity} bytes on this Bufferfish`);
    }
    return value;
  };
  peekN = (n) => {
    this.startReading();
    const value = this.inner.slice(this.position, this.position + n);
    if (this.position + n > this.inner.length) {
      return new Error(`peek of ${n} bytes exceeds the max capacity of ${this.capacity} bytes on this Bufferfish`);
    }
    return value;
  };
  push = (arr) => {
    if (arr instanceof Bufferfish) {
      const err = this.write(arr.view());
      if (err instanceof Error) {
        return err;
      }
    } else if (arr instanceof Uint8Array) {
      const err = this.write(arr);
      if (err instanceof Error) {
        return err;
      }
    } else if (arr instanceof ArrayBuffer) {
      const err = this.write(new Uint8Array(arr));
      if (err instanceof Error) {
        return err;
      }
    } else if (arr instanceof Array) {
      const err = this.write(new Uint8Array(arr));
      if (err instanceof Error) {
        return err;
      }
    } else {
      return new Error("invalid type");
    }
  };
  writeUint8 = (value) => {
    if (value > 255 || value < 0) {
      return new Error(`value ${value} must be between 0 and 255`);
    }
    const slice = new Uint8Array(1);
    const view = new DataView(slice.buffer);
    view.setUint8(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeUint16 = (value) => {
    if (value > 65535 || value < 0) {
      return new Error(`value ${value} must be between 0 and 65535`);
    }
    const slice = new Uint8Array(2);
    const view = new DataView(slice.buffer);
    view.setUint16(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeUint32 = (value) => {
    if (value > 4294967295 || value < 0) {
      return new Error(`value ${value} must be between 0 and 4294967295`);
    }
    const slice = new Uint8Array(4);
    const view = new DataView(slice.buffer);
    view.setUint32(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeInt8 = (value) => {
    if (value > 127 || value < -128) {
      return new Error(`value ${value} must be between -128 and 127`);
    }
    const slice = new Uint8Array(1);
    const view = new DataView(slice.buffer);
    view.setInt8(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeInt16 = (value) => {
    if (value > 32767 || value < -32768) {
      return new Error(`value ${value} must be between -32768 and 32767`);
    }
    const slice = new Uint8Array(2);
    const view = new DataView(slice.buffer);
    view.setInt16(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeInt32 = (value) => {
    if (value > 2147483647 || value < -2147483648) {
      return new Error(`value ${value} must be between -2147483648 and 2147483647`);
    }
    const slice = new Uint8Array(4);
    const view = new DataView(slice.buffer);
    view.setInt32(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeBool = (value) => {
    const err = this.writeUint8(value ? 1 : 0);
    if (err instanceof Error) {
      return err;
    }
  };
  writePackedBools = (values) => {
    if (values.length > 8) {
      return new Error("cannot pack more than 8 bools into a single byte");
    }
    let packedValue = 0;
    for (const value of values) {
      packedValue <<= 1;
      if (value) {
        packedValue |= 1;
      }
    }
    packedValue <<= 8 - values.length;
    const err = this.writeUint8(packedValue);
    if (err instanceof Error) {
      return err;
    }
  };
  writeString = (value) => {
    if (!this.textEncoder)
      this.textEncoder = new TextEncoder;
    const slice = this.textEncoder.encode(value);
    const err = this.writeUint16(slice.length);
    if (err instanceof Error) {
      return err;
    }
    const err2 = this.write(slice);
    if (err2 instanceof Error) {
      return err2;
    }
  };
  writeRawBytes = (value) => {
    const err = this.write(value);
    if (err instanceof Error) {
      return err;
    }
  };
  readUint8() {
    this.startReading();
    if (this.position + 1 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 1).getUint8(0);
    this.position += 1;
    return value;
  }
  readUint16() {
    this.startReading();
    if (this.position + 2 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 2).getUint16(0);
    this.position += 2;
    return value;
  }
  readUint32() {
    this.startReading();
    if (this.position + 4 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 4).getUint32(0);
    this.position += 4;
    return value;
  }
  readInt8() {
    this.startReading();
    if (this.position + 1 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 1).getInt8(0);
    this.position += 1;
    return value;
  }
  readInt16() {
    this.startReading();
    if (this.position + 2 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 2).getInt16(0);
    this.position += 2;
    return value;
  }
  readInt32() {
    this.startReading();
    if (this.position + 4 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 4).getInt32(0);
    this.position += 4;
    return value;
  }
  readBool() {
    const valueOrError = this.readUint8();
    if (valueOrError instanceof Error) {
      return valueOrError;
    }
    return valueOrError === 1;
  }
  readPackedBools(count = 8) {
    if (count > 8) {
      return new Error("cannot read more than 8 bools from a single byte");
    }
    const packedValueOrError = this.readUint8();
    if (packedValueOrError instanceof Error) {
      return packedValueOrError;
    }
    const packedValue = packedValueOrError;
    const bools = [];
    for (let i = 0;i < count; i++) {
      bools.push((packedValue & 1 << 7 - i) !== 0);
    }
    return bools;
  }
  readString() {
    const lengthOrError = this.readUint16();
    if (lengthOrError instanceof Error) {
      return lengthOrError;
    }
    const length = lengthOrError;
    if (this.position + length > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    if (!this.textDecoder)
      this.textDecoder = new TextDecoder("utf-8");
    const value = this.textDecoder.decode(this.inner.subarray(this.position, this.position + length));
    this.position += length;
    return value;
  }
  readArray(readFn) {
    const lengthOrError = this.readUint16();
    if (lengthOrError instanceof Error) {
      return lengthOrError;
    }
    const length = lengthOrError;
    const values = [];
    for (let i = 0;i < length; i++) {
      const valueOrError = readFn();
      if (valueOrError instanceof Error) {
        return valueOrError;
      }
      values.push(valueOrError);
    }
    return values;
  }
}
export {
  Bufferfish
};
