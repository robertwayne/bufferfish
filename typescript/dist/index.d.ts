declare module "bufferfish" {
    /**
     * A wrapper around Uint8Array that provides a simple API for reading and
     * writing binary data. This is meant to be used with its companion library in
     * Rust to provide consistent encoding and decoding interop.
     */
    export class Bufferfish {
        private inner;
        private position;
        private reading;
        private capacity;
        private textDecoder;
        private textEncoder;
        constructor(bf?: ArrayBufferLike);
        /**
         * Writes a byte array to the internal buffer. Returns the numbers of bytes
         * written to the buffer or an error if the capacity is exceeded.
         *
         * This should only be called by the library.
         */
        private write;
        /**
         * Returns the (immutable) internal Uint8Array.
         */
        view: () => Uint8Array;
        /**
         * Resets the buffer cursor to the start postion when reading after a write.
         *
         * This should only be called by the library.
         */
        private startReading;
        /**
         * Sets the max capacity (in bytes) for the internal buffer.
         * A value of 0 will allow the buffer to grow indefinitely.
         */
        setMaxCapacity(capacity: number): void;
        /**
         * Returns the next byte in the buffer without advancing the cursor.
         *
         * Throws if the cursor is at the end of the buffer.
         */
        peek: () => number | Error;
        /**
         * Returns the next n bytes in the buffer without advancing the cursor.
         * Returns undefined if the cursor is at the end of the buffer.
         */
        peekN: (n: number) => Uint8Array | Error;
        /**
         * Appends another `Bufferfish`, `Uint8Array`, `ArrayBuffer`, or `Array<number>` to the buffer. This modifies the `Bufferfish` in-place.
         */
        push: (arr: Bufferfish | Uint8Array | ArrayBuffer | Array<number>) => void | Error;
        /**
         * Writes a single u8 to the buffer as one byte.
         *
         * Returns an error if the value is out of range (0-255).
         */
        writeUint8: (value: number) => void | Error;
        /**
         * Writes a u16 to the buffer as two bytes.
         *
         * Returns an error if the value is out of range (0-65535).
         */
        writeUint16: (value: number) => void | Error;
        /**
         * Writes a u32 to the buffer as four bytes.
         *
         * Returns an error if the value is out of range (0-4294967295).
         */
        writeUint32: (value: number) => void | Error;
        /**
         * Writes an i8 to the buffer as one byte.
         *
         * Returns an error if the value is out of range (-128-127).
         */
        writeInt8: (value: number) => void | Error;
        /**
         * Writes an i16 to the buffer as two bytes.
         *
         * Returns an error if the value is out of range (-32768-32767).
         */
        writeInt16: (value: number) => void | Error;
        /**
         * Writes an i32 to the buffer as four bytes.
         *
         * Returns an error if the value is out of range (-2147483648-2147483647).
         */
        writeInt32: (value: number) => void | Error;
        /**
         * Writes a boolean to the buffer as one byte.
         */
        writeBool: (value: boolean) => void | Error;
        /**
         * Writes a packed array of booleans to the buffer as a single byte.
         * Can pack up to 8 booleans into a single byte.
         */
        writePackedBools: (values: Array<boolean>) => void | Error;
        /**
         * Writes a unicode string literal to the buffer. It will be prefixed with
         * its length in bytes as a u16 (two bytes), and each character will be 1 to
         * 4-bytes, whichever is the smallest it can fit into.
         */
        writeString: (value: string) => void | Error;
        /**
         * Writes an array of raw bytes to the buffer. Useful for encoding
         * distinct structs into byte arrays and appending them to a buffer later.
         */
        writeRawBytes: (value: Uint8Array) => void | Error;
        /**
         * Attempts to read a u8 from the buffer.
         */
        readUint8(): number | Error;
        /**
         * Attempts to read a u16 from the buffer.
         */
        readUint16(): number | Error;
        /**
         * Attempts to read a u32 from the buffer.
         */
        readUint32(): number | Error;
        /**
         * Attempts to read an i8 from the buffer.
         */
        readInt8(): number | Error;
        /**
         * Attempts to read an i16 from the buffer.
         */
        readInt16(): number | Error;
        /**
         * Attempts to read an i32 from the buffer.
         */
        readInt32(): number | Error;
        /**
         * Attempts to read a bool from the buffer.
         */
        readBool(): boolean | Error;
        /**
         * Attempts to read a packed array of booleans from the buffer.
         * By default, it will read 8 booleans from a single byte. If you
         * packed less than 8, the count parameter can be used to specify
         * how many booleans to read.
         */
        readPackedBools(count?: number): Array<boolean> | Error;
        /**
         * Attempts to read a variable length string from the buffer.
         */
        readString(): string | Error;
        /**
         * Attempts to read a variable-length array of elements from the buffer.
         */
        readArray<T>(readFn: () => T): Array<T> | Error;
    }
}
