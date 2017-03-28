use api::Binding as api;

pub struct Tasks {
  init: api,
}

impl Tasks {
  pub fn init(sock: api) -> Tasks {
    Tasks { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/tasks")
  }
}
