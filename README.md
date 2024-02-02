## Crypto-min
An encryption/decryption program with an algorithm I came up on my own.
It encrypts files based on successive rotations of their bytes.

## The algorithm
The algorithm is designed to hinder frequency analysis and plain text attacks, by rotating bytes 
not only based on passwords, but based on their position in the file as well. 
Here is a verbal description of it:

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

TODO: If one encrypts plain text with this algorithm and the attackers know it, 
it is trivial for them to brute force the first bytes and see if anything meaningful is produced, 
possibly revealing the password if not complex enough. A form of password-derived padding shall be added periodically 
in the textfile (total extra size no more than 5% of the original), including 
the beginning and the end, and encrypted along with the rest of the file to make it harder to attack.


The algorithm has no official name, but if I had to give it one, that would be SYNTOM (from the Greek word 'syntomos' (=quick/brief) ). 

## Running
Encrytion: run --package crypto-min --bin crypto-min -- -e {path_to_unencrypted_file} -p {password}
Decryption: run --package crypto-min --bin crypto-min -- -d {path_to_encrypted_file} -p {password}

### TODOs
1. Add a flag enabling double encryption. 
2. Strengthen the encryption key by default. E.g. if someone gives a password of less than 5 characters, append its' reverse to the end, or some bytes derived from the pass.
3. Add padding (see above)