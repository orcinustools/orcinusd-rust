use api::Binding as api;

pub struct Ping {
  init: api,
}

impl Ping {
  pub fn init(sock: api) -> Ping {
    Ping { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/_ping")
  }
}
