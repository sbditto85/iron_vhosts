Iron Vhosts
====

> Vhost handler for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate iron;
extern crate iron_vhosts;

use iron::prelude::*;
use iron::status;
use iron_vhosts::Vhosts;

fn main () {
    //Default handler passed to new
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));
    
    //Add any host specific handlers
    vhosts.add_host("localhost", localhost_handler);
    vhosts.add_host("media.localhost", media_handler);
    
    fn localhost_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "localhost")))
    }

    fn media_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "media")))
    }

    Iron::new(vhosts).http("localhost:3000").unwrap();
}
```

## Installation

If you're using cargo, just add iron_vhosts to your `Cargo.toml`.

```toml
[dependencies]

iron_vhosts = "*"
```
