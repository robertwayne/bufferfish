import { expect, test } from "bun:test"
import { Bufferfish } from "./bufferfish.js"

test("should peek one byte", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)
    bf.writeUint8(255)

    expect(bf.peek()).toEqual(0)
    expect(bf.peek()).toEqual(0)
})

test("should peek two bytes", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)
    bf.writeUint8(255)

    expect(bf.peekN(2)).toEqual(new Uint8Array([0, 255]))
    expect(bf.peekN(2)).toEqual(new Uint8Array([0, 255]))
})

test("should peek one byte over", () => {
    const bf = new Bufferfish()

    expect(() => bf.peek()).toThrow(
        "peek of 1 byte exceeds the max capacity of 1024 bytes on this Bufferfish",
    )
})

test("should fail to peek too many bytes", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)
    bf.writeUint8(1)

    expect(() => bf.peekN(3)).toThrow(
        Error(
            "peek of 3 bytes exceeds the max capacity of 1024 bytes on this Bufferfish",
        ),
    )
})

test("should push another bufferfish", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)

    const buf2 = new Bufferfish()
    buf2.writeUint8(1)

    bf.push(buf2)

    expect(bf.view()).toEqual(new Uint8Array([0, 1]))
})

test("should push array-likes", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)

    bf.push([1])
    bf.push(new Uint8Array([2]))

    expect(bf.view()).toEqual(new Uint8Array([0, 1, 2]))
})

test("should write u8", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)
    bf.writeUint8(255)

    expect(bf.view()).toEqual(new Uint8Array([0, 255]))
})

test("should write u16", () => {
    const bf = new Bufferfish()
    bf.writeUint16(0)
    bf.writeUint16(12345)
    bf.writeUint16(65535)

    expect(bf.view()).toEqual(new Uint8Array([0, 0, 48, 57, 255, 255]))
})

test("should write u32", () => {
    const bf = new Bufferfish()
    bf.writeUint32(0)
    bf.writeUint32(1234567890)
    bf.writeUint32(4294967295)

    expect(bf.view()).toEqual(
        new Uint8Array([0, 0, 0, 0, 73, 150, 2, 210, 255, 255, 255, 255]),
    )
})

test("should read u8", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)
    bf.writeUint8(255)

    expect(bf.readUint8()).toEqual(0)
    expect(bf.readUint8()).toEqual(255)
})

test("should read u16", () => {
    const bf = new Bufferfish()
    bf.writeUint16(0)
    bf.writeUint16(12345)
    bf.writeUint16(65535)

    expect(bf.readUint16()).toEqual(0)
    expect(bf.readUint16()).toEqual(12345)
    expect(bf.readUint16()).toEqual(65535)
})

test("should read u32", () => {
    const bf = new Bufferfish()
    bf.writeUint32(0)
    bf.writeUint32(1234567890)
    bf.writeUint32(4294967295)

    expect(bf.readUint32()).toEqual(0)
    expect(bf.readUint32()).toEqual(1234567890)
    expect(bf.readUint32()).toEqual(4294967295)
})

test("should write i8", () => {
    const bf = new Bufferfish()
    bf.writeInt8(0)
    bf.writeInt8(127)
    bf.writeInt8(-128)

    expect(bf.view()).toEqual(new Uint8Array([0, 127, 128]))
})

test("should write i16", () => {
    const bf = new Bufferfish()
    bf.writeInt16(0)
    bf.writeInt16(12345)
    bf.writeInt16(32767)
    bf.writeInt16(-32768)

    expect(bf.view()).toEqual(new Uint8Array([0, 0, 48, 57, 127, 255, 128, 0]))
})

test("should write i32", () => {
    const bf = new Bufferfish()
    bf.writeInt32(0)
    bf.writeInt32(1234567890)
    bf.writeInt32(2147483647)
    bf.writeInt32(-2147483648)

    expect(bf.view()).toEqual(
        new Uint8Array([
            0, 0, 0, 0, 73, 150, 2, 210, 127, 255, 255, 255, 128, 0, 0, 0,
        ]),
    )
})

test("should read i8", () => {
    const bf = new Bufferfish()
    bf.writeInt8(0)
    bf.writeInt8(127)
    bf.writeInt8(-128)

    expect(bf.readInt8()).toEqual(0)
    expect(bf.readInt8()).toEqual(127)
    expect(bf.readInt8()).toEqual(-128)
})

test("should read i16", () => {
    const bf = new Bufferfish()
    bf.writeInt16(0)
    bf.writeInt16(12345)
    bf.writeInt16(32767)
    bf.writeInt16(-32768)

    expect(bf.readInt16()).toEqual(0)
    expect(bf.readInt16()).toEqual(12345)
    expect(bf.readInt16()).toEqual(32767)
    expect(bf.readInt16()).toEqual(-32768)
})

test("should read i32", () => {
    const bf = new Bufferfish()
    bf.writeInt32(0)
    bf.writeInt32(1234567890)
    bf.writeInt32(2147483647)
    bf.writeInt32(-2147483648)
    bf.writeInt32(-1)

    expect(bf.readInt32()).toEqual(0)
    expect(bf.readInt32()).toEqual(1234567890)
    expect(bf.readInt32()).toEqual(2147483647)
    expect(bf.readInt32()).toEqual(-2147483648)
    expect(bf.readInt32()).toEqual(-1)
})

test("should read from reset position", () => {
    const bf = new Bufferfish()
    bf.writeUint8(0)
    bf.readUint8()
    bf.writeUint8(255)

    expect(bf.readUint8()).toEqual(0)
})

test("should return overflow error", () => {
    const bf = new Bufferfish()

    for (let i = 0; i < 1024; i++) {
        bf.writeUint8(0)
    }

    const err = bf.writeUint32(0)
    expect(err).toEqual(Error(`Bufferfish capacity exceeded (1024 bytes)`))
})

test("should be unbounded", () => {
    const bf = new Bufferfish()
    bf.setMaxCapacity(0)

    expect(() => {
        for (let i = 0; i < 2000; i++) {
            bf.writeUint8(0)
        }
    }).not.toThrow("Bufferfish is full")
})

test("should write string", () => {
    const bf = new Bufferfish()
    bf.writeString("Bufferfish")

    expect(bf.view()).toEqual(
        new Uint8Array([
            0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104,
        ]),
    )
})

test("should write string big chars", () => {
    const bf = new Bufferfish()
    bf.writeString("안녕하세요")

    expect(bf.view()).toEqual(
        new Uint8Array([
            0, 15, 236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184,
            236, 154, 148,
        ]),
    )
})

test("should write multiple strings", () => {
    const bf = new Bufferfish()
    bf.writeString("Bufferfish")
    bf.writeString("안녕하세요")

    expect(bf.view()).toEqual(
        new Uint8Array([
            0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 0, 15, 236,
            149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154,
            148,
        ]),
    )
})

test("should read string", () => {
    const bf = new Bufferfish()
    bf.writeString("Bufferfish")

    expect(bf.readString()).toEqual("Bufferfish")
})

test("should write bool", () => {
    const bf = new Bufferfish()
    bf.writeBool(true)
    bf.writeBool(false)

    expect(bf.view()).toEqual(new Uint8Array([1, 0]))
})

test("should write full packed bools", () => {
    const bf = new Bufferfish()
    bf.writePackedBools([true, false, true, true, false, false, true, false])

    expect(bf.view()).toEqual(new Uint8Array([0b10110010]))
})

test("should write partial packed bools", () => {
    const bf = new Bufferfish()
    bf.writePackedBools([true, false])

    expect(bf.view()).toEqual(new Uint8Array([0b10000000]))
})

test("should read bool", () => {
    const bf = new Bufferfish()
    bf.writeBool(true)
    bf.writeBool(false)

    expect(bf.readBool()).toEqual(true)
    expect(bf.readBool()).toEqual(false)
})

test("should read full packed bools", () => {
    const bf = new Bufferfish()
    bf.writePackedBools([true, false, true, true, false, false, true, false])

    expect(bf.readPackedBools()).toEqual([
        true,
        false,
        true,
        true,
        false,
        false,
        true,
        false,
    ])
})

test("should read partial packed bools", () => {
    const bf = new Bufferfish()
    bf.writePackedBools([true, false])

    expect(bf.readPackedBools(2)).toEqual([true, false])
})

test("should write raw bytes", () => {
    const bf = new Bufferfish()
    bf.writeString("Bufferfish")

    const buf2 = new Bufferfish()
    buf2.writeString("안녕하세요")

    bf.writeRawBytes(buf2.view())

    expect(bf.readString()).toEqual("Bufferfish")
    expect(bf.readString()).toEqual("안녕하세요")
})

test("should return error on out-of-bounds read", () => {
    const bf = new Bufferfish()

    const err1 = bf.readUint8()
    const err2 = bf.readUint16()
    const err3 = bf.readUint32()
    const err4 = bf.readInt8()
    const err5 = bf.readInt16()
    const err6 = bf.readInt32()
    const err7 = bf.readBool()
    const err8 = bf.readPackedBools()
    const err9 = bf.readString()

    for (const err of [err1, err2, err3, err4, err5, err6, err7, err8, err9]) {
        expect(err).toEqual(
            Error("attempted to read past the end of the Bufferfish"),
        )
    }
})

test("should return error on out-of-range write", () => {
    const bf = new Bufferfish()
    const err1 = bf.writeUint8(256)
    const err2 = bf.writeUint16(65536)
    const err3 = bf.writeUint32(4294967296)
    const err4 = bf.writeInt8(128)
    const err5 = bf.writeInt16(32768)
    const err6 = bf.writeInt32(2147483648)

    expect(err1).toEqual(Error(`value 256 must be between 0 and 255`))
    expect(err2).toEqual(Error(`value 65536 must be between 0 and 65535`))
    expect(err3).toEqual(
        Error(`value 4294967296 must be between 0 and 4294967295`),
    )
    expect(err4).toEqual(Error(`value 128 must be between -128 and 127`))
    expect(err5).toEqual(Error(`value 32768 must be between -32768 and 32767`))
    expect(err6).toEqual(
        Error(`value 2147483648 must be between -2147483648 and 2147483647`),
    )
})

test("should return error on more than 8 packed bools", () => {
    const bf = new Bufferfish()
    const err = bf.writePackedBools([
        true,
        true,
        true,
        true,
        true,
        true,
        true,
        true,
        true,
    ])

    expect(err).toEqual(
        Error("cannot pack more than 8 bools into a single byte"),
    )
})

test("should write and read arrays of u8", () => {
    const bf = new Bufferfish()

    const numbers = [1, 2, 3, 4, 5]
    bf.writeArray(numbers, (n) => bf.writeUint8(n))

    expect(bf.view()).toEqual(new Uint8Array([0, 5, 1, 2, 3, 4, 5]))

    const readArray = bf.readArray(() => {
        const val = bf.readUint8()
        if (val instanceof Error) {
            throw val
        }
        return val
    })

    expect(readArray).toEqual(numbers)
})

test("should write and read arrays of strings", () => {
    const bf = new Bufferfish()

    const strings = ["hello", "world", "bufferfish"]
    bf.writeArray(strings, (s) => bf.writeString(s))

    const readArray = bf.readArray(() => {
        const val = bf.readString()
        if (val instanceof Error) {
            throw val
        }
        return val
    })

    expect(readArray).toEqual(strings)
})

test("should handle empty arrays", () => {
    const bf = new Bufferfish()

    const emptyArray: number[] = []
    bf.writeArray(emptyArray, (n) => bf.writeUint8(n))

    expect(bf.view()).toEqual(new Uint8Array([0, 0]))

    const readArray = bf.readArray(() => {
        const val = bf.readUint8()
        if (val instanceof Error) {
            throw val
        }
        return val
    })

    expect(readArray).toEqual(emptyArray)
})

test("should return error for arrays exceeding maximum length", () => {
    const bf = new Bufferfish()

    const hugeArray = new Array(65536).fill(1)

    const err = bf.writeArray(hugeArray, (n) => bf.writeUint8(n))
    expect(err).toEqual(
        Error("array length 65536 exceeds maximum size of 65535"),
    )
})

test("should read an array of objects implementing read methods", () => {
    const arr = [
        { id: 1, name: "Alice", key: { inner: 1 } },
        { id: 2, name: "Bob", key: { inner: 2 } },
        { id: 3, name: "Charlie", key: { inner: 3 } },
    ]

    const decodeKey = (bf: Bufferfish) => {
        const inner = bf.readUint8()
        if (inner instanceof Error) {
            throw inner
        }
        return { inner }
    }

    const decodePerson = (bf: Bufferfish) => {
        const id = bf.readUint8()
        const name = bf.readString()
        const key = decodeKey(bf)
        if (id instanceof Error || name instanceof Error) {
            throw id instanceof Error ? id : name
        }
        return { id, name, key }
    }

    const bf = new Bufferfish()

    bf.writeArray(arr, (person) => {
        bf.writeUint8(person.id)
        bf.writeString(person.name)
        bf.writeUint8(person.key.inner)
    })

    const people = bf.readArray(() => decodePerson(bf))
    expect(people).toEqual(arr)
})
