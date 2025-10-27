use cpp::cpp;
use qmetaobject::{PropertyType, QObject, qml_register_module, qml_register_type};
use semver::Version;
use std::cell::RefCell;
use std::ffi::{CStr, CString};

cpp! {{
  #include <qglobal.h>
  #include <qqml.h>
  #include <qquickitem.h>
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

  pub fn component<T>(&self, qml_name: Option<&CStr>)
  where
    T: qmetaobject::QObject + Default + Sized,
  {
    let qml_name = qml_name.unwrap_or_else(|| extract_type_name::<T>());

    log::trace!(
      target: "qml_type_registration",
      "[{}] registered qml component {}", self.uri.clone().into_string().unwrap_or_default(), qml_name.to_str().unwrap_or_default()
    );

    qml_register_type::<T>(
      &self.uri,
      self.version.major as u32,
      self.version.minor as u32,
      qml_name,
    );
  }

  pub fn file(&self, url: &CStr, qml_name: Option<&CStr>) {
    let qml_name = qml_name.unwrap_or_else(|| extract_file_name(url));

    log::trace!(
      target: "qml_type_registration",
      "[{}] registered qml file '{}' ({})",
      self.uri.clone().into_string().unwrap_or_default(),
      qml_name.to_str().unwrap_or_default(),
      url.to_str().unwrap_or_default()
    );

    let uri_ptr = self.uri.as_ptr();
    let url_ptr = url.as_ptr();
    let version_major = self.version.major as i32;
    let version_minor = self.version.minor as i32;
    let qml_name_ptr = qml_name.as_ptr();
    cpp! {unsafe [
      uri_ptr as "const char *",
      version_major as "int",
      version_minor as "int",
      url_ptr as "const char *",
      qml_name_ptr as "const char *"
    ] {
      qmlRegisterType(
        QUrl(url_ptr),
        uri_ptr,
        version_major,
        version_minor,
        qml_name_ptr
      );
    }};
  }

  pub fn uncreatable<T>(&self, name: Option<&CStr>)
  where
    T: QObject + Sized,
  {
    let name = name.unwrap_or_else(|| extract_type_name::<T>());
    log::trace!(
      target: "qml_type_registration",
      "[{}] registered qml uncreatable type {}", self.uri.clone().into_string().unwrap_or_default(), name.to_str().unwrap_or_default()
    );
    let version_major = self.version.major as i32;
    let version_minor = self.version.minor as i32;
    qml_register_uncreatable_type::<T>(
      &self.uri,
      version_major as u32,
      version_minor as u32,
      name,
      qmetaobject::QString::from("Access to enums & flags only"),
    );
  }

  fn register_module(&self) {
    log::trace!(
      target: "qml_type_registration",
      "registered qml module {} v{}", self.uri.clone().into_string().unwrap_or_default(), self.version
    );
    qml_register_module(
      &self.uri,
      self.version.major as u32,
      self.version.minor as u32,
    );
  }
}

fn extract_type_name<T>() -> &'static CStr {
  static NAME: once_cell::sync::OnceCell<&'static CStr> = once_cell::sync::OnceCell::new();
  NAME.get_or_init(|| {
    let full_type_name = std::any::type_name::<T>();
    let struct_name = full_type_name.rsplit("::").next().unwrap_or(full_type_name);
    Box::leak(
      CString::new(struct_name)
        .expect("Type name contains null byte")
        .into_boxed_c_str(),
    )
  })
}

fn extract_file_name(path: &CStr) -> &'static CStr {
  static NAME: once_cell::sync::OnceCell<&'static CStr> = once_cell::sync::OnceCell::new();
  NAME.get_or_init(|| {
    let path_str = path.to_str().expect("Path contains invalid UTF-8");

    let file_name = path_str
      .rsplit('/')
      .next()
      .unwrap_or(path_str)
      .split('.')
      .next()
      .unwrap_or(path_str);

    Box::leak(
      CString::new(file_name)
        .expect("Filename contains null byte")
        .into_boxed_c_str(),
    )
  })
}

fn qml_register_uncreatable_type<T: QObject + Sized>(
  uri: &CStr,
  version_major: u32,
  version_minor: u32,
  qml_name: &CStr,
  reason: qmetaobject::QString,
) {
  let uri_ptr = uri.as_ptr();
  let qml_name_ptr = qml_name.as_ptr();
  let meta_object = T::static_meta_object();

  let size = T::cpp_size();

  let type_id = <RefCell<T> as PropertyType>::register_type(Default::default());

  cpp!(unsafe [
      qml_name_ptr as "char *",
      uri_ptr as "char *",
      version_major as "int",
      version_minor as "int",
      meta_object as "const QMetaObject *",
      size as "size_t",
      type_id as "int",
      reason as "QString"
  ] {
      QQmlPrivate::RegisterType api = {
          /*version*/ 0,

      #if QT_VERSION < QT_VERSION_CHECK(6,0,0)
          /*typeId*/ type_id,
      #else
          /*typeId*/ QMetaType(type_id),
      #endif
          /*listId*/ {},  // FIXME: list type?
          /*objectSize*/ int(size),
          /*create*/ nullptr,
      #if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
          /* userdata */ nullptr,
      #endif
          /*noCreationReason*/ reason,
      #if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
          /* createValueType */ nullptr,
      #endif

          /*uri*/ uri_ptr,
      #if QT_VERSION < QT_VERSION_CHECK(6,0,0)
          /*versionMajor*/ version_major,
          /*versionMinor*/ version_minor,
      #else
          /*version*/ QTypeRevision::fromVersion(version_major, version_minor),
      #endif
          /*elementName*/ qml_name_ptr,
          /*metaObject*/ meta_object,

          /*attachedPropertiesFunction*/ nullptr,
          /*attachedPropertiesMetaObject*/ nullptr,

          /*parserStatusCast*/ -1,
          /*valueSourceCast*/ -1,
          /*valueInterceptorCast*/ -1,

          /*extensionObjectCreate*/ nullptr,
          /*extensionMetaObject*/ nullptr,
          /*customParser*/ nullptr,
          /*revision*/ {}  // FIXME: support revisions?
      };
      QQmlPrivate::qmlregister(QQmlPrivate::TypeRegistration, &api);
  })
}
