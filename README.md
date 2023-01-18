"# rust-guess-num" 

Try to guess a 4-digit number from 0~9 with the right order sequence:

1. The computer will first generates a 4 digit number in the background
2. User input any 4-digit number
3. If one of the number matches but incorrect order, it will count as B, which is 1B
4. If one of the number matches also in the right order, it will count as A, which is 1A
5. If none of the number matches, it will shows 0A and 0B
6. Your target is to get 4A to win
7. No duplicates such as 4444 or 4423  

For example:
Target number is [4159]

User inputs：1234

--> 0A and 2B

User inputs: 5678

--> 0A and 1B

User inputs: 6789

--> 1A and 0B

so on....

Hint: Try to eliminate numbers that are not in the sequence 

Build and cargo run to play it!

