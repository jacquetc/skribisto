#[cxx_qt::bridge(namespace = "presenter::structure_management")]
pub mod qobject {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[cxx_qt::qobject]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type StructureManagementController = super::StructureManagementControllerRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        fn say_hi(self: &MyObject, string: &QString, number: i32);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

pub struct StructureManagementControllerRust {
    number: i32,
    string: QString,
}

impl Default for StructureManagementControllerRust {
    fn default() -> Self {
        Self {
            number: 0,
            string: QString::from(""),
        }
    }
}

impl qobject::StructureManagementController {
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.as_ref().number();
        self.set_number(previous + 1);
    }

    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is '{string}' and number is {number}");
    }
}
