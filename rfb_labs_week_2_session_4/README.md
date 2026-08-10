# Rust for Bitcoin 2.0 — Week 2, Session 4

Build a small lending library while practising structs, enums, traits,
ownership, borrowing, collections, and `Result`-based error handling. No
Bitcoin and no external crates — just Rust.

The crate is intentionally incomplete. Search for `TODO` and implement each
part; do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Part 2 in `error.rs`, then Part 3 in `library.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits in Part 4 and the two operations in Parts 5–6.
5. Run the ownership experiments and record the errors.
6. Build the demo in `main.rs`.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in
the submission.

## Written answers

Answer in your own words. Add both ownership compiler errors from Part 7 as
fenced text blocks, then explain what caused each.

1. Why is `LoanStatus` an enum rather than a `bool` plus two `Option` fields?
2. What does `match` force you to do when a fourth `MediaKind` is added later?
3. `Item::new` takes `String` rather than `&str`. Who owns the title afterwards?
4. Why does `add_item` take `self` by `&mut` but `item` by value?
5. When `add_item` returns `Err`, what happened to the `Item` the caller passed
   in? Was that a good design choice, and what is the alternative?
6. Why does `find_item` return `Option<&Item>` rather than `Option<Item>`?
7. What is the lifetime `'a` in `items_by_author` actually saying?
8. Why can't `checkout` hold a `&mut Item` and a `&mut Member` from the same
   `Library` at once, and how did you structure the method around that?
9. Why are `Library`'s fields private?
10. What duplication does the provided `late_fee_cents` remove, and what would
    you lose by making it a free function instead?
11. Why is `Result` preferable to `panic!` for validation failures? Name a
    place in this crate where a panic would be defensible.
12. Which derive did you deliberately leave off a type, and why?

## Design notes

Describe any choices you made, including how you kept an item's status and its
borrower's list from drifting apart, and (if attempted) the optional generic
search.

## Example output

Paste the output of `cargo run` here once Part 8 is complete.

## ASSIGNMENT ANSWERS
1. Why is `LoanStatus` an enum rather than a `bool` plus two `Option` fields?

    =Loan Status is not a struct of data types bool,Option<u32> and Option<u32> but an enum because a bool + 2 Options<u32> would construct an impossible combination,like loan = false while member_id = Some(100),claiming a book is both on the shelf and checked out to someone at the same time .The compiler has no way to catch that contradiction because the 3 fields aren't linked;they're just 3 independent boxes you could fill in inconsistently and an enum fixes this by attaching the extra data (member_id), (day_borrowed) only to the OnLoan variant Available and Lost simply have no fields to fill in incorrectly. this makes the invalid state - "available but also on loan" -impossible to even write down,not just something you have to remember not to do.

    Because LoanStatus is an enum,matching on it is exhaustive the compiler requires every variant (Available,OnLoan {..}) to be handled, or an explicit _ fallback.This guarantees you can never accidnetally forget to handle one of the 3 states.(e.g forgetting what happens when an item is Lost).If a new variant is added later, the compiler will flag every match that needs updating,so nothing is silently skipped. 
2. What does `match` force you to do when a fourth `MediaKind` is added later?


    