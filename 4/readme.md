Challenge 4: Detect single-character XOR
========================================

This challenge is a slight step above the previous challenge, but
really forces good habits if you want a solution that performs well.

Instead of searching for the key of one string, we're searching for
the one string in a list (`data.txt`) that actually is encrypted. This
is where the frequency searching comes in really handy. Performing
this task manually would require skimming 15,000 lines, so you can see
how it would get really out of hand for a larger task.

To leverage the previous challenge, just find the best result for each
line, then find the best of the best. Pretty straightforward.

I mention performance because this was the first challenge I noticed
that ran really slow. My first solution was taking about 5 seconds to
find the answer. I ended up rewriting the entire program to rely soely
on `Vec<u8>` instead of ever using strings and chars. It turned out my
scoring solution was really slow. I had input the freqeuncy dict as a
string and was searching that by casting every char to a byte.
Switching the frequency dict to a vector and using `binary_search`
greatly improved performance down to less than 1 second.

I have now learned my lesson about using chars and encoded strings.
