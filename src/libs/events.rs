use api::Binding as api;

pub struct Events {
  init: api,
}

impl Events {
  pub fn init(sock: api) -> Events {
    Events { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/events")
  }
}
