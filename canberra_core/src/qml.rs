use cpp::cpp;
use qmetaobject::{PropertyType, QMetaObject, QObject};
use semver::Version;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

cpp! {{
  #include <qglobal.h>
  #include <qqml.h>
  #include <qquickitem.h>

  using CreatorFunction = void(*)(void*, void*);
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
    let uri_ptr = self.uri.as_ptr();
    let qml_name_ptr = qml_name.as_ptr();
    let meta_object = T::static_meta_object();
    let version_major = self.version.major as i32;
    let version_minor = self.version.minor as i32;

    extern "C" fn extra_destruct(c: *mut c_void) {
      cpp!(unsafe [c as "QObject *"] {
          QQmlPrivate::qdeclarativeelement_destructor(c);
      })
    }

    extern "C" fn creator_fn<T: QObject + Default + Sized>(c: *mut c_void, _: *mut c_void) {
      let b: Box<RefCell<T>> = Box::new(RefCell::new(T::default()));
      let ed: extern "C" fn(c: *mut c_void) = extra_destruct;
      unsafe {
        T::qml_construct(&b, c, ed);
      }
      Box::leak(b);
    }
    let creator_fn: extern "C" fn(c: *mut c_void, _: *mut c_void) = creator_fn::<T>;
    let size = T::cpp_size();
    let type_id = <RefCell<T> as PropertyType>::register_type(Default::default());

    cpp!(unsafe [
        qml_name_ptr as "char *",
        uri_ptr as "char *",
        version_major as "int",
        version_minor as "int",
        meta_object as "const QMetaObject *",
        creator_fn as "CreatorFunction",
        size as "size_t",
        type_id as "int"
    ] {
        int parserStatusCast = meta_object && meta_object->inherits(&QQuickItem::staticMetaObject)
            ? QQmlPrivate::StaticCastSelector<QQuickItem, QQmlParserStatus>::cast()
            : -1;

        QQmlPrivate::RegisterType api = {
            /*version*/ 0,
        #if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
            /*typeId*/ type_id,
        #else
            /*typeId*/ QMetaType(type_id),
        #endif
            /*listId*/ {},  // FIXME: list type?
            /*objectSize*/ int(size),
            /*create*/ creator_fn,
        #if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
            /* userdata */ nullptr,
        #endif
            /*noCreationReason*/ QString(),
        #if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
            /* createValueType */ nullptr,
        #endif
            /*uri*/ uri_ptr,
        #if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
            /*versionMajor*/ version_major,
            /*versionMinor*/ version_minor,
        #else
            /*version*/ QTypeRevision::fromVersion(version_major, version_minor),
        #endif
            /*elementName*/ qml_name_ptr,
            /*metaObject*/ meta_object,

            /*attachedPropertiesFunction*/ nullptr,
            /*attachedPropertiesMetaObject*/ nullptr,

            /*parserStatusCast*/ parserStatusCast,
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

  // pub fn uncreatable<T>(&self, name: Option<&CStr>)
  // where
  //   T: QRegisterable + Sized,
  // {
  //   let uri_ptr = self.uri.as_ptr();
  //   let version_major = self.version.major as i32;
  //   let version_minor = self.version.minor as i32;
  //   let name = name.unwrap_or_else(|| {
  //     static NAME: once_cell::sync::OnceCell<&'static CStr> = once_cell::sync::OnceCell::new();
  //     *NAME.get_or_init(|| {
  //       let full_type_name = std::any::type_name::<T>();
  //       let struct_name = full_type_name.rsplit("::").next().unwrap_or(full_type_name);
  //       Box::leak(CString::new(struct_name).unwrap().into_boxed_c_str())
  //     })
  //   });
  //   cpp! {unsafe [
  //     uri_ptr as "const char *",
  //     version_major as "int",
  //     version_minor as "int",
  //     name as "const char *"
  //   ] {
  //     qmlRegisterUncreatableType(
  //       uri_ptr,
  //       version_major,
  //       version_minor,
  //       name,
  //       "Access to enums & flags only"
  //     );
  //   }};
  // }

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

fn extract_type_name<T>() -> &'static CStr {
  static NAME: once_cell::sync::OnceCell<&'static CStr> = once_cell::sync::OnceCell::new();
  *NAME.get_or_init(|| {
    let full_type_name = std::any::type_name::<T>();
    let struct_name = full_type_name.rsplit("::").next().unwrap_or(full_type_name);
    Box::leak(CString::new(struct_name).unwrap().into_boxed_c_str())
  })
}

fn qml_register_uncreatable_type_qobject<T: QObject + Sized>(
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
