use iron::prelude::*;
use iron::status;
use router::Router;
use iron::mime::Mime;

use orcinus::Init;

static SOCK: &'static str = "/var/run/docker.sock";

pub fn server(host: &str) {
    let hostname = host;

    pub fn orc() -> Init {
        Init::sock(SOCK)
    }

    let mut router = Router::new();
    router.get("/ping",ping, "ping");

    router.get("/info",info, "info");

    router.get("/cluster",cluster, "cluster_info");

    Iron::new(router).http(hostname).unwrap();

    /* Ping */
    fn ping(_: &mut Request) -> IronResult<Response> {
        let ping = orc().ping().info();
        Ok(Response::with((status::Ok, "text/plain", ping)))
    }

    /* Docker info */
    fn info(_: &mut Request) -> IronResult<Response> {
        let api_info = orc().info().info();
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok, json_type, api_info)))
    }

    /* Cluster */
    fn cluster(_: &mut Request) -> IronResult<Response> {
      let data = orc().cluster().info();
      let json_type = "application/json".parse::<Mime>().unwrap();
      Ok(Response::with((status::Ok, json_type,data)))
    }

    // /* Nodes */
    // fn nodes_info(_: &mut Request) -> IronResult<Response> {
    //     let data = orc().nodes_info();
    //     let json_type = "application/json".parse::<Mime>().unwrap();
    //     Ok(Response::with((status::Ok, json_type, data)))
    // }
    //
    // /* Services */
    // fn services_info(_: &mut Request) -> IronResult<Response> {
    //     let data = orc().services_info();
    //     let json_type = "application/json".parse::<Mime>().unwrap();
    //     Ok(Response::with((status::Ok, json_type, data)))
    // }
    //
    // /* Tasks */
    // fn tasks_info(_: &mut Request) -> IronResult<Response> {
    //     let data = orc().tasks_info();
    //     let json_type = "application/json".parse::<Mime>().unwrap();
    //     Ok(Response::with((status::Ok, json_type, data)))
    // }
    //
    // /* Volumes */
    // fn volumes_info(_: &mut Request) -> IronResult<Response> {
    //     let data = orc().volumes_info();
    //     let json_type = "application/json".parse::<Mime>().unwrap();
    //     Ok(Response::with((status::Ok, json_type, data)))
    // }
    //
    // /* Events */
    // fn events_info(_: &mut Request) -> IronResult<Response> {
    //     let data = orc().events_info();
    //     let json_type = "application/json".parse::<Mime>().unwrap();
    //     Ok(Response::with((status::Ok, json_type, data)))
    // }
}
