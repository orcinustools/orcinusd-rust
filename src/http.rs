use iron::prelude::*;
use iron::status;
use router::Router;
use iron::mime::Mime;

use api::Binding;

pub fn initialize(){

    fn bind() -> Binding {
        let endpoint = "/var/run/docker.sock";
        Binding::new(endpoint)
    }

    let mut router = Router::new();
    router.get("/info", info,"info");
    router.get("/cluster", cluster,"cluster");

    fn info(_: &mut Request) -> IronResult<Response> {
        let api_info = bind().get("/info");
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok,json_type, api_info)))
    }

    fn cluster(_: &mut Request) -> IronResult<Response> {
        let data = bind().get("/swarm");
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok,json_type, data)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
