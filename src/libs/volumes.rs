use api::Binding as api;

pub struct Volumes {
  init: api,
}

impl Volumes {
  pub fn init(sock: api) -> Volumes {
    Volumes { init : sock }
  }

  pub fn info(&self) -> String {
    self.init.get("/volumes")
  }
}
