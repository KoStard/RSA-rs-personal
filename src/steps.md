Encryption Key - (e, n) - public
Decryption Key - (d, n)

Encryption:
- represent message as blocks, convert them to numbers between 0 and n-1 (just representation of message)
- (M ** e) % n = C

Decryption:
- (C ** d) % n = M


Calculation:

p, q - big primes

n = p * q
d -> some large random integer relatively prime to (p-1) * (q-1)
e -> ?