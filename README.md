# Forever

> A never-dropping data store

Defines the `Forever` struct, which provides immutable access to data
that is Sync, Send, and is never dropped. You can think of it as an Arc
with an always positive refcount.

## Example:

```rust
fn main() {
    let a = Forever::new(7u); // 7u will never be dropped.
    let b = a.clone() // Same underlying data.

    spawn(proc() {
        println!("{}", *b); // 7
    });
}
```

