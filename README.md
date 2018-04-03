# zombie-par
  Parallel bridge riddle solver in Rust

This program solves the TED-ed bridge riddle by Alex Gendler:

https://ed.ted.com/lessons/can-you-solve-the-bridge-riddle-alex-gendler

Besides solving the puzzle, this program more so serves as test bed project to familiarize myself with the Rust programming language.

The solver uses a naive depth-first backtracking to find a solution.
It does so by:
  Keeping track of all states of 'Which laboratory staff members are on which side of the bridge'.
    Depending on where we are in the cross-sequence, it will either
  - generate a list of duos who can cross or
  - pick a lab-staff member to bring back the lantern.
When a sequence is satisfied, meaning the current crossing structure is fully saturated,
the current crossing is compared to the fastest known crossing.
and will update the fastest if the current crossing was even more efficient.

The Rayon crate is used to parallelize the search by using the 'parallel iterator'.
However, because rayon does some sophistaced accounting, work-stealing load balancing,
the parallel searching is actually slower than the single threaded version.

Duration of main loop is shown at the end.
