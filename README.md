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
["bemix", "clunk", "grypt", "vozhd", "waqfs"] [43.245s]
["bling", "jumpy", "treck", "vozhd", "waqfs"] [59.105s]
["blunk", "cimex", "grypt", "vozhd", "waqfs"] [64.941s]
["brick", "glent", "jumpy", "vozhd", "waqfs"] [83.935s]
["brung", "cylix", "kempt", "vozhd", "waqfs"] [89.169s]
["chunk", "fjord", "gymps", "vibex", "waltz"] [125.002s]
["clipt", "jumby", "kreng", "vozhd", "waqfs"] [128.169s]
["fjord", "gucks", "nymph", "vibex", "waltz"] [176.294s]
["glent", "jumby", "prick", "vozhd", "waqfs"] [192.328s]
["jumby", "pling", "treck", "vozhd", "waqfs"] [205.250s]
Total time: 211.691s`
```
