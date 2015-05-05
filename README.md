Iron Vhosts
====

> Vhost handler for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
fn main () {
    //Default handler passed to new
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));

    //Add any host specific handlers
    vhosts.add_host("localhost", localhost_handler);

    fn localhost_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "localhost")))
    }
}

```

## Installation

If you're using cargo, just add iron_vhosts to your `Cargo.toml`.

```toml
[dependencies]

iron_vhosts = "*"
```
