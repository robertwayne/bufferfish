const OVERFLOW_ERR = "attempted to read past the end of the Bufferfish"

/**
 * A wrapper around Uint8Array that provides a simple API for reading and
 * writing binary data. This is meant to be used with its companion library in
 * Rust to provide consistent encoding and decoding interop.
 */
export class Bufferfish {
    private inner: Uint8Array
    private position: number
    private reading: boolean
    private capacity: number

    private textDecoder: TextDecoder | undefined
    private textEncoder: TextEncoder | undefined

    constructor(bf: ArrayBufferLike = new ArrayBuffer(0)) {
        this.inner = new Uint8Array(bf)
        this.position = 0
        this.reading = false
        this.capacity = 1024

        this.textDecoder = undefined
        this.textEncoder = undefined
    }

    /**
     * Writes a byte array to the internal buffer. Returns the numbers of bytes
     * written to the buffer or an error if the capacity is exceeded.
     *
     * This should only be called by the library.
     */
    private write(bf: Uint8Array): number | Error {
        if (
            this.capacity > 0 &&
            (bf.length > this.capacity ||
                this.inner.length + bf.length > this.capacity)
        ) {
            return new Error(
                `Bufferfish capacity exceeded (${this.capacity} bytes)`,
            )
        }

        this.reading = false

        const tmp = new Uint8Array(this.inner.length + bf.length)
        tmp.set(this.inner, 0)
        tmp.set(bf, this.inner.length)
        this.inner = tmp

        const bytesWritten = bf.length
        this.position += bytesWritten

        return bytesWritten
    }

    /**
     * Returns a view into the inner Uint8Array.
     */
    public view = (): Uint8Array => {
        return this.inner.subarray()
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

        this.position = 0
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
     * Returns the next byte in the buffer without advancing the cursor.
     *
     * Throws if the cursor is at the end of the buffer.
     */
    public peek = (): number | Error => {
        this.startReading()

        const value = this.inner.slice(this.position, this.position + 1)[0]

        if (this.position >= this.inner.length || value === undefined) {
            return new Error(
                `peek of 1 byte exceeds the max capacity of ${this.capacity} bytes on this Bufferfish`,
            )
        }

        return value
    }

    /**
     * Returns the next n bytes in the buffer without advancing the cursor.
     * Returns undefined if the cursor is at the end of the buffer.
     */
    public peekN = (n: number): Uint8Array | Error => {
        this.startReading()

        const value = this.inner.slice(this.position, this.position + n)

        if (this.position + n > this.inner.length) {
            return new Error(
                `peek of ${n} bytes exceeds the max capacity of ${this.capacity} bytes on this Bufferfish`,
            )
        }

        return value
    }

    /**
     * Appends another `Bufferfish`, `Uint8Array`, `ArrayBuffer`, or `Array<number>` to the buffer. This modifies the `Bufferfish` in-place.
     */
    public push = (
        arr: Bufferfish | Uint8Array | ArrayBuffer | Array<number>,
    ): void | Error => {
        if (arr instanceof Bufferfish) {
            const err = this.write(arr.view())
            if (err instanceof Error) {
                return err
            }
        } else if (arr instanceof Uint8Array) {
            const err = this.write(arr)
            if (err instanceof Error) {
                return err
            }
        } else if (arr instanceof ArrayBuffer) {
            const err = this.write(new Uint8Array(arr))
            if (err instanceof Error) {
                return err
            }
        } else if (arr instanceof Array) {
            const err = this.write(new Uint8Array(arr))
            if (err instanceof Error) {
                return err
            }
        } else {
            return new Error("invalid type")
        }
    }

    /**
     * Writes a single u8 to the buffer as one byte.
     *
     * Returns an error if the value is out of range (0-255).
     */
    public writeUint8 = (value: number): void | Error => {
        if (value > 255 || value < 0) {
            return new Error(`value ${value} must be between 0 and 255`)
        }

        const slice: Uint8Array = new Uint8Array(1)
        const view = new DataView(slice.buffer)
        view.setUint8(0, value)

        const err = this.write(slice)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes a u16 to the buffer as two bytes.
     *
     * Returns an error if the value is out of range (0-65535).
     */
    public writeUint16 = (value: number): void | Error => {
        if (value > 65535 || value < 0) {
            return new Error(`value ${value} must be between 0 and 65535`)
        }

        const slice: Uint8Array = new Uint8Array(2)
        const view = new DataView(slice.buffer)
        view.setUint16(0, value)

        const err = this.write(slice)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes a u32 to the buffer as four bytes.
     *
     * Returns an error if the value is out of range (0-4294967295).
     */
    public writeUint32 = (value: number): void | Error => {
        if (value > 4294967295 || value < 0) {
            return new Error(`value ${value} must be between 0 and 4294967295`)
        }

        const slice: Uint8Array = new Uint8Array(4)
        const view = new DataView(slice.buffer)
        view.setUint32(0, value)

        const err = this.write(slice)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes an i8 to the buffer as one byte.
     *
     * Returns an error if the value is out of range (-128-127).
     */
    public writeInt8 = (value: number): void | Error => {
        if (value > 127 || value < -128) {
            return new Error(`value ${value} must be between -128 and 127`)
        }

        const slice: Uint8Array = new Uint8Array(1)
        const view = new DataView(slice.buffer)
        view.setInt8(0, value)

        const err = this.write(slice)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes an i16 to the buffer as two bytes.
     *
     * Returns an error if the value is out of range (-32768-32767).
     */
    public writeInt16 = (value: number): void | Error => {
        if (value > 32767 || value < -32768) {
            return new Error(`value ${value} must be between -32768 and 32767`)
        }

        const slice: Uint8Array = new Uint8Array(2)
        const view = new DataView(slice.buffer)
        view.setInt16(0, value)

        const err = this.write(slice)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes an i32 to the buffer as four bytes.
     *
     * Returns an error if the value is out of range (-2147483648-2147483647).
     */
    public writeInt32 = (value: number): void | Error => {
        if (value > 2147483647 || value < -2147483648) {
            return new Error(
                `value ${value} must be between -2147483648 and 2147483647`,
            )
        }

        const slice: Uint8Array = new Uint8Array(4)
        const view = new DataView(slice.buffer)
        view.setInt32(0, value)

        const err = this.write(slice)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes a boolean to the buffer as one byte.
     */
    public writeBool = (value: boolean): void | Error => {
        const err = this.writeUint8(value ? 1 : 0)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes a packed array of booleans to the buffer as a single byte.
     * Can pack up to 8 booleans into a single byte.
     */
    public writePackedBools = (values: Array<boolean>): void | Error => {
        if (values.length > 8) {
            return new Error("cannot pack more than 8 bools into a single byte")
        }

        let packedValue = 0x00
        for (const value of values) {
            packedValue <<= 1
            if (value) {
                packedValue |= 1
            }
        }

        packedValue <<= 8 - values.length // Shift remaining bits to the left to fill the byte

        const err = this.writeUint8(packedValue)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Writes a unicode string literal to the buffer. It will be prefixed with
     * its length in bytes as a u16 (two bytes), and each character will be 1 to
     * 4-bytes, whichever is the smallest it can fit into.
     */
    public writeString = (value: string): void | Error => {
        if (!this.textEncoder) this.textEncoder = new TextEncoder()
        const slice: Uint8Array = this.textEncoder.encode(value)

        const err = this.writeUint16(slice.length)
        if (err instanceof Error) {
            return err
        }

        const err2 = this.write(slice)
        if (err2 instanceof Error) {
            return err2
        }
    }

    /**
     * Writes an array of raw bytes to the buffer. Useful for encoding
     * distinct structs into byte arrays and appending them to a buffer later.
     */
    public writeRawBytes = (value: Uint8Array): void | Error => {
        const err = this.write(value)
        if (err instanceof Error) {
            return err
        }
    }

    /**
     * Attempts to read a u8 from the buffer.
     */
    public readUint8(): number | Error {
        this.startReading()

        if (this.position + 1 > this.inner.length) {
            return new Error(OVERFLOW_ERR)
        }

        const value = new DataView(
            this.inner.buffer,
            this.position,
            1,
        ).getUint8(0)
        this.position += 1

        return value
    }

    /**
     * Attempts to read a u16 from the buffer.
     */
    public readUint16(): number | Error {
        this.startReading()

        if (this.position + 2 > this.inner.length) {
            return new Error(OVERFLOW_ERR)
        }

        const value = new DataView(
            this.inner.buffer,
            this.position,
            2,
        ).getUint16(0)
        this.position += 2

        return value
    }

    /**
     * Attempts to read a u32 from the buffer.
     */
    public readUint32(): number | Error {
        this.startReading()

        if (this.position + 4 > this.inner.length) {
            return new Error(OVERFLOW_ERR)
        }

        const value = new DataView(
            this.inner.buffer,
            this.position,
            4,
        ).getUint32(0)
        this.position += 4

        return value
    }

    /**
     * Attempts to read an i8 from the buffer.
     */
    public readInt8(): number | Error {
        this.startReading()

        if (this.position + 1 > this.inner.length) {
            return new Error(OVERFLOW_ERR)
        }

        const value = new DataView(this.inner.buffer, this.position, 1).getInt8(
            0,
        )
        this.position += 1

        return value
    }

    /**
     * Attempts to read an i16 from the buffer.
     */
    public readInt16(): number | Error {
        this.startReading()

        if (this.position + 2 > this.inner.length) {
            return new Error(OVERFLOW_ERR)
        }

        const value = new DataView(
            this.inner.buffer,
            this.position,
            2,
        ).getInt16(0)
        this.position += 2

        return value
    }

    /**
     * Attempts to read an i32 from the buffer.
     */
    public readInt32(): number | Error {
        this.startReading()

        if (this.position + 4 > this.inner.length) {
            return new Error(OVERFLOW_ERR)
        }

        const value = new DataView(
            this.inner.buffer,
            this.position,
            4,
        ).getInt32(0)
        this.position += 4

        return value
    }

    /**
     * Attempts to read a bool from the buffer.
     */
    public readBool(): boolean | Error {
        const valueOrError = this.readUint8()
        if (valueOrError instanceof Error) {
            return valueOrError
        }

        return valueOrError === 1
    }

    /**
     * Attempts to read a packed array of booleans from the buffer.
     * By default, it will read 8 booleans from a single byte. If you
     * packed less than 8, the count parameter can be used to specify
     * how many booleans to read.
     */
    public readPackedBools(count: number = 8): Array<boolean> | Error {
        if (count > 8) {
            return new Error("cannot read more than 8 bools from a single byte")
        }

        const packedValueOrError = this.readUint8()
        if (packedValueOrError instanceof Error) {
            return packedValueOrError
        }

        const packedValue = packedValueOrError as number
        const bools: Array<boolean> = []
        for (let i = 0; i < count; i++) {
            bools.push((packedValue & (1 << (7 - i))) !== 0)
        }

        return bools
    }

    /**
     * Attempts to read a variable length string from the buffer.
     */
    public readString(): string | Error {
        const lengthOrError = this.readUint16()
        if (lengthOrError instanceof Error) {
            return lengthOrError
        }

        const length = lengthOrError as number
        if (this.position + length > this.inner.length) {
            return new Error(OVERFLOW_ERR)
        }

        if (!this.textDecoder) this.textDecoder = new TextDecoder("utf-8")

        const value = this.textDecoder.decode(
            this.inner.subarray(this.position, this.position + length),
        )
        this.position += length

        return value
    }

    /**
     * Attempts to read a variable-length array of elements from the buffer.
     */
    public readArray<T>(readFn: () => T): Array<T> | Error {
        const lengthOrError = this.readUint16()
        if (lengthOrError instanceof Error) {
            return lengthOrError
        }

        const length = lengthOrError as number
        const values: Array<T> = []
        for (let i = 0; i < length; i++) {
            const valueOrError = readFn()
            if (valueOrError instanceof Error) {
                return valueOrError
            }

            values.push(valueOrError as T)
        }

        return values
    }
}
