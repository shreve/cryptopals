Challenge 5: Implement repeating-key XOR
========================================

In this challenge we're given a key and asked to encrypt the message
by repeating the key to expand it to the length of the message and
performing an xor. This can be done in a streaming approach rather
than building a full length key and performing one xor.

Using key ICE, you can turn this plaintext into this ciphertext.

```
Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal
```

```
0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
```

After going through the previous 4 challenges, this one should feel
pretty straightforward. We're just reading in bytes and performing
lots of xors.
