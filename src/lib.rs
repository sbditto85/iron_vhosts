extern crate iron;

use std::collections::HashMap;
use iron::{Handler, IronResult, Request, Response};

pub struct Vhosts {
    default: Box<Handler>,
    vhosts: HashMap<&'static str, Box<Handler>>,
}

impl Vhosts {

    pub fn new<H: Handler>(h: H) -> Vhosts {
        Vhosts {
            default: Box::new(h) as Box<Handler>,
            vhosts: HashMap::new()
        }
    }

    /// For adding a handler for a host
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate iron;
    /// extern crate iron_vhosts;
    ///
    /// use iron::prelude::*;
    /// use iron::status;
    /// use iron_vhosts::Vhosts;
    ///
    /// fn main () {
    ///     //Default handler passed to new
    ///     let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));
    ///
    ///     //Add any host specific handlers
    ///     vhosts.add_host("localhost", localhost_handler);
    ///     vhosts.add_host("media.localhost", media_handler);
    ///   
    ///     fn localhost_handler(_: &mut Request) -> IronResult<Response> {
    ///         Ok(Response::with((status::Ok, "localhost")))
    ///     }
    ///
    ///     fn media_handler(_: &mut Request) -> IronResult<Response> {
    ///         Ok(Response::with((status::Ok, "media")))
    ///     }
    ///
    ///     Iron::new(vhosts).http("localhost:3000").unwrap();
    /// }
    /// ```
    pub fn add_host<H: Handler>(&mut self, host: &'static str, h: H) -> Option<Box<Handler>> {
        self.vhosts.insert(host, Box::new(h))
    }

}

impl Handler for Vhosts {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        //get the host from the request
        let host = {
            let host = req.url.host();
            format!("{}", host)
        };

        //get the handler associated to the host
        let handler = match self.vhosts.get(host.as_str()){
            Some(box_handler) => box_handler,
            None              => &self.default
        };
        
        //fire off the handler
        handler.handle(req)
    }
}


#[test]
fn it_works() {
}
