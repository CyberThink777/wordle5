Rust impl of [fredoverflow solution](https://gist.github.com/fredoverflow/a7c9f230d86787ed6164ef652f675d8d) on
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
