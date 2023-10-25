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
     * Writes a byte array to the internal buffer. Returns the numbers of bytes
     * written to the buffer.
     *
     * This should only be called by the library.
     */
    private write(buf: Uint8Array): number {
        if (
            this.capacity > 0 &&
            (buf.length > this.capacity ||
                this.inner.length + buf.length > this.capacity)
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
    private startReading(): void {
        if (this.reading) {
            return
        }

        this.pos = 0
        this.reading = true
    }

    /**
     * Sets the max capacity (in bytes) for the internal buffer.
     * A value of 0 will allow the buffer to grow indefinitely.
     */
    public setMaxCapacity(capacity: number): void {
        this.capacity = capacity
    }

    /**
     * Returns the next byte in the buffer without advancing the cursor. Returns
     * undefined if the cursor is at the end of the buffer.
     */
    public peek = (): number | null => {
        this.startReading()

        if (this.pos >= this.inner.length) {
            return null
        }

        return this.inner.slice(this.pos, this.pos + 1)[0] ?? null
    }

    /**
     * Returns the next n bytes in the buffer without advancing the cursor.
     * Returns undefined if the cursor is at the end of the buffer.
     */
    public peekN = (n: number): Uint8Array | null => {
        this.startReading()

        if (this.pos + n > this.inner.length) {
            return null
        }

        return this.inner.slice(this.pos, this.pos + n)
    }

    /**
     * Appends another Bufferfish, Uint8Array, ArrayBuffer, or Array<number> to
     * the buffer. This modifies the Bufferfish in-place.
     */
    public push = (
        arr: Bufferfish | Uint8Array | ArrayBuffer | Array<number>,
    ): void => {
        if (arr instanceof Bufferfish) {
            this.write(arr.view())
        } else if (arr instanceof Uint8Array) {
            this.write(arr)
        } else if (arr instanceof ArrayBuffer) {
            this.write(new Uint8Array(arr))
        } else if (arr instanceof Array) {
            this.write(new Uint8Array(arr))
        } else {
            throw new Error("Invalid type")
        }
    }

    /**
     * Writes a single u8 to the buffer as one byte.
     */
    public writeUint8 = (value: number): void => {
        if (value > 255 || value < 0) {
            throw new Error("Value is out of range")
        }

        const slice: Uint8Array = new Uint8Array(1)
        const view = new DataView(slice.buffer)
        view.setUint8(0, value)

        this.write(slice)
    }

    /**
     * Writes a u16 to the buffer as two bytes.
     */
    public writeUint16 = (value: number): void => {
        if (value > 65535 || value < 0) {
            throw new Error("Value is out of range")
        }

        const slice: Uint8Array = new Uint8Array(2)
        const view = new DataView(slice.buffer)
        view.setUint16(0, value)

        this.write(slice)
    }

    /**
     * Writes a u32 to the buffer as four bytes.
     */
    public writeUint32 = (value: number): void => {
        if (value > 4294967295 || value < 0) {
            throw new Error("Value is out of range")
        }

        const slice: Uint8Array = new Uint8Array(4)
        const view = new DataView(slice.buffer)
        view.setUint32(0, value)

        this.write(slice)
    }

    /**
     * Writes an i8 to the buffer as one byte.
     */
    public writeInt8 = (value: number): void => {
        if (value > 127 || value < -128) {
            throw new Error("Value is out of range")
        }

        const slice: Uint8Array = new Uint8Array(1)
        const view = new DataView(slice.buffer)
        view.setInt8(0, value)

        this.write(slice)
    }

    /**
     * Writes an i16 to the buffer as two bytes.
     */
    public writeInt16 = (value: number): void => {
        if (value > 32767 || value < -32768) {
            throw new Error("Value is out of range")
        }

        const slice: Uint8Array = new Uint8Array(2)
        const view = new DataView(slice.buffer)
        view.setInt16(0, value)

        this.write(slice)
    }

    /**
     * Writes an i32 to the buffer as four bytes.
     */
    public writeInt32 = (value: number): void => {
        if (value > 2147483647 || value < -2147483648) {
            throw new Error("Value is out of range")
        }

        const slice: Uint8Array = new Uint8Array(4)
        const view = new DataView(slice.buffer)
        view.setInt32(0, value)

        this.write(slice)
    }

    /**
     * Writes a bool to the buffer as a single byte.
     */
    public writeBool = (value: boolean): void => {
        this.writeUint8(value ? 1 : 0)
    }

    /**
     * Writes a series of bools to the buffer as a single byte. This allows up
     * to 4 bools to be represented as a single byte. The first 4 bits are used
     * as a mask to determine which of the last 4 bits are set.
     */
    public writePackedBools = (values: Array<boolean>): void => {
        if (values.length > 4) {
            throw new Error(
                "Each packed bool can only represent 4 or fewer values",
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
     * Writes a unicode string literal to the buffer. It will be prefixed with
     * its length in bytes as a u16 (two bytes), and each character will be 1 to
     * 4-bytes, whichever is the smallest it can fit into.
     */
    public writeString = (value: string): void => {
        const slice: Uint8Array = new TextEncoder().encode(value)

        this.writeUint16(slice.length)
        this.write(slice)
    }

    /**
     * Writes a unicode string literal to the buffer without a length prefix.
     * Each character will be 1 to 4-bytes, whichever is the smallest it can fit
     * into.
     */
    public writeSizedString = (value: string): void => {
        const slice: Uint8Array = new TextEncoder().encode(value)

        this.write(slice)
    }

    /**
     * Writes an array of raw bytes to the buffer. Useful for serializing ///
       distinct structs into byte arrays and appending them to a buffer later.
     */
    public writeRawBytes = (value: Uint8Array): void => {
        this.write(value)
    }

    /**
     * Reads a u8 from the buffer.
     */
    public readUint8 = (): number => {
        this.startReading()

        const buf = new Uint8Array(1)
        buf.set(this.inner.subarray(this.pos, this.pos + 1))
        this.pos += 1

        return new DataView(buf.buffer).getUint8(0)
    }

    /**
     * Reads a u16 from the buffer.
     */
    public readUint16 = (): number => {
        this.startReading()

        const buf = new Uint8Array(2)
        buf.set(this.inner.subarray(this.pos, this.pos + 2))
        this.pos += 2

        return new DataView(buf.buffer).getUint16(0)
    }

    /**
     * Reads a u32 from the buffer.
     */
    public readUint32 = (): number => {
        this.startReading()

        const buf = new Uint8Array(4)
        buf.set(this.inner.subarray(this.pos, this.pos + 4))
        this.pos += 4

        return new DataView(buf.buffer).getUint32(0)
    }

    /**
     * Reads an i8 from the buffer.
     */
    public readInt8 = (): number => {
        this.startReading()

        const buf = new Uint8Array(1)
        buf.set(this.inner.subarray(this.pos, this.pos + 1))
        this.pos += 1

        return new DataView(buf.buffer).getInt8(0)
    }

    /**
     * Reads an i16 from the buffer.
     */
    public readInt16 = (): number => {
        this.startReading()

        const buf = new Uint8Array(2)
        buf.set(this.inner.subarray(this.pos, this.pos + 2))
        this.pos += 2

        return new DataView(buf.buffer).getInt16(0)
    }

    /**
     * Reads an i32 from the buffer.
     */
    public readInt32 = (): number => {
        this.startReading()

        const buf = new Uint8Array(4)
        buf.set(this.inner.subarray(this.pos, this.pos + 4))
        this.pos += 4

        return new DataView(buf.buffer).getInt32(0)
    }

    /**
     * Reads a bool from the buffer.
     */
    public readBool = (): boolean => {
        this.startReading()

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
        this.startReading()

        const len = this.readUint16()
        const slice = this.inner.subarray(this.pos, this.pos + len)
        const str = new TextDecoder("utf-8").decode(slice)
        this.pos += len

        return str
    }

    /**
     * Reads a sized string from the buffer. You must pass the length of the
     * string in bytes.
     */
    public readSizedString = (size: number): string => {
        this.startReading()

        const slice = this.inner.subarray(this.pos, this.pos + size)
        const str = new TextDecoder("utf-8").decode(slice)
        this.pos += size

        return str
    }

    /**
     * Reads a sized string from the buffer. This will read from the buffers
     * current position until the end of the buffer, so this function should not
     * be used unless you know that the string is the last value in the buffer.
     * This removes the overhead of a length prefix; it is recommended to plan
     * your packets out such that they end with a sized string where possible.
     */
    public readStringRemaining = (): string => {
        this.startReading()

        const slice = this.inner.subarray(this.pos, this.inner.length)
        const str = new TextDecoder("utf-8").decode(slice)
        this.pos = this.inner.length

        return str
    }

    // public serialize = (obj: object) => {}

    // public serializeNumber = (number: number) => {}

    // public serializeString = (string: string) => {}

    // public serializeBoolean = (bool: boolean) => {}
}
