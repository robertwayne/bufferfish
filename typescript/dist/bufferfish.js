// src/bufferfish.ts
var OVERFLOW_ERR = "attempted to read past the end of the Bufferfish";

class Bufferfish {
  inner;
  position;
  reading;
  maxCapacity;
  textDecoder;
  textEncoder;
  constructor(bf = new ArrayBuffer(0)) {
    this.inner = new Uint8Array(bf);
    this.position = 0;
    this.reading = false;
    this.maxCapacity = 1024;
    this.textDecoder = undefined;
    this.textEncoder = undefined;
  }
  write(bf) {
    if (this.maxCapacity > 0 && (bf.length > this.maxCapacity || this.inner.length + bf.length > this.maxCapacity)) {
      return new Error(`Bufferfish capacity exceeded (${this.maxCapacity} bytes)`);
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
  bytes = () => {
    return this.inner.subarray();
  };
  startReading = () => {
    if (this.reading) {
      return;
    }
    this.position = 0;
    this.reading = true;
  };
  setMaxCapacity = (capacity) => {
    this.maxCapacity = capacity;
  };
  isEmpty = () => {
    return this.inner.length === 0;
  };
  length = () => {
    return this.inner.length;
  };
  reset = () => {
    this.inner = new Uint8Array(0);
    this.position = 0;
    this.reading = false;
  };
  peek = () => {
    this.startReading();
    const value = this.inner.slice(this.position, this.position + 1)[0];
    if (this.position >= this.inner.length || value === undefined) {
      return new Error(`peek of 1 byte exceeds the max capacity of ${this.maxCapacity} bytes on this Bufferfish`);
    }
    return value;
  };
  peekN = (n) => {
    this.startReading();
    const value = this.inner.slice(this.position, this.position + n);
    if (this.position + n > this.inner.length) {
      return new Error(`peek of ${n} bytes exceeds the max capacity of ${this.maxCapacity} bytes on this Bufferfish`);
    }
    return value;
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
  writeUint64 = (value) => {
    if (value > BigInt("18446744073709551615") || value < BigInt(0)) {
      return new Error(`value ${value} must be between 0 and 18446744073709551615`);
    }
    const slice = new Uint8Array(8);
    const view = new DataView(slice.buffer);
    view.setBigUint64(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeUint128 = (value) => {
    if (value > BigInt("340282366920938463463374607431768211455") || value < BigInt(0)) {
      return new Error(`value ${value} must be between 0 and 340282366920938463463374607431768211455`);
    }
    const slice = new Uint8Array(16);
    const view = new DataView(slice.buffer);
    view.setBigUint64(0, value >> 64n);
    view.setBigUint64(8, value & BigInt("0xffffffffffffffff"));
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
  writeInt64 = (value) => {
    if (value > BigInt("9223372036854775807") || value < BigInt("-9223372036854775808")) {
      return new Error(`value ${value} must be between -9223372036854775808 and 9223372036854775807`);
    }
    const slice = new Uint8Array(8);
    const view = new DataView(slice.buffer);
    view.setBigInt64(0, value);
    const err = this.write(slice);
    if (err instanceof Error) {
      return err;
    }
  };
  writeInt128 = (value) => {
    if (value > BigInt("170141183460469231731687303715884105727") || value < BigInt("-170141183460469231731687303715884105728")) {
      return new Error(`value ${value} must be between -170141183460469231731687303715884105728 and 170141183460469231731687303715884105727`);
    }
    const slice = new Uint8Array(16);
    const view = new DataView(slice.buffer);
    let unsignedValue = value;
    if (value < 0n) {
      unsignedValue = (1n << 128n) + value;
    }
    view.setBigUint64(0, unsignedValue >> 64n);
    view.setBigUint64(8, unsignedValue & BigInt("0xffffffffffffffff"));
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
  writeArray = (values, writeFn) => {
    if (values.length > 65535) {
      return new Error(`array length ${values.length} exceeds maximum size of 65535`);
    }
    const err = this.writeUint16(values.length);
    if (err instanceof Error) {
      return err;
    }
    for (const value of values) {
      const err2 = writeFn(value);
      if (err2 instanceof Error) {
        return err2;
      }
    }
  };
  readUint8 = () => {
    this.startReading();
    if (this.position + 1 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 1).getUint8(0);
    this.position += 1;
    return value;
  };
  readUint16 = () => {
    this.startReading();
    if (this.position + 2 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 2).getUint16(0);
    this.position += 2;
    return value;
  };
  readUint32 = () => {
    this.startReading();
    if (this.position + 4 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 4).getUint32(0);
    this.position += 4;
    return value;
  };
  readUint64 = () => {
    this.startReading();
    if (this.position + 8 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 8).getBigUint64(0);
    this.position += 8;
    return value;
  };
  readUint128 = () => {
    this.startReading();
    if (this.position + 16 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const high = new DataView(this.inner.buffer, this.position, 8).getBigUint64(0);
    const low = new DataView(this.inner.buffer, this.position + 8, 8).getBigUint64(0);
    this.position += 16;
    return high << 64n | low;
  };
  readInt8 = () => {
    this.startReading();
    if (this.position + 1 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 1).getInt8(0);
    this.position += 1;
    return value;
  };
  readInt16 = () => {
    this.startReading();
    if (this.position + 2 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 2).getInt16(0);
    this.position += 2;
    return value;
  };
  readInt32 = () => {
    this.startReading();
    if (this.position + 4 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 4).getInt32(0);
    this.position += 4;
    return value;
  };
  readInt64 = () => {
    this.startReading();
    if (this.position + 8 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const value = new DataView(this.inner.buffer, this.position, 8).getBigInt64(0);
    this.position += 8;
    return value;
  };
  readInt128 = () => {
    this.startReading();
    if (this.position + 16 > this.inner.length) {
      return new Error(OVERFLOW_ERR);
    }
    const high = new DataView(this.inner.buffer, this.position, 8).getBigUint64(0);
    const low = new DataView(this.inner.buffer, this.position + 8, 8).getBigUint64(0);
    this.position += 16;
    let value = high << 64n | low;
    if (value >> 127n === 1n) {
      value = value - (1n << 128n);
    }
    return value;
  };
  readBool = () => {
    const valueOrError = this.readUint8();
    if (valueOrError instanceof Error) {
      return valueOrError;
    }
    return valueOrError === 1;
  };
  readPackedBools = (count = 8) => {
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
  };
  readString = () => {
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
  };
  readArray = (readFn) => {
    const lengthOrError = this.readUint16();
    if (lengthOrError instanceof Error) {
      return lengthOrError;
    }
    const length = lengthOrError;
    const values = new Array(length);
    try {
      for (let i = 0;i < length; i++) {
        const valueOrError = readFn();
        if (valueOrError instanceof Error) {
          return valueOrError;
        }
        values[i] = valueOrError;
      }
      return values;
    } catch (error) {
      return error instanceof Error ? error : new Error(String(error));
    }
  };
}
export {
  Bufferfish
};
