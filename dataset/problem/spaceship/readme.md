Welcome to the Spaceship course!

In 2020, most of us have learned how to operate a spaceship. In this course we'll play a small chess-like game featuring the spaceship! The game operates on an infinite 2D chess board, with the spaceship initially located on `(0,0)`. The spaceship has a velocity `vx` and `vy`, which are initially both set to `0`. In each turn the player can increase/decrease each of those numbers by at most one, and then the piece moves `vx` steps to the right and `vy` steps up.

Moves are represented with a single digit, inspired by the old numeric pad on a computer keyboard that we used to have in the old days on Earth. For example, `7` means decreasing `vx` and increasing `vy` by `1`, while `6` means increasing `vx` by `1` and keeping `vy` the same. A path can then be represented by a sequence of digits, e.g. the path `236659` visits, in this order, the following squares: `(0,0) (0,-1) (1,-3) (3,-5) (6,-7) (9,-9) (13,-10)`.

Now the challenge is the following: given a list of squares to be visited, find a sequence of moves that visits all those squares. Your solution may consist of at most `10,000,000` moves.

The following levels are available:
* [spaceship1] Best score: 5.
* [spaceship2] Best score: 49.
* [spaceship3] Best score: 10.
* [spaceship4] Best score: 99.
* [spaceship5] Best score: 116.
* [spaceship6] Best score: 117.
* [spaceship7] Best score: 94.
* [spaceship8] Best score: 90.
* [spaceship9] Best score: 213.
* [spaceship10] Best score: 305.
* [spaceship11] Best score: 8192.
* [spaceship12] Best score: 8192.
* [spaceship13] Best score: 23791.
* [spaceship14] Best score: 137.
* [spaceship15] Best score: 40.
* [spaceship16] Best score: 1572.
* [spaceship17] Best score: 440.
* [spaceship18] Best score: 1959.
* [spaceship19] Best score: 13004.
* [spaceship20] Best score: 2394.
* [spaceship21] Best score: 2437.
* [spaceship22] Best score: 1269.
* [spaceship23] Best score: 186177.
* [spaceship24] Best score: 631912.
* [spaceship25] Best score: 719942.

To submit a solution, send an ICFP expression that evaluates to:

```
solve spaceshipX moves
```

Your score is the number of moves, so a lower score is better.
