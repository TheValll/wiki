# Rust Exercises

Progressive exercises covering Parts 1-7 of the wiki.

---

## Level 1 — Fundamentals (Parts 1-4)

### Exercise 1: Unit Converter

Build an interactive unit converter with a menu loop.

**Requirements:**
- Display a menu: km→miles, kg→lbs, °C→°F (and reverse)
- The user picks a conversion, enters a value, gets the result
- Loop until the user types "quit"
- Handle invalid input gracefully

**Concepts:** `loop`, `match`, `mut`, functions, `f64`, `String`, `parse()`

**Hints:**
- 1 km = 0.621371 miles
- 1 kg = 2.20462 lbs
- °F = °C × 9/5 + 32

---

### Exercise 2: Custom FizzBuzz

A twist on the classic FizzBuzz.

**Requirements:**
- Ask the user for two divisors and two words (e.g., 3 → "Fizz", 5 → "Buzz")
- Print numbers from 1 to 100, replacing multiples accordingly
- If a number is a multiple of both, print both words concatenated
- Extract the core logic into a function: `fn fizzbuzz(n: u32, div1: u32, word1: &str, div2: u32, word2: &str)`

**Concepts:** `for`, `if/else`, functions with parameters, `String`, `&str`

---

## Level 2 — Ownership & Borrowing (Parts 5-6)

### Exercise 3: Word Analyzer

Write a function that analyzes a sentence.

**Requirements:**
- `fn longest_word(s: &str) -> &str` — returns a slice of the longest word
- `fn word_count(s: &str) -> usize` — returns the number of words
- `fn first_word(s: &str) -> &str` — returns the first word as a slice
- Test with both `String` and string literals

**Concepts:** slices, `&str`, iterators, borrowing, returning references

**Bonus:** Write `fn words_longer_than(s: &str, min_len: usize) -> Vec<&str>` that collects all words longer than `min_len`.

---

### Exercise 4: Swap Without Clone

Explore ownership vs. borrowing through a swap exercise.

**Requirements:**
1. Write `fn swap_owned(a: String, b: String) -> (String, String)` — takes ownership, returns swapped
2. Write `fn swap_ref(a: &mut String, b: &mut String)` — swaps in place via mutable references
3. In `main`, call both and print results
4. Try using `a` after calling `swap_owned` — observe the compiler error
5. Try using `a` after calling `swap_ref` — observe that it still works

**Concepts:** move semantics, `&mut`, ownership transfer vs. borrowing

**Bonus:** Can you swap two `String` fields inside a single struct using `std::mem::swap`?

---

## Level 3 — Structs & Methods (Part 7)

### Exercise 5: Book Library

Model a simple book with structs and methods.

**Requirements:**
- Define `struct Book` with fields: `title: String`, `author: String`, `pages: u32`, `read: bool`
- Derive `Debug`
- Implement:
  - `Book::new(title: &str, author: &str, pages: u32) -> Self` — associated function, `read` defaults to `false`
  - `summary(&self) -> String` — returns `"Title by Author (X pages)"` or `"Title by Author (X pages) [READ]"`
  - `mark_as_read(&mut self)` — sets `read` to `true`
  - `is_longer_than(&self, other: &Book) -> bool` — compares page count
- In `main`, create a few books, mark some as read, print summaries, compare lengths

**Concepts:** struct, `impl`, `&self`, `&mut self`, associated functions, `#[derive(Debug)]`

---

### Exercise 6: RPG Combat System

Simulate a turn-based fight between two fighters.

**Requirements:**
- Define `struct Fighter` with fields: `name: String`, `hp: i32`, `attack: u32`, `defense: u32`
- Derive `Debug`
- Implement:
  - `Fighter::warrior(name: &str) -> Self` — high HP/defense, lower attack (e.g., 100 hp, 15 atk, 10 def)
  - `Fighter::mage(name: &str) -> Self` — lower HP/defense, high attack (e.g., 70 hp, 25 atk, 5 def)
  - `is_alive(&self) -> bool`
  - `take_damage(&mut self, amount: u32)` — reduce HP by `amount - defense` (minimum 1 damage)
  - `attack_target(&self, target: &mut Fighter)` — print who attacks whom, call `take_damage`
  - `status(&self) -> String` — returns `"Name: X HP"`
- In `main`, create two fighters and loop:
  - Fighter A attacks Fighter B
  - If B is dead, A wins — break
  - Fighter B attacks Fighter A
  - If A is dead, B wins — break
  - Print status after each round

**Concepts:** structs, methods, `&self` + `&mut`, associated functions, `loop`, `if`, `break`

---

## Level 4 — Capstone Project (Everything)

### Exercise 7: Contact Manager (Mini CRM)

Combine all concepts into one interactive program.

**Requirements:**
- Define:
  ```
  struct Contact {
      name: String,
      email: String,
      phone: String,
      favorite: bool,
  }
  ```
- Derive `Debug`
- Implement:
  - `Contact::new(name: &str, email: &str, phone: &str) -> Self`
  - `display(&self) -> String` — formatted output, with a star if favorite
  - `mark_favorite(&mut self)`
- Store contacts in a `Vec<Contact>`
- Interactive menu loop:
  1. **Add** — prompt for name, email, phone
  2. **List** — print all contacts (numbered)
  3. **Search** — take a `&str` query, find contacts whose name contains the query (case-insensitive)
  4. **Favorite** — pick a contact by number and toggle favorite
  5. **Quit** — exit the loop
- Search function signature: `fn search_contacts(contacts: &[Contact], query: &str) -> Vec<&Contact>`

**Concepts:** Cargo project, variables, mutability, loops, match, functions, ownership, borrowing, slices, `Vec`, structs, methods, associated functions, `#[derive(Debug)]`

---

## Bonus — Challenge

### Exercise 8: Calculator with History

Build a calculator that parses expressions and keeps a history.

**Requirements:**
- Define:
  ```
  struct Operation {
      left: f64,
      operator: char,
      right: f64,
      result: f64,
  }
  ```
- Derive `Debug`
- Implement:
  - `Operation::from_input(input: &str) -> Option<Self>` — parse `"3 + 5"` into an `Operation`, return `None` if invalid
  - `display(&self) -> String` — returns `"3 + 5 = 8"`
- Support operators: `+`, `-`, `*`, `/` (handle division by zero)
- Store history in `Vec<Operation>`
- Interactive commands:
  - `calc` or just type an expression — compute and add to history
  - `history` — print all past operations
  - `last` — print the most recent operation
  - `quit` — exit

**Concepts:** `&str` parsing, `split`, `match`, `Option`, slices, structs, `impl`, ownership of `Vec`, `loop`

---

## Tips

- Create each exercise as its own Cargo project: `cargo new exercise_name`
- Start with the simplest version, then add features incrementally
- When the compiler gives an error about ownership or borrowing, **read the full error message** — it usually tells you exactly what to do
- Use `cargo clippy` to get idiomatic Rust suggestions
