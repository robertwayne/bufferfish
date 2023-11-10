import { expect, test } from "bun:test";
import { Bufferfish } from "./bufferfish.js";
test("should peek one byte", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    buf.writeUint8(255);
    expect(buf.peek()).toEqual(0);
    expect(buf.peek()).toEqual(0);
});
test("should peek two bytes", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    buf.writeUint8(255);
    expect(buf.peekN(2)).toEqual(new Uint8Array([0, 255]));
    expect(buf.peekN(2)).toEqual(new Uint8Array([0, 255]));
});
test("should peek one byte over", () => {
    const buf = new Bufferfish();
    const result = buf.peek();
    expect(result).toEqual(null);
});
test("should fail to peek too many bytes", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    buf.writeUint8(1);
    const result = buf.peekN(3);
    expect(result).toEqual(null);
});
test("should push another bufferfish", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    const buf2 = new Bufferfish();
    buf2.writeUint8(1);
    buf.push(buf2);
    expect(buf.view()).toEqual(new Uint8Array([0, 1]));
});
test("should push array-likes", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    buf.push([1]);
    buf.push(new Uint8Array([2]));
    expect(buf.view()).toEqual(new Uint8Array([0, 1, 2]));
});
test("should write u8", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    buf.writeUint8(255);
    expect(buf.view()).toEqual(new Uint8Array([0, 255]));
});
test("should write u16", () => {
    const buf = new Bufferfish();
    buf.writeUint16(0);
    buf.writeUint16(12345);
    buf.writeUint16(65535);
    expect(buf.view()).toEqual(new Uint8Array([0, 0, 48, 57, 255, 255]));
});
test("should write u32", () => {
    const buf = new Bufferfish();
    buf.writeUint32(0);
    buf.writeUint32(1234567890);
    buf.writeUint32(4294967295);
    expect(buf.view()).toEqual(new Uint8Array([0, 0, 0, 0, 73, 150, 2, 210, 255, 255, 255, 255]));
});
test("should read u8", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    buf.writeUint8(255);
    expect(buf.readUint8()).toEqual(0);
    expect(buf.readUint8()).toEqual(255);
});
test("should read u16", () => {
    const buf = new Bufferfish();
    buf.writeUint16(0);
    buf.writeUint16(12345);
    buf.writeUint16(65535);
    expect(buf.readUint16()).toEqual(0);
    expect(buf.readUint16()).toEqual(12345);
    expect(buf.readUint16()).toEqual(65535);
});
test("should read u32", () => {
    const buf = new Bufferfish();
    buf.writeUint32(0);
    buf.writeUint32(1234567890);
    buf.writeUint32(4294967295);
    expect(buf.readUint32()).toEqual(0);
    expect(buf.readUint32()).toEqual(1234567890);
    expect(buf.readUint32()).toEqual(4294967295);
});
test("should write i8", () => {
    const buf = new Bufferfish();
    buf.writeInt8(0);
    buf.writeInt8(127);
    buf.writeInt8(-128);
    expect(buf.view()).toEqual(new Uint8Array([0, 127, 128]));
});
test("should write i16", () => {
    const buf = new Bufferfish();
    buf.writeInt16(0);
    buf.writeInt16(12345);
    buf.writeInt16(32767);
    buf.writeInt16(-32768);
    expect(buf.view()).toEqual(new Uint8Array([0, 0, 48, 57, 127, 255, 128, 0]));
});
test("should write i32", () => {
    const buf = new Bufferfish();
    buf.writeInt32(0);
    buf.writeInt32(1234567890);
    buf.writeInt32(2147483647);
    buf.writeInt32(-2147483648);
    expect(buf.view()).toEqual(new Uint8Array([
        0, 0, 0, 0, 73, 150, 2, 210, 127, 255, 255, 255, 128, 0, 0, 0,
    ]));
});
test("should read i8", () => {
    const buf = new Bufferfish();
    buf.writeInt8(0);
    buf.writeInt8(127);
    buf.writeInt8(-128);
    expect(buf.readInt8()).toEqual(0);
    expect(buf.readInt8()).toEqual(127);
    expect(buf.readInt8()).toEqual(-128);
});
test("should read i16", () => {
    const buf = new Bufferfish();
    buf.writeInt16(0);
    buf.writeInt16(12345);
    buf.writeInt16(32767);
    buf.writeInt16(-32768);
    expect(buf.readInt16()).toEqual(0);
    expect(buf.readInt16()).toEqual(12345);
    expect(buf.readInt16()).toEqual(32767);
    expect(buf.readInt16()).toEqual(-32768);
});
test("should read i32", () => {
    const buf = new Bufferfish();
    buf.writeInt32(0);
    buf.writeInt32(1234567890);
    buf.writeInt32(2147483647);
    buf.writeInt32(-2147483648);
    buf.writeInt32(-1);
    expect(buf.readInt32()).toEqual(0);
    expect(buf.readInt32()).toEqual(1234567890);
    expect(buf.readInt32()).toEqual(2147483647);
    expect(buf.readInt32()).toEqual(-2147483648);
    expect(buf.readInt32()).toEqual(-1);
});
test("should read reset", () => {
    const buf = new Bufferfish();
    buf.writeUint8(0);
    buf.readUint8();
    buf.writeUint8(255);
    expect(buf.readUint8()).toEqual(0);
});
test("should overflow", () => {
    const buf = new Bufferfish();
    expect(() => {
        for (let i = 0; i < 1025; i++) {
            buf.writeUint8(0);
        }
    }).toThrow("Bufferfish is full");
});
test("should be unbounded", () => {
    const buf = new Bufferfish();
    buf.setMaxCapacity(0);
    expect(() => {
        for (let i = 0; i < 2000; i++) {
            buf.writeUint8(0);
        }
    }).not.toThrow("Bufferfish is full");
});
test("should write string", () => {
    const buf = new Bufferfish();
    buf.writeString("Bufferfish");
    expect(buf.view()).toEqual(new Uint8Array([
        0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104,
    ]));
});
test("should write string big chars", () => {
    const buf = new Bufferfish();
    buf.writeString("안녕하세요");
    expect(buf.view()).toEqual(new Uint8Array([
        0, 15, 236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184,
        236, 154, 148,
    ]));
});
test("should write multiple strings", () => {
    const buf = new Bufferfish();
    buf.writeString("Bufferfish");
    buf.writeString("안녕하세요");
    expect(buf.view()).toEqual(new Uint8Array([
        0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 0, 15, 236,
        149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154,
        148,
    ]));
});
test("should write fixed string", () => {
    const buf = new Bufferfish();
    buf.writeSizedString("Bufferfish");
    expect(buf.view()).toEqual(new Uint8Array([66, 117, 102, 102, 101, 114, 102, 105, 115, 104]));
});
test("should read string", () => {
    const buf = new Bufferfish();
    buf.writeString("Bufferfish");
    expect(buf.readString()).toEqual("Bufferfish");
});
test("should read sized string", () => {
    const buf = new Bufferfish();
    buf.writeSizedString("Bufferfish");
    expect(buf.readSizedString(10)).toEqual("Bufferfish");
});
test("should write bool", () => {
    const buf = new Bufferfish();
    buf.writeBool(true);
    buf.writeBool(false);
    expect(buf.view()).toEqual(new Uint8Array([1, 0]));
});
test("should write packed bools", () => {
    const buf = new Bufferfish();
    buf.writePackedBools([true, false, true, true]);
    buf.writePackedBools([false, false, true, false]);
    expect(buf.view()).toEqual(new Uint8Array([11, 2]));
});
test("should read bool", () => {
    const buf = new Bufferfish();
    buf.writeBool(true);
    buf.writeBool(false);
    expect(buf.readBool()).toEqual(true);
    expect(buf.readBool()).toEqual(false);
});
test("should write raw bytes", () => {
    const buf = new Bufferfish();
    buf.writeString("Bufferfish");
    const buf2 = new Bufferfish();
    buf2.writeString("안녕하세요");
    buf.writeRawBytes(buf2.view());
    expect(buf.readString()).toEqual("Bufferfish");
    expect(buf.readString()).toEqual("안녕하세요");
});
