use iron::prelude::*;
use iron::status;
use router::Router;
use iron::mime::Mime;

use orcinus::Init;


// pub fn web(host: &str,port: &str){
pub fn web(){
    let hostname = "localhost:3000";
    // if host {
    //     hostname = format!("{}:{}",host,port);
    // }
    let mut router = Router::new();
    router.get("/info", info,"info");
    router.get("/cluster", cluster_info,"cluster_info");
    router.get("/services", services_info,"services_info");
    router.get("/nodes", nodes_info,"nodes_info");
    router.get("/tasks", tasks_info,"tasks_info");
    Iron::new(router).http(hostname).unwrap();
    pub fn orc() -> Init{
        Init::sock("/var/run/docker.sock")
    }
    /* DOCKER INFO */
    fn info(_: &mut Request) -> IronResult<Response> {
        let api_info = orc().info();
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok,json_type, api_info)))
    }

    /* Cluster */
    fn cluster_info(_: &mut Request) -> IronResult<Response> {
        let data = orc().cluster_info();
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok,json_type, data)))
    }

    /* Nodes */
    fn nodes_info(_: &mut Request) -> IronResult<Response> {
        let data = orc().nodes_info();
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok,json_type, data)))
    }

    /* Services */
    fn services_info(_: &mut Request) -> IronResult<Response> {
        let data = orc().services_info();
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok,json_type, data)))
    }

    /* Tasks */
    fn tasks_info(_: &mut Request) -> IronResult<Response> {
        let data = orc().tasks_info();
        let json_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok,json_type, data)))
    }
}
