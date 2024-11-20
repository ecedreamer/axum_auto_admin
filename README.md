## Axum Auto Admin


### just derive your struct from DataModel and it will automatically have CRUD operation pages

```rust

#[derive(DataModel, Debug)]
struct Post {
    title: String,
    image: String,
    content: String,
}
```