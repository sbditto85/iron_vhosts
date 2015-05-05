extern crate iron;
extern crate url;

use std::collections::HashMap;
use iron::{Handler, IronResult, Request, Response};
use url::Host::{Domain, Ipv6};

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
    /// ```
    /// extern crate iron;
    /// extern crate url;
    /// extern crate iron_vhosts;
    /// 
    /// use iron::{Handler, IronResult, Request, Response, status};
    /// use iron_vhosts::Vhosts;
    /// 
    /// fn main () {
    ///     //Default handler passed to new
    ///     let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));
    ///     
    ///     //Add any host specific handlers
    ///     vhosts.add_host("localhost", localhost_handler);
    ///
    ///     fn localhost_handler(_: &mut Request) -> IronResult<Response> {
    ///         Ok(Response::with((status::Ok, "localhost")))
    ///     }
    /// }
    /// ```
    pub fn add_host<H: Handler>(&mut self, host: &'static str, h: H) -> Option<Box<Handler>> {
        self.vhosts.insert(host, Box::new(h))
    }

}

impl Handler for Vhosts {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        //get the host from the request
        let host = match req.url.host {
            Domain(ref h) => h.clone(),
            Ipv6(ref h)   => format!("{}", h)
        };

        //get the handler associated to the host
        let handler = match self.vhosts.get(&*host){
            Some(box_handler) => &*box_handler,
            None              => &self.default
        };
        
        //fire off the handler
        handler.handle(req)
    }
}


#[test]
fn it_works() {
}
