extern crate orcinusd;

//use orcinusd::orcinus::Init;
// use orcinusd::utils;
use orcinusd::http::web;

fn main() {
    // let init = Init::sock("/var/run/docker.sock");
    // let json_swarm = utils::to_json(&init.info());
    // println!("{}",&json_swarm);

    web();

}
