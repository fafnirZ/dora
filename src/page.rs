// this enum controls
// which "page" is rendered
// a page is not a Popup
// its a full page re-render
// which will remove all existing rendered
// states.
use crate::io::FileType;

pub enum PageState {
    TablePage,
    MultiSheetSelectionPage,
    // FileExplorerPage, // TODO
}


impl PageState {

    // helper.
    // if the file extension is any of the excel ones.
    // check how many sheets there are
    // if > 1, it should be set to multisheet selection page
    pub fn determine_is_multisheet_selection_page(
        file_path: &str,
    ) -> bool {
        match FileType::determine_extension(file_path).unwrap() {
            FileType::Excel => {
                // NOTE this means any excel file 
                // even if they have only 1 tab
                // will render into the tab selector
                // which imean fair enough?
                true
            }
            _ => false
        }
    }
}