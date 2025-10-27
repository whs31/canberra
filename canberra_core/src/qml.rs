use cpp::cpp;
use qmetaobject::prelude::*;
use semver::Version;
use std::ffi::{CStr, CString};

cpp! {{
  #include <qglobal.h>
  #include <qqml.h>
}}

#[derive(Debug)]
pub struct Module {
  pub uri: CString,
  pub version: Version,
}

impl Module {
  pub fn new(uri: &CStr, version: Version) -> Self {
    let this = Self {
      uri: uri.to_owned(),
      version,
    };
    this.register_module();
    this
  }

  pub fn new_versionless(uri: &CStr) -> Self {
    Self::new(uri, Version::new(1, 0, 0))
  }

  pub fn component<T>(&self, name: Option<&CStr>)
  where
    T: QObject + Default + Sized,
  {
  }

  fn register_module(&self) {
    let uri_ptr = self.uri.as_ptr();
    let version_major = self.version.major as i32;
    let version_minor = self.version.minor as i32;
    
    cpp! {unsafe [
      uri_ptr as "const char *",
      version_major as "int",
      version_minor as "int"
    ] {
    #if QT_VERSION >= QT_VERSION_CHECK(5,9,0)
      qmlRegisterModule(
        uri_ptr,
        version_major,
        version_minor
      );
    #endif
    }};
  }
}
