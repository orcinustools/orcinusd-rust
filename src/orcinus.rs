extern crate rustc_serialize;

use api::Binding as api;
use libs::cluster::Cluster;
use libs::ping::Ping;
use libs::info::Info;

pub struct Init {
    sock : api,
}

impl Init {
    pub fn sock(socket: &'static str) -> Init {
        let binding = api::new(socket);
        Init { sock : binding }
    }

    pub fn cluster(self) -> Cluster {
        Cluster::init(self.sock)
    }

    pub fn ping(self) -> Ping {
        Ping::init(self.sock)
    }

    pub fn info(self) -> Info {
        Info::init(self.sock)
    }
    //
    // /* Nodes */
    // pub fn nodes_info(&self) -> String {
    //     self.sock.get("/nodes")
    // }
    //
    // /* Services */
    // pub fn services_info(&self) -> String {
    //     self.sock.get("/services")
    // }
    //
    // /* Tasks */
    // pub fn tasks_info(&self) -> String {
    //     self.sock.get("/tasks")
    // }
    //
    // /* Volumes */
    // pub fn volumes_info(&self) -> String {
    //     self.sock.get("/volumes")
    // }
    //
    // /* Events */
    // pub fn events_info(&self) -> String {
    //     self.sock.get("/events")
    // }
}
