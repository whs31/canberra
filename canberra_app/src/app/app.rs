use anyhow::Result;
use qmetaobject::prelude::*;

cpp::cpp! {{
  #include <qquickstyle.h>
}}

pub struct Application {
  qml_engine: QmlEngine,
}

impl Default for Application {
  fn default() -> Self {
    Self::new()
  }
}

impl Application {
  pub fn new() -> Self {
    cpp::cpp! {{
      QQuickStyle::setStyle("Material");
    }}
    Self {
      qml_engine: QmlEngine::new(),
    }
  }
}

impl crate::api::Application for Application {
  fn run(mut self) -> Result<()> {
    super::qml::load_qml();
    self
      .qml_engine
      .load_url(QString::from("qrc:/canberra/app/qml/Main.qml").into());
    self.qml_engine.exec();
    Ok(())
  }
}
