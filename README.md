# WordleHelper

## Building

```bash
cargo build --release
```

## Sample

```
$ wordle_helper
Please input command: suggest
raise - 5.877909690821486 bits
slate - 5.855775376955972 bits
crate - 5.834874004263524 bits
irate - 5.831396980440792 bits
trace - 5.830548713859446 bits
Please input command: update
Please input word: trace
Please input result: BOBBB
Please input command: suggest
surly - 4.6693981953738835 bits
sorry - 4.63502739943295 bits
hurry - 4.585229102437745 bits
lorry - 4.557207868318167 bits
gourd - 4.511587797403417 bits
Please input command: update
Please input word: sorry
Please input result: BOOBB
Please input command: suggest
minor - 2.584962500721156 bits
flour - 2.251629167387823 bits
humor - 2.2516291673878226 bits
rhino - 2.2516291673878226 bits
vigor - 2.2516291673878226 bits
Please input command: update
Please input word: minor
Please input result: OBBGG
Please input command: suggest
humor - 0 bits
Please input command: 
Exiting...
```
