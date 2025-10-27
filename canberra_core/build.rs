fn main() {
  cargo_build::rerun_if_changed(["src/lib.rs", "src/qml.rs"]);
  
  let qt_include_path = std::env::var("DEP_QT_INCLUDE_PATH").unwrap();
  let qt_library_path = std::env::var("DEP_QT_LIBRARY_PATH").unwrap();
  let qt_version = std::env::var("DEP_QT_VERSION")
    .unwrap()
    .parse::<semver::Version>()
    .expect("Parsing Qt version failed");
  // let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

  let mut config = cpp_build::Config::new();

  for f in std::env::var("DEP_QT_COMPILE_FLAGS")
    .unwrap()
    .split_terminator(';')
  {
    config.flag(f);
  }

  let mut public_include = |name| {
    if cfg!(target_os = "macos") {
      config.include(format!("{}/{}.framework/Headers/", qt_library_path, name));
    }
    config.include(format!("{}/{}", qt_include_path, name));
  };
  public_include("QtCore");
  public_include("QtGui");
  public_include("QtQuick");
  public_include("QtQml");
  public_include("QtQuickControls2");

  let mut private_include = |name| {
    if cfg!(target_os = "macos") {
      config.include(format!(
        "{}/{}.framework/Headers/{}",
        qt_library_path, name, qt_version
      ));
      config.include(format!(
        "{}/{}.framework/Headers/{}/{}",
        qt_library_path, name, qt_version, name
      ));
    }
    config
      .include(format!("{}/{}/{}", qt_include_path, name, qt_version))
      .include(format!(
        "{}/{}/{}/{}",
        qt_include_path, name, qt_version, name
      ));
  };
  private_include("QtCore");
  private_include("QtGui");
  private_include("QtQuick");
  private_include("QtQml");

  config.include(qt_include_path).build("src/lib.rs");
}
