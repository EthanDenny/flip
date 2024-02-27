# Flip

A language inspired by functional programming and Lisp.

Execute code with `cargo run [path]`

## What is this?

Flip is a lazy evaluated, strongly typed, functional programming language. It's syntax is similar to Rust, and it's underlying structure is similar to Lisp.

Although it's still in a pre-alpha stage (and may never progress past that point), it can be used for some small tasks. The `test/` directory contains some examples, which can solve the first and seond problems from https://projecteuler.net/, as well as calculate the sum of the first 200 primes.

## How does it work?

This prints "43110":

```
main() {
    43110
}
```

This does the same:

```
hello(): Int {
    43110
}

main() {
    hello()
}
```

Now, a more complex example, which sums the numbers from 1 to 100:

```
sum(nums: [Int]): Int {
    if(is_null(nums),
        0,
        +(head(nums), sum(tail(nums)))
    )
}

range(nums: [Int], start: Int, end: Int): [Int] {
    if(<=(start, end),
        range(push(nums, start), +(start, 1), end),
        nums
    )
}

main() {
    sum(range([Int](), 1, 100))
}

```

- `[Int]()` constructs a new list of integers.
- `push([Int], Int): [Int]` adds a new item to a list.
- `is_null([Int]): Bool` tests if a list is empty
- `head([Int]): Int` returns the first item in a list
- `tail([Int]): [Int]` returns all the items after the first
- `+` and `<=` are the addition and less than or equal
- `if(Bool, T, T): T` returns the second argument if the first is true and the third argument if the first is false. The second and third arguments can be any tybe, but they must be the same.

There are a few other build-in functions not covered by this example:

- `len([Int]): Int` returns the length of a list
- All basic arithmetic operators
- `mod` for modular division
- All basic comparison operators
- `and`, `or`, and `not`
