extern crate iron;
extern crate iron_vhosts;
extern crate router;


use iron::prelude::*;
use iron::status;
use iron_vhosts::Vhosts;
use router::Router;

fn main () {
    //Default handler passed to new
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));
    
    let mut router = Router::new();
    router.get("/", localhost_handler);
    router.get("/:query", localhost_handler);

    //Add any host specific handlers
    vhosts.add_host("localhost", router);
    vhosts.add_host("media.localhost", media_handler);
    
    fn localhost_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    fn media_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "media")))
    }

    Iron::new(vhosts).http("localhost:3000").unwrap();
}
