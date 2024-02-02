## Crypto-min
An encryption/decryption program with an algorithm I came up on my own.
It encrypts files based on successive rotations of their bytes.

## The algorithm
Here is a verbal description of the algorithm.

1. The user gives an input and a password.
2. The characters of the password are treated as u8, and successively
        XORed or XNORed, depending on whether their are an odd or an even number of characters.
        Let T the result of this operation.
        T is a byte treated as a three-digit integer between 000-255.
        The first digit defines whether the first rotation is to the left (0 or 1), or to the right (2).
        The second digit defines the number of successive rotations to the right. The third, the number
        of rotations to the left.
3. The bytes of the input are rotated to the right or the left, for as many bits as the "ones" of the password bytes XOR T.
        i.e. We XOR the first char of the password with T, and use the resulting number of 1s to rotate the first char of the input.
             We XOR the second char of the password with T, and use the resulting number of 1s to rotate the second char of the input.
             e.t.c

There is a TODO of optimizing it by working with four bytes at once instead of one.



