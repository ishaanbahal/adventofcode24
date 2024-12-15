## AdventOfCode 2024

Contains my solutions to the AdventOfCode 2024 puzzles, using Rust.

To know what AdventOfCode is, check the [official website](https://adventofcode.com/).

> These by no means are the best solutions to the puzzles, but they are the ones I came up with. The exercise is meant to learn rust. This repository also contains the test data for my user, which you should change for your user if you're following along. Every exercise requires a `testfile`, which is the input data for the exercise, simply modify create a file in the exercise directory and add test data to it.

### Running the solutions

To run the solutions, use the following command:

```bash
cargo run 1_1
```

This will run the solution for the first exercise of the first day. The solution will be printed to the console.
To run any other exercise, simply change the argument to the command, for example if you want to run the solution for  day 2, problem 1

```bash
cargo run 2_1
```

## Solutions
These are my solutions, with runtimes! :sunglasses:

| Day | Title | 1 :running_man: | 2 :running_man: | Part 1 | Part 2 |
|:-|:-|:-|:-|:-|:-|
| [01](https://adventofcode.com/2024/day/1)  | Historian Hysteria              | 373µs | 393µs | [Exercise1_1.rs](./src/runner/exercise1_1/exercise1_1.rs) | [Exercise1_2.rs](./src/runner/exercise1_2/exercise1_2.rs) |
| [02](https://adventofcode.com/2024/day/2)  | Red-Nosed Reports               | 521µs | 1.5ms | [Exercise2_1.rs](./src/runner/exercise2_1/exercise2_1.rs) | [Exercise2_2.rs](./src/runner/exercise2_2/exercise2_2.rs) |
| [03](https://adventofcode.com/2024/day/3)  | Mull it Over                    | 506µs | 560µs | [Exercise3_1.rs](./src/runner/exercise3_1/exercise3_1.rs) | [Exercise3_2.rs](./src/runner/exercise3_2/exercise3_2.rs) |
| [04](https://adventofcode.com/2024/day/4)  | Ceres Search                    | 3.7ms | 657µs | [Exercise4_1.rs](./src/runner/exercise4_1/exercise4_1.rs) | [Exercise4_2.rs](./src/runner/exercise4_2/exercise4_2.rs) |
| [05](https://adventofcode.com/2024/day/5)  | Print Queue                     | 511µs  | 795µs  | [Exercise5_1.rs](./src/runner/exercise5_1/exercise5_1.rs) | [Exercise5_2.rs](./src/runner/exercise5_2/exercise5_2.rs) |
| [06](https://adventofcode.com/2024/day/6)  | Guard Gallivant                 | 313µs | 740µs | [Exercise6_1.rs](./src/runner/exercise6_1/exercise6_1.rs) | [Exercise6_2.rs](./src/runner/exercise6_2/exercise6_2.rs) |
| [07](https://adventofcode.com/2024/day/7)  | Bridge Repair                   | 1.80ms | 19.3ms  | [Exercise7_1.rs](./src/runner/exercise7_1/exercise7_1.rs) | [Exercise7_2.rs](./src/runner/exercise7_2/exercise7_2.rs) |
| [08](https://adventofcode.com/2024/day/8)  | Resonant Collinearity           | 781µs | 996µs  | [Exercise8_1.rs](./src/runner/exercise8_1/exercise8_1.rs) | [Exercise8_2.rs](./src/runner/exercise8_2/exercise8_2.rs) |
| [09](https://adventofcode.com/2024/day/9)  | Disk Fragmenter                 | 1.73ms | 90.8ms | [Exercise9_1.rs](./src/runner/exercise9_1/exercise9_1.rs) | [Exercise9_2.rs](./src/runner/exercise9_2/exercise9_2.rs) |
| [10](https://adventofcode.com/2024/day/10) | Hoof It                         | 998µs  | 868µs  | [Exercise10_1.rs](./src/runner/exercise10_1/exercise10_1.rs) | [Exercise10_2.rs](./src/runner/exercise10_2/exercise10_2.rs) |
| [11](https://adventofcode.com/2024/day/11) | Plutonian Pebbles               | 916µs  | 18.1ms | [Exercise11_1.rs](./src/runner/exercise11_1/exercise11_1.rs) | [Exercise11_2.rs](./src/runner/exercise11_2/exercise11_2.rs) |
| [12](https://adventofcode.com/2024/day/12) | Garden Groups               | 6.08ms  | NA | [Exercise12_1.rs](./src/runner/exercise12_1/exercise12_1.rs) | [Exercise12_2.rs](./src/runner/exercise12_2/exercise12_2.rs) |
| [13](https://adventofcode.com/2024/day/13) | Claw Contraption               | 60µs  | 62µs | [Exercise13_1.rs](./src/runner/exercise13_1/exercise13_1.rs) | [Exercise13_2.rs](./src/runner/exercise13_2/exercise13_2.rs) |
| [14](https://adventofcode.com/2024/day/14) | Restroom Redoubt               | 725µs  | manual | [Exercise14_1.rs](./src/runner/exercise14_1/exercise14_1.rs) | [Exercise14_2.rs](./src/runner/exercise14_2/exercise14_2.rs) |
| [15](https://adventofcode.com/2024/day/15) | Restroom Redoubt               | 4ms  | 5ms | [Exercise15_1.rs](./src/runner/exercise15_1/exercise15_1.rs) | [Exercise15_2.rs](./src/runner/exercise15_2/exercise15_2.rs) |


## Notes
1. Day 13, linear equation solving, though unsure why the results are in floating points when done on paper, when for sample case the integer values satify the equation. Need to read up on this.
