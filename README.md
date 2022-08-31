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

Result (Single Threaded as of [460f57a](https://github.com/CyberThink777/wordle5/commit/460f57aeeefac58e25ca351d7f5001a2c0fd4a8d)):
```
["bemix", "clunk", "grypt", "vozhd", "waqfs"] [21.880s]
["bling", "jumpy", "treck", "vozhd", "waqfs"] [31.461s]
["blunk", "cimex", "grypt", "vozhd", "waqfs"] [36.090s]
["brick", "glent", "jumpy", "vozhd", "waqfs"] [48.229s]
["brung", "cylix", "kempt", "vozhd", "waqfs"] [51.457s]
["chunk", "fjord", "gymps", "vibex", "waltz"] [73.271s]
["clipt", "jumby", "kreng", "vozhd", "waqfs"] [75.841s]
["fjord", "gucks", "nymph", "vibex", "waltz"] [108.237s]
["glent", "jumby", "prick", "vozhd", "waqfs"] [115.445s]
["jumby", "pling", "treck", "vozhd", "waqfs"] [122.159s]
Time: 126.575s
```
