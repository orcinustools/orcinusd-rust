extern crate orcinusd;

use orcinusd::utils;
use orcinusd::orcinus::Init;
//use orcinusd::http::server;

fn main() {
    let init = Init::sock("/var/run/docker.sock");
    let json_swarm = utils::to_json(&init.cluster().info());
    println!("{}",&json_swarm);

    //server("127.0.0.1:8000");
}
