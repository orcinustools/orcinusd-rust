use api::Binding as api;

pub struct Services {
  init: api,
}

impl Services {
  pub fn init(sock: api) -> Services {
    Services { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/services")
  }
}
