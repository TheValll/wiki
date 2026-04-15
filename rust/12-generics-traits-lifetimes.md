# Part 12 — Generic Types, Traits & Lifetimes

Every language needs tools to handle duplication. Rust provides three that work together:

| Tool | Purpose |
|------|---------|
| **Generics** | Write one definition that works with many types |
| **Traits** | Describe shared behavior types can opt into |
| **Lifetimes** | Tell the compiler how references relate to each other |

All three are **compile-time only** — zero runtime cost.

---

## 12.1 — Removing Duplication with Generic Types

### The motivation: two nearly identical functions

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

Only the types differ. Copy-pasted logic is a maintenance trap: fix a bug in one, forget the other.

### Generic functions

Introduce a **type parameter** `T` in angle brackets. It reads as "for any type `T`":

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&numbers));       // works for i32

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(&chars));         // works for char
}
```

The `: PartialOrd` is a **trait bound** — without it, `item > largest` wouldn't compile, because not every type supports `>`.

| Convention | Meaning |
|------------|---------|
| `T`, `U`, `V` | Single uppercase letters for type parameters |
| Declared after function name | `fn name<T>(...)` |
| Shows up in the signature | Parameter types, return type |

### Generic structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5,   y: 10  };
let float   = Point { x: 1.0, y: 4.0 };
```

With **one** type parameter, both fields must be the same type. This won't compile:

```rust
let wont_work = Point { x: 5, y: 4.0 };   // ERROR: mismatched types
```

Use **multiple** parameters when fields can differ:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

let mixed = Point { x: 5, y: 4.0 };   // fine now
```

### Generic enums

You've already used them — `Option` and `Result` are defined this way:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

One definition, infinite uses. `Option<i32>`, `Option<String>`, `Result<File, io::Error>` — all the same enum, specialized by the compiler.

### Generic methods

When implementing methods on a generic type, **declare `T` after `impl`** so Rust knows it's generic and not a concrete type called `T`:

```rust
struct Point<T> { x: T, y: T }

impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}
```

You can also write methods for a **specific** instantiation only:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

`distance_from_origin` exists on `Point<f32>` but not on `Point<i32>` — the compiler enforces this.

Methods can have their **own** generic parameters, independent from the struct's:

```rust
struct Point<X1, Y1> { x: X1, y: Y1 }

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}

let p1 = Point { x: 5,       y: 10.4 };
let p2 = Point { x: "Hello", y: 'c'  };
let p3 = p1.mixup(p2);                       // Point<i32, char>
```

### Performance: monomorphization

Generics in Rust have **zero runtime cost**. At compile time, the compiler performs **monomorphization**: it generates a specialized copy of the generic code for each concrete type you actually use.

```
┌──────────────────────────────────────────────────────────────────┐
│  You write:                          The compiler generates:     │
│                                                                   │
│  let a = Some(5);                    enum Option_i32 { ... }      │
│  let b = Some(5.0);                  enum Option_f64 { ... }      │
│                                                                   │
│  One generic Option<T>  →  two separate concrete enums.           │
│  Runtime performance = hand-written duplicated code.              │
└──────────────────────────────────────────────────────────────────┘
```

Trade-off: larger binary (one copy per concrete type) in exchange for zero dispatch cost.

---

## 12.2 — Traits: Defining Shared Behavior

A **trait** is a collection of method signatures that a type can agree to implement. It's how Rust says "these otherwise-unrelated types share this capability."

Close cousins: interfaces (Java), protocols (Swift), type classes (Haskell).

### Defining a trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Note the semicolon — no body. Each implementing type provides its own.

### Implementing a trait on a type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author:   String,
    pub content:  String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content:  String,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Once implemented, `article.summarize()` and `post.summarize()` both work — same method name, type-specific behavior.

### The orphan rule

You can implement a trait on a type **only if** the trait or the type is local to your crate.

```
┌──────────────────────────────────────────────────────────────────┐
│  You CAN:                                                         │
│    • Implement YourTrait for i32      (your trait)                │
│    • Implement Display for YourType   (your type)                 │
│                                                                   │
│  You CANNOT:                                                      │
│    • Implement Display for Vec<T>     (both foreign)              │
│                                                                   │
│  Why: without this rule, two crates could define conflicting      │
│  implementations and the compiler couldn't pick one.              │
└──────────────────────────────────────────────────────────────────┘
```

### Default implementations

A trait method can have a default body — types can use it as-is or override:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}   // empty block — uses default
```

Defaults can call other methods of the same trait, even ones with no body:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;                 // must implement

    fn summarize(&self) -> String {                       // free, uses above
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String { format!("@{}", self.username) }
}
```

### Traits as parameters — `impl Trait`

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

`item` can be any type that implements `Summary`. Short, readable, good default.

### Trait bound syntax (the longer form)

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Subtle but important difference when there are **multiple parameters**:

```rust
// impl Trait — each parameter can be a different concrete type
pub fn notify(a: &impl Summary, b: &impl Summary) { /* ... */ }

// Trait bound — both parameters MUST be the same concrete type
pub fn notify<T: Summary>(a: &T, b: &T) { /* ... */ }
```

| Form | Use when |
|------|----------|
| `&impl Summary` | One-off parameter, don't care if arguments have different types |
| `<T: Summary>` | Need to enforce that multiple parameters share the same type |

### Multiple trait bounds with `+`

```rust
pub fn notify<T: Summary + Display>(item: &T) { /* ... */ }
// or
pub fn notify(item: &(impl Summary + Display)) { /* ... */ }
```

Require both `Summary` **and** `Display`.

### `where` clauses

Long lists of bounds hurt readability. Move them into a `where` clause:

```rust
// Hard to read
fn f<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { /* ... */ }

// Easier
fn f<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    /* ... */
}
```

### Returning `impl Trait`

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content:  String::from("of course, as you probably already know, people"),
    }
}
```

The caller knows "this returns *something* that implements `Summary`" — useful for iterators and closures whose concrete type is ugly or unnameable.

**Restriction:** you can only return **one** concrete type. This won't compile:

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { /* ... */ }       // one type here
    } else {
        SocialPost  { /* ... */ }       // different type here — ERROR
    }
}
```

For that, you need **trait objects** (`Box<dyn Summary>`) — covered in a later chapter.

### Conditional method implementations

Implement methods **only** when the generic type satisfies a trait bound:

```rust
use std::fmt::Display;

struct Pair<T> { x: T, y: T }

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { Self { x, y } }       // always available
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {                             // only for comparable, printable T
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

### Blanket implementations

Implement a trait for **every type** that meets a bound. The standard library does this for `ToString`:

```rust
impl<T: Display> ToString for T {
    // ...
}
```

Result: any type with `Display` automatically gets `.to_string()`:

```rust
let s = 3.to_string();    // works because i32: Display
```

```
┌──────────────────────────────────────────────────────────────────┐
│  Traits + generics = compile-time polymorphism.                   │
│  The caller picks the concrete type; the compiler specializes;   │
│  the runtime pays nothing.                                        │
└──────────────────────────────────────────────────────────────────┘
```

---

## 12.3 — Validating References with Lifetimes

Every reference in Rust has a **lifetime** — the scope over which it's valid. Usually the compiler infers it. Sometimes the relationships are ambiguous and you have to spell them out.

The goal: **prevent dangling references**.

### Dangling references — the thing lifetimes stop

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;           // r points to x
    }                     // x is dropped here
    println!("r: {r}");   // ERROR: x doesn't live long enough
}
```

`r` tries to outlive `x`. In C this would compile and crash (or leak data); in Rust it doesn't compile at all.

### The borrow checker

Conceptually, each reference has an implicit lifetime label. The borrow checker compares them:

```
fn main() {
    let r;                // ┐ 'a (outer)
    {                     // │
        let x = 5;        // │ ┐ 'b (inner)
        r = &x;           // │ │
    }                     // │ ┘   x dropped
    println!("r: {r}");   // │     r still used — but its data is gone!
}                         // ┘
```

`'b` is shorter than `'a`, so the borrow is rejected. The fixed version keeps `x` alive long enough:

```rust
fn main() {
    let x = 5;            // ┐ 'b
    let r = &x;           // ┐ 'a  │
    println!("r: {r}");   // │     │
}                         // ┘     ┘
```

### When you need to write lifetimes — generic lifetimes in functions

This looks fine but doesn't compile:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

```
error[E0106]: missing lifetime specifier
```

**Why?** The compiler can't tell whether the returned reference points to `x` or to `y`, so it can't verify the caller will use it safely. We need to tell it.

### Lifetime annotation syntax

A lifetime parameter starts with `'` and is usually a single lowercase letter:

```rust
&i32          // reference — inferred lifetime
&'a i32       // reference with explicit lifetime 'a
&'a mut i32   // mutable reference with explicit lifetime 'a
```

Annotations don't **change** lifetimes — they just describe relationships between them.

### Annotating the function

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Read as: "for some lifetime `'a`, both `x` and `y` live at least as long as `'a`, and the return value also lives at least as long as `'a`."

The compiler picks `'a` to be the **shorter** of the two inputs' actual lifetimes — that's the longest lifetime it can safely guarantee for the return.

### It enforces what you'd expect

Valid:

```rust
fn main() {
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("{result}");
    }
}
```

Invalid — `result` used after `s2` is dropped:

```rust
fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
    }   // s2 dropped
    println!("{result}");   // ERROR: s2 doesn't live long enough
}
```

### Not all parameters need the same lifetime

If the return doesn't depend on `y`, don't tie their lifetimes together:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

And **the return reference must trace back to a parameter** — you can't return a reference to a local:

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()   // ERROR: returns reference to local data
}
```

Fix: return `String` (owned), not `&str` (borrowed).

### Lifetimes in struct definitions

A struct that holds a reference must declare a lifetime:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_sentence };
}
```

This means: "an `ImportantExcerpt` cannot outlive the `&str` it borrows."

### Lifetime elision — the rules the compiler applies for you

You rarely write lifetimes on simple functions because the compiler applies **three rules** to fill them in. If the rules produce an unambiguous signature, you get away with no annotations.

```
┌──────────────────────────────────────────────────────────────────┐
│  Rule 1: Each reference parameter gets its own lifetime.          │
│    fn foo(x: &i32, y: &i32)                                       │
│  → fn foo<'a, 'b>(x: &'a i32, y: &'b i32)                         │
│                                                                   │
│  Rule 2: If there's exactly ONE input lifetime, it's used for     │
│          all output lifetimes.                                    │
│    fn foo(x: &i32) -> &i32                                        │
│  → fn foo<'a>(x: &'a i32) -> &'a i32                              │
│                                                                   │
│  Rule 3: In a method, if one input is &self or &mut self, its     │
│          lifetime is used for all output lifetimes.               │
└──────────────────────────────────────────────────────────────────┘
```

Example — `first_word` compiles with no annotations:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[0..i]; }
    }
    &s[..]
}
```

Rule 1 gives `s` a lifetime `'a`. Rule 2 applies (exactly one input lifetime) — the output gets `'a` too. Done.

`longest(x: &str, y: &str) -> &str` fails because rule 1 creates **two** lifetimes, rule 2 doesn't fire (not exactly one input), rule 3 doesn't apply (not a method). The compiler can't guess → you must annotate.

### Lifetimes in method definitions

Declare the struct's lifetime after `impl` and after the type name:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

Rule 3 kicks in: `&self`'s lifetime propagates to the return. You don't have to write it.

### The static lifetime

`'static` means "lives for the entire program." All string literals are `&'static str` — they live in the binary:

```rust
let s: &'static str = "I have a static lifetime.";
```

```
┌──────────────────────────────────────────────────────────────────┐
│  Temptation: the compiler suggests `'static` in an error,         │
│  so you slap it on and the error goes away.                       │
│                                                                   │
│  Reality: most of the time the actual bug is a dangling reference │
│  or a wrong lifetime relationship. `'static` hides the bug, it    │
│  doesn't fix it. Only use it when the reference really is good    │
│  for the whole program.                                           │
└──────────────────────────────────────────────────────────────────┘
```

### All three together — generics, trait bounds, and lifetimes

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

One function, three features:

- `'a` — lifetime parameter (declared alongside type parameters in the same `<...>`).
- `T` — generic type parameter.
- `where T: Display` — trait bound on `T`.

---

## 12.4 — Summary

| Feature | Syntax | Purpose |
|---------|--------|---------|
| **Generic function** | `fn f<T>(x: T)` | One definition, many types |
| **Generic struct/enum** | `struct S<T> { ... }` | Reusable data containers |
| **Trait** | `trait Foo { fn bar(&self); }` | Define shared behavior |
| **Implementing a trait** | `impl Foo for Type { ... }` | Opt a type into a behavior |
| **Trait bound** | `<T: Foo>` or `where T: Foo` | Constrain generics to types with capabilities |
| **`impl Trait` param** | `fn f(x: &impl Foo)` | Short trait bound for a single parameter |
| **`impl Trait` return** | `fn f() -> impl Foo` | Hide the concrete return type |
| **Lifetime parameter** | `<'a>` | Tie reference lifetimes together |
| **Static lifetime** | `'static` | Lives for the whole program |

```
┌──────────────────────────────────────────────────────────────────┐
│  The big picture:                                                 │
│                                                                   │
│  Generics    — "this works for many types"                        │
│  Traits      — "these types all do X"                             │
│  Bounds      — "I need a type that does X"                        │
│  Lifetimes   — "this reference is good at least as long as that"  │
│                                                                   │
│  All of it is checked at compile time and costs nothing at run.   │
│  This is the essence of Rust's zero-cost abstractions.            │
└──────────────────────────────────────────────────────────────────┘
```
