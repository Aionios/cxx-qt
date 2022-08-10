#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");
        include!("cxx-qt-lib/include/convert.h");
        include ! (< QtCore / QObject >);

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[cxx_name = "unsafe_rust"]
        fn rust(self: &MyObjectQt) -> &RustObj;
        #[rust_name = "new_cpp_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafe_rust_mut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut RustObj>;
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type RustObj;

        #[cxx_name = "invokable"]
        fn invokable(self: &RustObj);
        #[cxx_name = "invokableCppObjWrapper"]
        fn invokable_cpp_obj_wrapper(self: &RustObj, cpp: Pin<&mut MyObjectQt>);
        #[cxx_name = "invokableMutable"]
        fn invokable_mutable(self: &mut RustObj);
        #[cxx_name = "invokableMutableCppObjWrapper"]
        fn invokable_mutable_cpp_obj_wrapper(self: &mut RustObj, cpp: Pin<&mut MyObjectQt>);
        #[cxx_name = "invokableParameters"]
        fn invokable_parameters(self: &RustObj, opaque: &QColor, trivial: &QPoint, primitive: i32);
        #[cxx_name = "invokableParametersCppObjWrapper"]
        fn invokable_parameters_cpp_obj_wrapper(
            self: &RustObj,
            primitive: i32,
            cpp: Pin<&mut MyObjectQt>,
        );
        #[cxx_name = "invokableReturnOpaqueWrapper"]
        fn invokable_return_opaque_wrapper(self: &mut RustObj) -> UniquePtr<QColor>;
        #[cxx_name = "invokableReturnPrimitive"]
        fn invokable_return_primitive(self: &mut RustObj) -> i32;
        #[cxx_name = "invokableReturnStaticWrapper"]
        fn invokable_return_static_wrapper(self: &mut RustObj) -> UniquePtr<QString>;

        #[cxx_name = "createRs"]
        fn create_rs() -> Box<RustObj>;

        #[cxx_name = "initialiseCpp"]
        fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
        type QPoint = cxx_qt_lib::QPoint;
        type QString = cxx_qt_lib::QString;
    }
}

pub use self::cxx_qt_my_object::*;
mod cxx_qt_my_object {
    use super::my_object::*;

    pub type FFICppObj = super::my_object::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        pub fn invokable_cpp_obj_wrapper(&self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_cpp_obj(&mut cpp);
        }

        pub fn invokable_mutable_cpp_obj_wrapper(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_mutable_cpp_obj(&mut cpp);
        }

        pub fn invokable_parameters_cpp_obj_wrapper(
            &self,
            primitive: i32,
            cpp: std::pin::Pin<&mut FFICppObj>,
        ) {
            let mut cpp = CppObj::new(cpp);
            self.invokable_parameters_cpp_obj(primitive, &mut cpp);
        }

        pub fn invokable_return_opaque_wrapper(&mut self) -> UniquePtr<cxx_qt_lib::QColor> {
            return self.invokable_return_opaque();
        }

        pub fn invokable_return_static_wrapper(&mut self) -> UniquePtr<cxx_qt_lib::QString> {
            return self.invokable_return_static();
        }

        pub fn invokable(&self) {
            println!("invokable");
        }

        pub fn invokable_cpp_obj(&self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        pub fn invokable_mutable(&mut self) {
            println!("This method is mutable!");
        }

        pub fn invokable_mutable_cpp_obj(&mut self, cpp: &mut CppObj) {
            println!("This method is mutable!");
        }

        pub fn invokable_parameters(&self, opaque: &QColor, trivial: &QPoint, primitive: i32) {
            println!(
                "Red: {}, Point X: {}, Number: {}",
                opaque.red(),
                trivial.x(),
                primitive,
            );
        }

        pub fn invokable_parameters_cpp_obj(&self, primitive: i32, cpp: &mut CppObj) {
            println!("{}", primitive);
        }

        pub fn invokable_return_opaque(&mut self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }

        pub fn invokable_return_primitive(&mut self) -> i32 {
            2
        }

        pub fn invokable_return_static(&mut self) -> UniquePtr<QString> {
            QString::from_str("static")
        }

        pub fn cpp_context_method(&self) {
            println!("C++ context method");
        }

        pub fn cpp_context_method_mutable(&mut self) {
            println!("mutable method");
        }

        pub fn cpp_context_method_cpp_obj(&mut self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        pub fn cpp_context_method_return_opaque(&self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn grab_values_from_data(&mut self, mut data: Data) {}
    }

    pub struct Data;

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(_value: &CppObj<'a>) -> Self {
            Self {}
        }
    }

    impl<'a> From<&mut CppObj<'a>> for Data {
        fn from(_value: &mut CppObj<'a>) -> Self {
            Self::from(&*_value)
        }
    }

    impl RustObj {
        pub fn rust_only_method(&self) {
            println!("QML or C++ can't call this :)");
        }
    }

    pub fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(Data::default());
    }
}
