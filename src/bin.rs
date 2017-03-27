extern crate orcinusd;

//use orcinusd::orcinus::Init;
use orcinusd::utils;
use orcinusd::http::server;
use orcinusd::orcinus::Init;

fn main() {
    let init = Init::sock("/var/run/docker.sock");
    let json_swarm = utils::to_json(&init.cluster().info());
    println!("{}",&json_swarm);
    //web();
    server("127.0.0.1:8000");
}
