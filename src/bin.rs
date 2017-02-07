extern crate orcinusd;
extern crate rustc_serialize;
// use rustc_serialize::json::Json;
//
// use orcinusd::api::Binding;
use orcinusd::http::initialize;

fn main() {
    // let endpoint = "/var/run/docker.sock";
    // let client = Binding::new(endpoint);
    //
    // let info = client.get("/info");
    // let json_info = Json::from_str(&info).unwrap();
    // println!("{}",&json_info);
    //
    // let swarm = client.get("/swarm");
    // let json_swarm = Json::from_str(&swarm).unwrap();
    // println!("{}",&json_swarm);

    initialize();
}
