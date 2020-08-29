# Notes

- Ownership Rules
    - Variable is valid only inside a block like other languages
- References and Borrowing
    - References by default immutable
    - Mutable references cannot be borrowed more than once
    - Once immutable reference to a pointer is used cannot be made mutable later.
      But at the same time when usage of immutable reference ends, mutable reference
      can be created.
- The Slice Type
    ```rust
    let a = "hello world"; or `let a = String::from("hello world");`
    ```
- Defining and Instantiating Structs
- Method Syntax
