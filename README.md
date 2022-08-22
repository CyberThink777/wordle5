Rust impl of [fredoverflow solution](https://github.com/fredoverflow/wordle) on
Original Matt Parker's Video [Can you find: five five-letter words with twenty-five unique letters?](https://youtu.be/_-AfhLQfb6w)
<br>
Solution example:
```
FJORD -> GUCKS -> NYMPH -> VIBEX -> WALTZ
Q is unused
```
Constraints:
- No duplicate letters (valid words have 5 unique characters)
- Order of letters irrelevant (ignore anagrams during search)
- i.e. Word is Set of Characters

wordlist.txt is taken by concatenating cfreshman 
[wordle-nyt-allowed-guesses](https://gist.github.com/cfreshman/40608e78e83eb4e1d60b285eb7e9732f) 
and [wordle-nyt-answers-alphabetical](https://gist.github.com/cfreshman/a7b776506c73284511034e63af1017ee)

Result (Single Threaded as of [675c226](https://github.com/CyberThink777/wordle5/commit/675c2265f871417329a43e7c19a45d49e12d77c6)):
```
["bemix", "clunk", "grypt", "vozhd", "waqfs"] [24.816s]
["bling", "jumpy", "treck", "vozhd", "waqfs"] [34.505s]
["blunk", "cimex", "grypt", "vozhd", "waqfs"] [39.342s]
["brick", "glent", "jumpy", "vozhd", "waqfs"] [50.708s]
["brung", "cylix", "kempt", "vozhd", "waqfs"] [53.322s]
["chunk", "fjord", "gymps", "vibex", "waltz"] [78.501s]
["clipt", "jumby", "kreng", "vozhd", "waqfs"] [83.149s]
["fjord", "gucks", "nymph", "vibex", "waltz"] [119.038s]
["glent", "jumby", "prick", "vozhd", "waqfs"] [128.946s]
["jumby", "pling", "treck", "vozhd", "waqfs"] [137.380s]
Time: 140.427s
```
