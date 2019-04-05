Challenge 3: Single-byte XOR Cipher
===================================

We are given a single hex-encoded string and are told that it's been
xor'd against a single character. This means xoring against a long
string of the same byte repeated.

```
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

We can find the answer by iterating over every possible key (the byte
values of 0 up to 255), repeating the byte up to the length of the
input, and performing an xor like we saw in the last challenge.

This challenge also states that you can find the secret by visually
scanning the results, but you should instead write some code to find
it for you. It also offers a vague clue about "ETAOIN SHRDLU".

This string is an ordered list of frequently used characters in the
english language. The full list is "etaoin shrdlucmfwypvbgkjqxz", but
you only need up to shrdlu to find the answer here. You can find the
correct secret by scoring each xor result against it's characters'
position in this string.

I believe this challenge was included here because it's the first
challenge that requires you to properly load the hex into byte
values. In the first challenge I used a bit buffer to convert directly
from hex to base64. In the second challenge I was able to get by
treating nibbles as bytes because I was comparing the same number of
hex characters then converting the result back using the same flawed
mechanism.
