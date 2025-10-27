use anyhow::Result;
use qmetaobject::prelude::*;

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
    qmetaobject::qtquickcontrols2::QQuickStyle::set_style("FluentWinUI3");
    Self {
      qml_engine: QmlEngine::new(),
    }
  }

  fn register_qml_types(&self) {
    super::qml::load_qml();

    crate::core::qml::Module::new_versionless(c"com.whs31.canberra.qml.windows")
      .file(c"qrc:/canberra/app/qml/windows/About.qml", None);
  }
}

impl crate::api::Application for Application {
  fn run(mut self) -> Result<()> {
    self.register_qml_types();
    self
      .qml_engine
      .load_url(QString::from("qrc:/canberra/app/qml/Main.qml").into());
    self.qml_engine.exec();
    Ok(())
  }
}
