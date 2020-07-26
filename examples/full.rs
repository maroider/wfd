extern crate wfd;

use wfd::{DialogParams, FOS_ALLOWMULTISELECT, DialogError};

fn main() {
    let params = DialogParams {
        file_types: vec![("DLL Files", "*.dll"), ("Executable Files", "*.exe;*.com;*.scr")],
        default_extension: "dll",
        default_folder: r"C:\Windows\System32".as_ref(),
        file_name: "win32k.sys",
        file_name_label: "Select some files!",
        file_type_index: 1,
        ok_button_label: "Custom OK",
        options: FOS_ALLOWMULTISELECT,
        title: "Test open file dialog",
        .. Default::default()
    };

    match wfd::open_dialog(params) {
        Ok(r) => {
            for file in r.selected_file_paths {
                println!("{}", file.to_str().unwrap());
            }
        }
        Err(e) => match e {
            DialogError::UserCancelled => {
                println!("User cancelled dialog");
            }
            DialogError::HResultFailed { hresult, error_method } => {
                println!("HResult Failed - HRESULT: {:X}, Method: {}", hresult, error_method);
            }
        },
    }
}