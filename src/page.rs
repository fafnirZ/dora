// this enum controls
// which "page" is rendered
// a page is not a Popup
// its a full page re-render
// which will remove all existing rendered
// states.
pub enum PageState {
    TablePage,
    MultiSheetSelectionPage,
    // FileExplorerPage, // TODO
}