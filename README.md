# Implementing Test Driven Development By Example in Rust

To aid in learning rust, this repository is an attempt to implement the
first example in the TDD book with Rust.
Commits will correspond to chapters and will indicate so.

**STATUS**: Presently this repository is done up until Chapter 14, and the final chapter 15 is
pending as it is not very clear regarding the benefits of changing to expression and most of the
functionality is existing in the current state. (Except multiple addition for `Moneys`).

## Learnings

Below are the learnings and take-aways from the book\*

- Make a list of the tests that are needed to be working and maintain it as a to-do list
- The key steps in the practice are:
  > - Add a little test (break tests down)
  > - Run all tests and fail
  > - Make a change
  > - Run the tests and succeed
  > - Refactor to remove duplication (do not skip this!)

Strategies to get tests working:

- **Fake it**: Return a constant and gradually replace constants with variables until you have the real code
- **Use Obvious Implementation**: Type in the real implementation only if it is obvious
- **Triangulation**: We only generalize code when we have two examples or more, when the second example demands a more general solution only then should we generalize the code

Good wisdom:

- "If two tests faile at once we are sunk"
- If you can't tackle a big test, invent smaller tests to represent progress.
- Reconcile duplicate implementations by moving code from a class to a superclass and then remove redundant implementations.
- Do not introduce more design until there is better motivation, using things like `getClass()`, `isInstanceOf` in the _interim_ is alright
- Break big tests to smaller ones, eg. instead of writing a test for adding `$` with `CHF` write one for `$` with `$` this will help you get started

\*
**Disclaimer**: All of the content in this README are for learning purposes and are points noted
for making the points learnt explicit and for ensuring easier recall. There is no intention to
stake any claims for the work of the book, the author of the book is Kent Beck and the book is
titled Test-Driven Development By Example and I highly recommend it.
