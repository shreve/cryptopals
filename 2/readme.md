Challenge 2: Fixed XOR
======================

In this challenge, we're asked to xor two input strings and output the result in hex.

```
1c0111001f010100061a024b53535009181c

686974207468652062756c6c277320657965

746865206b696420646f6e277420706c6179
```

This solution will heavily rely on the work of the previous challenge. Instead of re-encoding the result as base64, we'll xor and re-output as hex.
