use api::Binding as api;

pub struct Nodes {
  init: api,
}

impl Nodes {
  pub fn init(sock: api) -> Nodes {
    Nodes { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/nodes")
  }
}
