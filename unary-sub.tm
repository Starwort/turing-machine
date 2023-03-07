// Subtracts two unary numbers. The first number is inputted with 0s and the second with 1s
// Assumes blank is #
// Positive results are represented as a unary number written with 0s
// Negative results are represented as a unary number written with 1s
// Example: 0001111 -> 1 (-1)
// Example: 0000111 -> 0 (1)
A 0/0/>/B 1/1/? #/#/?
B 0/0/> 1/1/> #/#/</C
C 0/0/? 1/#/</D #/#/?
D 1/1/< 0/0/< #/#/>/E
E 0/#/>/A 1/1/? #/#/?