export declare class Bufferfish {
    private inner;
    private pos;
    private reading;
    private capacity;
    constructor(buf?: ArrayBuffer);
    /**
     * Writes a byte array to the internal buffer. Returns the numbers of bytes
     * written to the buffer.
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
     */
    setMaxCapacity(capacity: number): void;
    /**
     * Returns the next byte in the buffer without advancing the cursor.
     * Returns undefined if the cursor is at the end of the buffer.
     */
    peek: () => number | undefined;
    /**
     * Returns the next n bytes in the buffer without advancing the cursor.
     * Returns undefined if the cursor is at the end of the buffer.
     */
    peekN: (n: number) => Uint8Array | undefined;
    /**
     * Writes a single u8 to the buffer as one byte.
     */
    writeUint8: (value: number) => void;
    /**
     * Writes a u16 to the buffer as two bytes.
     */
    writeUint16: (value: number) => void;
    /**
     * Writes a u32 to the buffer as four bytes.
     */
    writeUint32: (value: number) => void;
    /**
     * Writes an i8 to the buffer as one byte.
     */
    writeInt8: (value: number) => void;
    /**
     * Writes an i16 to the buffer as two bytes.
     */
    writeInt16: (value: number) => void;
    /**
     * Writes an i32 to the buffer as four bytes.
     */
    writeInt32: (value: number) => void;
    /**
     * Writes a bool to the buffer as a single byte.
     */
    writeBool: (value: boolean) => void;
    /**
     * Writes a series of bools to the buffer as a single byte. This allows up
     * to 4 bools to be represented as a single byte. The first 4 bits are used
     * as a mask to determine which of the last 4 bits are set.
     */
    writePackedBools: (values: Array<boolean>) => void;
    /**
     * Writes a variable length string to the buffer. It will be prefixed with
     * its length in bytes as a u16 (two bytes).
     */
    writeString: (value: string) => void;
    /**
     * Writes a string to the buffer without a length prefix.
     */
    writeSizedString: (value: string) => void;
    /**
     * Writes an array of raw bytes to the buffer. Useful for serializing ///
       distinct structs into byte arrays and appending them to a buffer later.
     */
    writeRawBytes: (value: Uint8Array) => void;
    /**
     * Reads a u8 from the buffer.
     */
    readUint8: () => number;
    /**
     * Reads a u16 from the buffer.
     */
    readUint16: () => number;
    /**
     * Reads a u32 from the buffer.
     */
    readUint32: () => number;
    /**
     * Reads an i8 from the buffer.
     */
    readInt8: () => number;
    /**
     * Reads an i16 from the buffer.
     */
    readInt16: () => number;
    /**
     * Reads an i32 from the buffer.
     */
    readInt32: () => number;
    /**
     * Reads a bool from the buffer.
     */
    readBool: () => boolean;
    /**
     *
     */
    readPackedBools: () => Array<boolean>;
    /**
     * Reads a variable length string from the buffer.
     */
    readString: () => string;
    /**
     * Reads a sized string from the buffer. You must pass the length of the
     * string in bytes.
     */
    readSizedString: (size: number) => string;
    /**
     * Reads a sized string from the buffer. This will read from the buffers
     * current position until the end of the buffer, so this function should not
     * be used unless you know that the string is the last value in the buffer.
     * This removes the overhead of a length prefix; it is recommended to plan
     * your packets out such that they end with a sized string where possible.
     */
    readStringRemaining: () => string;
}
