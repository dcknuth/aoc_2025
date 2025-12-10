# AOC 2025
Advent of code 2025, twelve days this year

This year I'm going to try to create a Python template to help me first solve the puzzle quickly in Python. Then, after both stars are in hand for that day, I will do a version in Rust along with any optimizations that seem interesting. The first few days might see both in the same day, but later days I would expect the Rust version to trail by days to weeks.  
I will also look at making some commonly used helper/example Python files. Things like a linked list, an example using NetworkX and a 2D matrix map load. These seem common enough in AOC and would be good practice to create

## Templates and Examples
* Python Template: I did make a template file for Python and it seemed fine when used with day 1. It's very simple, but includes a decorator to time the parts
* Linked List Class: ...

## Python
* Day 1: Getting to a version that runs for both parts was straight forward. I didn't try any optimization as it seemed the type of puzzle where anything would finish pretty quickly. Part 1 was 0.001 seconds and part 2 was 0.04 seconds. I should have started on-time, made a file for the day and changed variables for the day ahead of time to get the two stars faster. In a day or two, I'm sure these will not matter ;-)
* Day 2: Part 1 was not too bad and part 2 was not terrible, but did take a bit of debugging to fix several errors. Part 2 also ran slow, but I think I will wait for the Rust version to see about making that better (tried regex with Rust instead, so still waiting on that)
* Day 3: I completed Part 1 in reasonable time, but I got bogged down by the new logic needed and a couple bugs in part 2. I need to think about moving to more, smaller functions over getting a first answer to try. Part 1 was 3.4 ms and part 2 6.0 ms
* Day 4: Some code to look around in the map, but not too bad and much could be reused for part 2. Part 1 took 11 ms and Part 2 was 190 ms
* Day 5: First day where part 2 cannot be solved with brute force. This was also the first time I ran out of memory instead of just waiting a long time and killing off the run. I used a range collapsing approach to solve (this has worked in previous years and I suspected straight away would be needed here). Could use clean-up, but done for now. Part 1 took 5.6 ms and part 2 34.7 ms
* Day 6: First part was easy, but the second part had a few formatting tricks. Errors in these needed to be debugged for a bit. Part 1 took 1.6 ms and part 2 3.7 ms
* Day 7: Like Day 5, the first part was fine, but the second part needed an elegant solution of tracking position sums by row. Part 1 ran in 2.8 ms and part 2 took 2.4 ms
* Day 8: This was the first day where I thought part 1 was also tricky. I started with a set based solution, but that was falling apart a bit. I moved to a graph solution with NetworkX and that worked pretty well. For part 2, I redid a lot of part 1 and checked for full connection after adding each pair. I could have saved some time in part 2 by reusing part 1 variables, but didn't want to move the scope of the variables, just getting the stars. I will probably see if I can reuse on the Rust version, but that will mean a apples to oranges comparison of run times. Part 1 took 0.72 seconds and part 2 was 4.4 seconds, so run times are increasing
* Day 9: Part 1 was quick to solve and run, but part 2 required some thought. First idea worked, but took a some time to run. Will save optimization ideas for the Rust implementation. Part 1 took 0.02 seconds and part 2 took 8.3 seconds
* Day 10: ...


## Rust
* Day 1: Did you know that Rust "%" is not modulus, but remainder and can be negative? I also made part 2 in Rust more efficient, because it's Rust and that's part of the choice. Those two items made it take way longer than I though it would. Part 1 and 2 each took about 0.0003 seconds
* Day 2: Figured I would try the Rust regex library for this one. Had to figure out why grouping does not work. It's because the standard Rust regex crate does not support it for speed reasons. No problem, use the fancy-regex library. It took 58 seconds for part 1 with a short regex. Part 2 only needed a small regex change to work, but was also very slow at 59 seconds. It also seemed hard to manipulate stings in the right way to get everything to work (str vs. &str vs. String vs. &String). Guess I still need more practice with this in Rust. I will leave improving the speed for later, but I'm guessing the Python approach would run OK and would respond well to a thread pool to take each range with
* Day 3: Some frustration as I continue to get used to Rust's argument passing and ownership. Since I knew what was coming in part 2, I made a function that could take the number of batteries used as a parameter. Part 1 ran in 0.14 ms and part 2 in 0.20 ms. Since I used the same approach as with Python, probably a normal Rust speed-up factor
* Day 4: Not too bad to move to Rust. I could have refactored part1 to serve for part2 and had less code. Part 1 took 0.23 ms and part 2 took 3.7 ms. Since this was comparable to the Python approach, Rust was ~50x faster for each part
* Day 5: I looked to write a part 1 with ranges that could be reused for part 2. Again, it took some time to get this all typed correctly. Part 1 took 0.53 ms and part 2 was 200 nano seconds. So part 1 was a 10x speed-up vs. Python, but since it was also merging ranges in part 1, part 2 was a >100,000x speed-up. More fairly, a 76x speed-up overall
* Day 6: I asked Copilot if parse() takes care of whitespace for you. It said yes, but actually, it does not. That combined with the usual argument passing and ownership learning made the second part take a while. Part 1 ran in 0.11 ms and part 2 was 0.58 ms. If I pass back through, I might be able to remove some clone()s in part 2 

## Things Learned This Year (in progress)
* Python is slower, but so easy. Yes I know it better at this point, however I think some of the following points will lend that observation some credibility
* The Rust operator % is remainder and not modulus. I assume this was done because remainder is faster to compute and is the same in many cases. In AOC, the wrap-around capability of modulus (even to a negative) is usually what I seem to want
* The default regex crate in Rust is missing some very common functionality. Every regex implementation seems to be a little different, but the Python one seems to hit the mark better
* The type matching in Rust means that is takes me longer to generate my Rust implementation, even when I already know the issues from having done the Python implementation. I don't think this would actually be the case in a very complex piece of software, but in AOC (short puzzle) it seems to add a lot of time. In day 3 I passed a vector incorrectly and it did not produce a compile error. The spot where the debugger showed the impact of that was also on a different line of the code when I made an unrelated vector change and was very confusing to debug. I assume I will get better at this over time, but it was time consuming to learn how this manifests and type checking does not 100% save you from passing values incorrectly
