use crate::config::Config;
use crate::model::Model;

#[derive(Clone)]
pub struct Context {
  pub config: Config,
  pub model: Model,
}