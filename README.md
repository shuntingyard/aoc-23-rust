# My Rusty Advent of Code 2023 🎄

My *framework* here is a copy of Christopher Biscardi's
[implementation]( https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust)
for a rusty take in 2023.

*Solutions* are mine.

## Compte Rendu

1. Was fun and not hard, tried some `nom`.
2. Quite like 1. - learning more `nom`.
3. Learned iterator things, had the right idea but even my part1 was too buggy
    to get a star.
4. Easy and fun again. Most time was spent setting up nom to read cards correctly.
    Modifying part1's iterator constructs to move a BTreeMap into a closure
    for tracking the number of repetitions for each card was a breeze...
5. Mapping seeds to soils to ... locations - was a bit hard.

    - part1: interesting use of Range to do the routing. Used fold_many1 for parsing.
    - part2: skipped
