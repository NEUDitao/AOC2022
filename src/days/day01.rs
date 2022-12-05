use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

/*
Day 1 really was just another warm up round! This time, in Rust though, there's
a lot of boilerplate that has to be done. Shoutout to @agubelu for his AOC
rust template (https://github.com/agubelu/AoC-rust-template), which does a lot of
cool stuff for me!

This was my first time playing with a Rust environment that actually has autocomplete,
type inlay hints, go-to-definitions, etc. That's mostly because currently, Meta
Rust development is still a struggle (lots of dependencies to juggle!), but it's
getting better there all the time.

Still, now I finally understand why Rust's ecosystem/community is touted so
highly, there's a lot of cool stuff going on here. There was also a lot of
noise. I turned off most of the inlay hints, and a lot of times the
autocomplete is also getting in my way, since it's recommending tooooo many
things to me.

Anyways, in terms of actually thinking about today's problems, there's not
much to be said. The goal of AOC every year for me (well, except 2021) isn't
to get the solution the fastest, but to get something reasonably pretty.

However, I'm going to allow myself unwraps and panics in Rust, since I don't want
to spend that much time appeasing the type checker. (installing anyhow just to throw
safe errors on what're essentially scripts seems like a massive waste of time)

Anyways, for this day 1, it's mostly about getting used to actually messing around
with the inputs. While normally, I wouldn't want to use mutable, there's no actual
way of sorting without making a clone. (I guess under the hood, we could abuse the
heck out of the stack, or hope we have enough heap allocated space, but that's still
time spent in memory, which as we all know is slow). And, that seems like a waste, so
part 2 made me change calorie_counts to mutable.

Part 1 was done with the max fn because I didn't know I'd have to take the top 3. It
could also be done in retrospect with just getting the first. Oh well
*/

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01").unwrap();

    let new_input = input.split("\n\n");

    let mut calorie_counts = new_input
        .into_iter()
        .map(|single_elf| {
            single_elf
                .split("\n")
                .into_iter()
                .map(|calories| str::parse::<i32>(calories).unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calorie_counts.sort();
    calorie_counts.reverse();

    // Your solution here...
    let sol1 = *calorie_counts.iter().max().unwrap();
    let sol2 = calorie_counts.into_iter().take(3).sum::<i32>();

    (Solution::from(sol1), Solution::from(sol2))
}
