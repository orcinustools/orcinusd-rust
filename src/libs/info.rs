use api::Binding as api;

pub struct Info {
  init: api,
}

impl Info {
  pub fn init(sock: api) -> Info {
    Info { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/info")
  }
}
