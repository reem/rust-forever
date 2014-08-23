# Forever

> A never-dropping data store

Defines the `Forever` struct, which provides immutable access to data
that is Sync, Share, and is never dropped. You can think of it as an Arc
with an always positive refcount.

## Example:

```rust
fn main() {
    // unsafe because you can cause memory leaks if you are not careful
    let a = unsafe { Forever::new(7u) };
    let b = a.clone() // Same underlying data.

    spawn(proc() {
        println!("{}", *b); // 7
    });
}
```

