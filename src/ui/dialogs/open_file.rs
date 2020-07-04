use gtk::prelude::*;
use gtk::{FileChooserAction, FileChooserDialog, ResponseType, Window, WindowPosition};
use std::path::PathBuf;

pub fn open_file(parent_window: &Window) -> Option<PathBuf> {
    let dlg = FileChooserDialog::new(
        Some("Open file"),
        Some(parent_window),
        FileChooserAction::Open,
    );
    dlg.set_icon_name(Some("password-storage"));
    dlg.set_property_window_position(WindowPosition::CenterOnParent);
    dlg.set_transient_for(Some(parent_window));

    dlg.add_button("_Cancel", ResponseType::Cancel);
    dlg.add_button("_Open", ResponseType::Ok);
    dlg.set_default_response(ResponseType::Ok);
    let open_clicked = dlg.run() == ResponseType::Ok;
    let filename = dlg.get_filename();
    dlg.close();

    if open_clicked {
        filename
    } else {
        None
    }
}
