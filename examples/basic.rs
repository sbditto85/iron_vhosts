extern crate iron;
extern crate url;
extern crate iron_vhosts;

use iron::{Handler, IronResult, Request, Response, status};
use iron_vhosts::Vhosts;

fn main () {
    //Default handler passed to new
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));
    
    //Add any host specific handlers
    vhosts.add_host("localhost", localhost_handler);
    
    fn localhost_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "localhost")))
    }
}
