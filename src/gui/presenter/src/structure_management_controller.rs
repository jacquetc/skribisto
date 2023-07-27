use lazy_static::lazy_static;

#[cxx_qt::bridge(
    cxx_file_stem = "structure_management_controller",
    namespace = "presenter::structure_management"
)]
pub mod qobject {

    #[namespace = ""]
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
        fn increment_number(self: Pin<&mut StructureManagementController>);

        #[qinvokable]
        fn say_hi(self: &StructureManagementController, string: &QString, number: i32);

        fn instance() -> Pin<Box<StructureManagementController>>;
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

    fn instance() -> Pin<Box<Self>> {
        lazy_static! {
            static ref INSTANCE: Pin<Box<StructureManagementControllerRust>> =
                Box::pin(StructureManagementControllerRust::default());
        }

        INSTANCE.clone()
    }
}
