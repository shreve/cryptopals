Challenge 6: Break reapeating-key XOR
=====================================

In this challenge we're given a text file which has been encrypted
with a repeating-key xor, then base64 encoded. We already know how to
read bytes from base64 from previous challenges, and in the setup for
this challenge, we're given an outline for breaking the repeating-key
xor.

The main component of the attack is a Hamming distance algorithm,
which counts the number of differing bits between two inputs. We can
estimate the length of the key by comparing various lengths of blocks
of the ciphertext and selecting the lengths that produce the lowest
distances.
