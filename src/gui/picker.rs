use cstr::cstr;
use qmetaobject::*;
use rfd::FileDialog;
use std::process::Command;

#[derive(QObject, Default)]
struct FilePickerApp {
    base: qt_base_class!(trait QObject),

    open_file_and_run: qt_method!(fn(&self)),
}

impl FilePickerApp {
    fn open_file_and_run(&self) {
        let file = FileDialog::new()
            .set_title("Select a .oiff image")
            .add_filter("OIFF Image (*.oiff)", &["oiff"])
            .pick_file();

        if let Some(path) = file {
            println!("Selected file: {path:?}");
            let status = Command::new("oiff").arg("display").arg(&path).status();

            match status {
                Ok(s) => println!("Command exited with status: {}", s),
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }
        }
    }
}

pub fn open_oiff() {
    qml_register_type::<FilePickerApp>(cstr!("RustApp"), 1, 0, cstr!("FilePickerApp"));

    let mut engine = QmlEngine::new();

    engine.load_data(
        r#"
        import QtQuick 2.15
        import QtQuick.Controls 2.15
        import QtQuick.Layouts 1.15
        import RustApp 1.0

        ApplicationWindow {
            visible: true
            width: 320
            height: 120
            title: "Select a .oiff image"

            FilePickerApp {
                id: pickerApp
            }

            Button {
                anchors.centerIn: parent
                text: "Open Image"
                onClicked: pickerApp.open_file_and_run()
            }
        }
        "#
        .into(),
    );

    engine.exec();
}
