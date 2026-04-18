use std::{collections::HashMap, sync::Arc};

use crate::Entity;

pub struct Scene {
  pub entities: HashMap<uuid::Uuid, Arc<dyn Entity>>,
}
