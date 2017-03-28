extern crate rustc_serialize;

use api::Binding as api;
use libs::cluster::Cluster;
use libs::ping::Ping;
use libs::info::Info;
use libs::nodes::Nodes;
use libs::events::Events;
use libs::services::Services;
use libs::tasks::Tasks;
use libs::volumes::Volumes;

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

    pub fn nodes(self) -> Nodes {
      Nodes::init(self.sock)
    }

    pub fn events(self) -> Events {
      Events::init(self.sock)
    }

    pub fn services(self) -> Services {
      Services::init(self.sock)
    }

    pub fn tasks(self) ->Tasks {
      Tasks::init(self.sock)
    }

    pub fn volumes(self) -> Volumes {
      Volumes::init(self.sock)
    }

}
