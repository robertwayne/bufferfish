import { TextDecoder, TextEncoder } from "util"

export class Bufferfish {
    private inner: Uint8Array
    private pos: number
    private reading: boolean
    private capacity: number

    constructor(buf: ArrayBuffer = new ArrayBuffer(0)) {
        this.inner = new Uint8Array(buf)
        this.pos = 0
        this.reading = false
        this.capacity = 1024
    }

    /**
     * Writes a byte array to the internal buffer.
     *
     * This should only be called by the library.
     */
    private write(buf: Uint8Array): number {
        if (
            buf.length > this.capacity ||
            this.inner.length + buf.length > this.capacity
        ) {
            throw new Error("Bufferfish is full")
        }

        this.reading = false

        const tmp = new Uint8Array(this.inner.length + buf.length)
        tmp.set(this.inner, 0)
        tmp.set(buf, this.inner.length)
        this.inner = tmp

        const bytesWritten = buf.length
        this.pos += bytesWritten

        return bytesWritten
    }

    /**
     * Returns the (immutable) internal Uint8Array.
     */
    public view = (): Uint8Array => {
        return this.inner.slice()
    }

    /**
     * Resets the buffer cursor to the start postion when reading after a write.
     *
     * This should only be called by the library.
     */
    private start_reading() {
        if (this.reading) {
            return
        }

        this.pos = 0
        this.reading = true
    }

    /**
     * Sets the max capacity (in bytes) for the internal buffer.
     */
    public set_max_capacity(capacity: number) {
        if (capacity < 1) {
            throw new Error("Max capacity must be at least 1 byte")
        }

        this.capacity = capacity
    }

    /**
     * Writes a single u8 to the buffer as one byte.
     */
    public writeUint8 = (value: number) => {
        if (value > 255 || value < 0) {
            throw new Error("Value is out of range")
        }

        this.write(new Uint8Array([value]))
    }

    /**
     * Writes a u16 to the buffer as two bytes.
     */
    public writeUint16 = (value: number) => {
        if (value > 65535 || value < 0) {
            throw new Error("Value is out of range")
        }

        this.write(new Uint8Array([value >> 8, value & 0xff]))
    }

    /**
     * Writes a u32 to the buffer as four bytes.
     */
    public writeUint32 = (value: number) => {
        if (value > 4294967295 || value < 0) {
            throw new Error("Value is out of range")
        }

        this.write(
            new Uint8Array([
                value >> 24,
                (value >> 16) & 0xff,
                (value >> 8) & 0xff,
                value & 0xff,
            ])
        )
    }

    /**
     * Writes an i8 to the buffer as one byte.
     */
    public writeInt8 = (value: number) => {
        if (value > 127 || value < -128) {
            throw new Error("Value is out of range")
        }

        this.write(new Uint8Array([value]))
    }

    /**
     * Writes an i16 to the buffer as two bytes.
     */
    public writeInt16 = (value: number) => {
        if (value > 32767 || value < -32768) {
            throw new Error("Value is out of range")
        }

        this.write(new Uint8Array([value >> 8, value & 0xff]))
    }

    /**
     * Writes an i32 to the buffer as four bytes.
     */
    public writeInt32 = (value: number) => {
        if (value > 2147483647 || value < -2147483648) {
            throw new Error("Value is out of range")
        }

        this.write(
            new Uint8Array([
                value >> 24,
                (value >> 16) & 0xff,
                (value >> 8) & 0xff,
                value & 0xff,
            ])
        )
    }

    /**
     * Writes a bool to the buffer as a single byte.
     */
    public writeBool = (value: boolean) => {
        this.writeUint8(value ? 1 : 0)
    }

    /**
     * Writes a series of bools to the buffer as a single byte. This allows up to 4 bools to be
     * represented as a single byte. The first 4 bits are used as a mask to determine which of the
     * last 4 bits are set.
     */
    public writePackedBools = (values: Array<boolean>) => {
        if (values.length > 4) {
            throw new Error(
                "Each packed bool can only represent 4 or fewer values"
            )
        }

        let packed_value = 0x00
        for (const value of values) {
            packed_value <<= 1
            if (value) {
                packed_value |= 1
            }
        }

        this.writeUint8(packed_value)
    }

    /**
     * Writes a variable length string to the buffer. It will be prefixed with its length in bytes
     * as a u16 (two bytes).
     */
    public writeString = (value: string) => {
        const slice: Uint8Array = new TextEncoder().encode(value)

        this.writeUint16(slice.length)
        this.write(slice)
    }

    /**
     * Writes a string to the buffer without a length prefix.
     */
    public writeSizedString = (value: string) => {
        const slice: Uint8Array = new TextEncoder().encode(value)

        this.write(slice)
    }

    /**
     * Reads a u8 from the buffer.
     */
    public readUint8 = (): number => {
        this.start_reading()

        const buf = new Uint8Array(1)
        buf.set(this.inner.subarray(this.pos, this.pos + 1))
        this.pos += 1

        return buf[0]
    }

    /**
     * Reads a u16 from the buffer.
     */
    public readUint16 = (): number => {
        this.start_reading()

        const buf = new Uint8Array(2)
        buf.set(this.inner.subarray(this.pos, this.pos + 2))
        this.pos += 2

        return (buf[0] << 8) | buf[1]
    }

    /**
     * Reads a u32 from the buffer.
     */
    public readUint32 = (): number => {
        this.start_reading()

        const buf = new Uint8Array(4)
        buf.set(this.inner.subarray(this.pos, this.pos + 4))
        this.pos += 4

        return ((buf[0] << 24) | (buf[1] << 16) | (buf[2] << 8) | buf[3]) >>> 0
    }

    /**
     * Reads an i8 from the buffer.
     */
    public readInt8 = (): number => {
        this.start_reading()

        const buf = new Uint8Array(1)
        buf.set(this.inner.subarray(this.pos, this.pos + 1))
        this.pos += 1

        const value = buf[0]

        return buf[0] & 0x80 ? -value : value
    }

    /**
     * Reads an i16 from the buffer.
     */
    public readInt16 = (): number => {
        this.start_reading()

        const buf = new Uint8Array(2)
        buf.set(this.inner.subarray(this.pos, this.pos + 2))
        this.pos += 2

        const value = (buf[0] << 8) | buf[1]

        return buf[0] & 0x80 ? -value : value
    }

    /**
     * Reads an i32 from the buffer.
     */
    public readInt32 = (): number => {
        this.start_reading()

        const buf = new Uint8Array(4)
        buf.set(this.inner.subarray(this.pos, this.pos + 4))
        this.pos += 4

        const value =
            ((buf[0] << 24) | (buf[1] << 16) | (buf[2] << 8) | buf[3]) >>> 0

        return buf[0] & 0x80 ? -value : value
    }

    /**
     * Reads a bool from the buffer.
     */
    public readBool = (): boolean => {
        this.start_reading()

        const buf = new Uint8Array(1)
        buf.set(this.inner.subarray(this.pos, this.pos + 1))
        this.pos += 1

        return buf[0] === 1
    }

    /**
     *
     */
    public readPackedBools = (): Array<boolean> => {
        return []
    }

    /**
     * Reads a variable length string from the buffer.
     */
    public readString = (): string => {
        this.start_reading()

        const len = this.readUint16()
        const slice = this.inner.subarray(this.pos, this.pos + len)
        const str = new TextDecoder("utf-8").decode(slice)
        this.pos += len

        return str
    }

    /**
     * Reads a sized string from the buffer. You must pass the length of the string in bytes.
     */
    public readSizedString = (size: number): string => {
        this.start_reading()

        const slice = this.inner.subarray(this.pos, this.pos + size)
        const str = new TextDecoder("utf-8").decode(slice)
        this.pos += size

        return str
    }

    /**
     * Reads a sized string from the buffer. This will read from the buffers current position until
     * the end of the buffer, so this function should not be used unless you know that the string is
     * the last value in the buffer. This removes the overhead of a length prefix; it is recommended
     * to plan your packets out such that they end with a sized string where possible.
     */
    public readStringRemaining = (): string => {
        this.start_reading()

        const slice = this.inner.subarray(this.pos, this.inner.length)
        const str = new TextDecoder("utf-8").decode(slice)
        this.pos = this.inner.length

        return str
    }

    public serialize = (obj: object) => {}

    public serializeNumber = (number: number) => {}

    public serializeString = (string: string) => {}

    public serializeBoolean = (bool: boolean) => {}
}

if (import.meta.vitest) {
    const { it, expect } = import.meta.vitest

    it("test write u8", () => {
        const buf = new Bufferfish()
        buf.writeUint8(0)
        buf.writeUint8(255)

        expect(buf.view()).toEqual(new Uint8Array([0, 255]))
    })

    it("test write u16", () => {
        const buf = new Bufferfish()
        buf.writeUint16(0)
        buf.writeUint16(12345)
        buf.writeUint16(65535)

        expect(buf.view()).toEqual(new Uint8Array([0, 0, 48, 57, 255, 255]))
    })

    it("test write u32", () => {
        const buf = new Bufferfish()
        buf.writeUint32(0)
        buf.writeUint32(1234567890)
        buf.writeUint32(4294967295)

        expect(buf.view()).toEqual(
            new Uint8Array([0, 0, 0, 0, 73, 150, 2, 210, 255, 255, 255, 255])
        )
    })

    it("test read u8", () => {
        const buf = new Bufferfish()
        buf.writeUint8(0)
        buf.writeUint8(255)

        expect(buf.readUint8()).toEqual(0)
        expect(buf.readUint8()).toEqual(255)
    })

    it("test read u16", () => {
        const buf = new Bufferfish()
        buf.writeUint16(0)
        buf.writeUint16(12345)
        buf.writeUint16(65535)

        expect(buf.readUint16()).toEqual(0)
        expect(buf.readUint16()).toEqual(12345)
        expect(buf.readUint16()).toEqual(65535)
    })

    it("test read u32", () => {
        const buf = new Bufferfish()
        buf.writeUint32(0)
        buf.writeUint32(1234567890)
        buf.writeUint32(4294967295)

        expect(buf.readUint32()).toEqual(0)
        expect(buf.readUint32()).toEqual(1234567890)
        expect(buf.readUint32()).toEqual(4294967295)
    })

    it("test write i8", () => {
        const buf = new Bufferfish()
        buf.writeInt8(0)
        buf.writeInt8(127)
        buf.writeInt8(-128)

        expect(buf.view()).toEqual(new Uint8Array([0, 127, 128]))
    })

    it("test write i16", () => {
        const buf = new Bufferfish()
        buf.writeInt16(0)
        buf.writeInt16(12345)
        buf.writeInt16(32767)
        buf.writeInt16(-32768)

        expect(buf.view()).toEqual(
            new Uint8Array([0, 0, 48, 57, 127, 255, 128, 0])
        )
    })

    it("test write i32", () => {
        const buf = new Bufferfish()
        buf.writeInt32(0)
        buf.writeInt32(1234567890)
        buf.writeInt32(2147483647)
        buf.writeInt32(-2147483648)

        expect(buf.view()).toEqual(
            new Uint8Array([
                0, 0, 0, 0, 73, 150, 2, 210, 127, 255, 255, 255, 128, 0, 0, 0,
            ])
        )
    })

    it("test read i8", () => {
        const buf = new Bufferfish()
        buf.writeInt8(0)
        buf.writeInt8(127)
        buf.writeInt8(-128)

        expect(buf.readInt8()).toEqual(0)
        expect(buf.readInt8()).toEqual(127)
        expect(buf.readInt8()).toEqual(-128)
    })

    it("test read i16", () => {
        const buf = new Bufferfish()
        buf.writeInt16(0)
        buf.writeInt16(12345)
        buf.writeInt16(32767)
        buf.writeInt16(-32768)

        expect(buf.readInt16()).toEqual(0)
        expect(buf.readInt16()).toEqual(12345)
        expect(buf.readInt16()).toEqual(32767)
        expect(buf.readInt16()).toEqual(-32768)
    })

    it("test read i32", () => {
        const buf = new Bufferfish()
        buf.writeInt32(0)
        buf.writeInt32(1234567890)
        buf.writeInt32(2147483647)
        buf.writeInt32(-2147483648)

        expect(buf.readInt32()).toEqual(0)
        expect(buf.readInt32()).toEqual(1234567890)
        expect(buf.readInt32()).toEqual(2147483647)
        expect(buf.readInt32()).toEqual(-2147483648)
    })

    it("test read reset", () => {
        const buf = new Bufferfish()
        buf.writeUint8(0)
        buf.readUint8()
        buf.writeUint8(255)

        expect(buf.readUint8()).toEqual(0)
    })

    it("test bufferfish overflow", () => {
        const buf = new Bufferfish()

        expect(() => {
            for (let i = 0; i < 1025; i++) {
                buf.writeUint8(0)
            }
        }).toThrowError("Bufferfish is full")
    })

    it("test write string", () => {
        const buf = new Bufferfish()
        buf.writeString("Bufferfish")

        expect(buf.view()).toEqual(
            new Uint8Array([
                0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104,
            ])
        )
    })

    it("test write string big chars", () => {
        const buf = new Bufferfish()
        buf.writeString("안녕하세요")

        expect(buf.view()).toEqual(
            new Uint8Array([
                0, 15, 236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132,
                184, 236, 154, 148,
            ])
        )
    })

    it("test write multiple strings", () => {
        const buf = new Bufferfish()
        buf.writeString("Bufferfish")
        buf.writeString("안녕하세요")

        expect(buf.view()).toEqual(
            new Uint8Array([
                0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 0, 15,
                236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236,
                154, 148,
            ])
        )
    })

    it("test write fixed string", () => {
        const buf = new Bufferfish()
        buf.writeSizedString("Bufferfish")

        expect(buf.view()).toEqual(
            new Uint8Array([66, 117, 102, 102, 101, 114, 102, 105, 115, 104])
        )
    })

    it("test read string", () => {
        const buf = new Bufferfish()
        buf.writeString("Bufferfish")

        expect(buf.readString()).toEqual("Bufferfish")
    })

    it("test read sized string", () => {
        const buf = new Bufferfish()
        buf.writeSizedString("Bufferfish")

        expect(buf.readSizedString(10)).toEqual("Bufferfish")
    })

    it("test write bool", () => {
        const buf = new Bufferfish()
        buf.writeBool(true)
        buf.writeBool(false)

        expect(buf.view()).toEqual(new Uint8Array([1, 0]))
    })

    it("test write packed bools", () => {
        const buf = new Bufferfish()
        buf.writePackedBools([true, false, true, true])
        buf.writePackedBools([false, false, true, false])

        expect(buf.view()).toEqual(new Uint8Array([11, 2]))
    })

    it("test read bool", () => {
        const buf = new Bufferfish()
        buf.writeBool(true)
        buf.writeBool(false)

        expect(buf.readBool()).toEqual(true)
        expect(buf.readBool()).toEqual(false)
    })

    it("test read packed bools")
}
