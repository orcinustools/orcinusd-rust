use api::Binding as api;

pub struct Cluster {
  init: api,
}

impl Cluster {
  pub fn init(sock: api) -> Cluster {
    Cluster { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/swarm")
  }

  pub fn create(&self,data: &str) -> String {
    self.init.post("/swarm/init",data)
  }
}
