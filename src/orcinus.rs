extern crate rustc_serialize;

use api::Binding;

pub struct Init {
    sock: Binding,
}

impl Init {
    pub fn sock(socket: &'static str) -> Init {
        let binding = Binding::new(socket);
        Init { sock: binding }
    }

    /* Ping */
    pub fn ping(&self) -> String {
        self.sock.get("/_ping")
    }

    /* Docker info */
    pub fn info(&self) -> String {
        self.sock.get("/info")
    }

    /* Cluster */
    pub fn cluster_info(&self) -> String {
        self.sock.get("/swarm")
    }

    /* Nodes */
    pub fn nodes_info(&self) -> String {
        self.sock.get("/nodes")
    }

    /* Services */
    pub fn services_info(&self) -> String {
        self.sock.get("/services")
    }

    /* Tasks */
    pub fn tasks_info(&self) -> String {
        self.sock.get("/tasks")
    }

    /* Volumes */
    pub fn volumes_info(&self) -> String {
        self.sock.get("/volumes")
    }

    /* Events */
    pub fn events_info(&self) -> String {
        self.sock.get("/events")
    }
}
