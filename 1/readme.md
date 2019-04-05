Challenge 1: Hex to Base64
==========================

In this problem, we are given a string encoded as hex and are asked to re-encode it as base64 instead.

Hex:

```
49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
```

Base64:

```
SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
```

To do this, we need to understand these encodings.

Hex encoding is base 16 (2^4), so it's able to encode 4 bits worth of information in a single character.

| char | binary |
|------|--------|
| 0    | 0000   |
| 1    | 0001   |
| 2    | 0010   |
| 3    | 0011   |
| 4    | 0100   |
| 5    | 0101   |
| 6    | 0110   |
| 7    | 0111   |
| 8    | 1000   |
| 9    | 1001   |
| a    | 1010   |
| b    | 1011   |
| c    | 1100   |
| d    | 1101   |
| e    | 1110   |
| f    | 1111   |

Base64 encoding is, well, base 64 (2^6), so it's similarly able to encode 6 bits worth of information in a single character.

| char | binary |
|------|--------|
| A    | 000000 |
| B    | 000001 |
| C    | 000010 |
| D    | 000011 |
| E    | 000100 |
| F    | 000101 |
| G    | 000110 |
| H    | 000111 |
| I    | 001000 |
| J    | 001001 |
| K    | 001010 |
| L    | 001011 |
| M    | 001100 |
| N    | 001101 |
| O    | 001110 |
| P    | 001111 |
| Q    | 010000 |
| R    | 010001 |
| S    | 010010 |
| T    | 010011 |
| U    | 010100 |
| V    | 010101 |
| W    | 010110 |
| X    | 010111 |
| Y    | 011000 |
| Z    | 011001 |
| a    | 011010 |
| b    | 011011 |
| c    | 011100 |
| d    | 011101 |
| e    | 011110 |
| f    | 011111 |
| g    | 100000 |
| h    | 100001 |
| i    | 100010 |
| j    | 100011 |
| k    | 100100 |
| l    | 100101 |
| m    | 100110 |
| n    | 100111 |
| o    | 101000 |
| p    | 101001 |
| q    | 101010 |
| r    | 101011 |
| s    | 101100 |
| t    | 101101 |
| u    | 101110 |
| v    | 101111 |
| w    | 110000 |
| x    | 110001 |
| y    | 110010 |
| z    | 110011 |
| 0    | 110100 |
| 1    | 110101 |
| 2    | 110110 |
| 3    | 110111 |
| 4    | 111000 |
| 5    | 111001 |
| 6    | 111010 |
| 7    | 111011 |
| 8    | 111100 |
| 9    | 111101 |
| +    | 111110 |
| /    | 111111 |

Each of these strings is encoding the same number of bits, but Base64 is able to encode the same information in 4/6 the number of output bytes.

The hex string is 96 bytes, so it represents a total of 384 bits. Our strategy should be to read in the string, parse out the bits, then re-encode those 384 bits into 64 bytes of Base64.

Here's a little two-liner in Ruby which does this process as described: It splits the hex input, evaluates each char as a 4-bit string, combines them all, splits them into 6-bit strings, then maps each to it's base64 character.

```ruby
map = %w(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z a b c d e f g h i j k l m n o p q r z t u v w x y z 0 1 2 3 4 5 6 7 8 9 + /)
input.split('').map { |i| (eval "0x#{i}").to_s(2).rjust(4, '0') }.join.scan(/.{6}/).map { |i| map[i.to_i(2)] }.join
```

This can be done more precisely in a programming language like rust. In fact, this solution relies on encoding strings to and from binary, but this challenge comes with a warning:

> Cryptopals Rule
>
> Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.

Check out `solution.rs` for a proper solution using Rust. This solution uses a 4-byte integer as a buffer to pull bytes out of the hex input, then convert them to base64 output.
