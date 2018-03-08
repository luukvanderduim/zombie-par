# zombie-par
  Parallel bridge riddle solver in Rust

This program solves the TED-ed bridge riddle by Alex Gendler:

https://ed.ted.com/lessons/can-you-solve-the-bridge-riddle-alex-gendler

Besides solving the puzzle it more so serves as test bed-project to learn some Rust.

The solver uses a naive depth-first backtracker to find a solution.
It does so by:
  keeping track of states on both sides of the bridghe.
  Which laboratory staff member is on which side of the bridge.
  Depending on where we are in the cross-sequence, it will either
  - generate a list of possible duos who could cross or 
  - pick a lab-staff member to bring back the lantern.
When a sequence ends, meaning the current crossing structure is complete,
the current structucture will compare itself with the fastest known crossing
and will update the fastest if the current crossing was even more efficient.

The rayon crate is used to parallelize the search. 
However, because rayon does some more sophistaced accounting, work-stealing load balancing, 
the parallel searching is slower the single threaded version.

During its search it will print the most efficient solution up until that point in time 
and show how many iterations it took before it found that solution.

