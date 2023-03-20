# Specification

_This is a work-in-progress._

## Draft Version 2023.03.15

### Types

| Type | ID | Description |
| --- | --- | --- |
| `int` | 0 | Variable-length 32-bit integer |
| `str` | 1 | Variable-length string |
| `bool` | 3 | Boolean |
| `float` | 4 | Floating-point number |
| `array` | 5 | Array of like-values of any non-array type |

## Semantics

`int` values are represented as up to 4-bytes (32-bit integer). If the value is less than or equal to 65535, it will be represented as a 2-byte 16-bit integer. If the value is less than or equal to 255, it will be represented as a 1-byte 8-bit integer.

If a number is negative... ???

`str` values are represented as a list of characters of up to 4-bytes long, representing a valid UTF-8 character.

`bool` values are represented as a single byte, with `0` representing `false` and `1` representing `true`.

`float` values are represented as a 4-byte floating-point number.

`array` values are represented as a list of values. The first byte represents the type. The next two bytes represent the length of the array as a u16. The rest of the bytes represent the values in the array.

## Layout

### Header

| ID | Fields | [Type] | [Offset] |
| --- | --- | --- | --- |
| u16 | u8 | [u8] | [u8] |

`ID` is two bytes representing the ID of the message.

`Fields` is one byte representing the number of fields.

`Type` is one byte representing the [type](#types) of the field.

`Offset` is one byte representing __the length of the field relative to the end of the last field__.

Some examples:

