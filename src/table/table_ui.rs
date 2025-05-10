use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};

use crate::app::App;
use crate::utils::cell::get_header_area;
use crate::utils::centered_text::render_text_centered_in_area;

use super::column_ui::ColumnUI;
use super::line_number_ui::LineNumberUI;
use super::table_banner::TableBanner;

pub struct TableUI {}

impl TableUI {
    pub fn new() -> Self {
        Self {}
    }
}

// priv
impl TableUI {
    fn render_table_borders(&self, area: Rect, buf: &mut Buffer) {
        // render a block for table.
        // give it a top & bottom border;
        let block = Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(64, 64, 64)));

        block.render(area, buf);
    }

    fn render_header(
        &self,
        area: Rect,
        buf: &mut Buffer,
        app_state: &mut <TableUI as StatefulWidget>::State,
    ) {
        let config_state = &app_state.config_state;
        let start_x = area.x;
        let start_y = area.y;

        // rendering the block
        let block = Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(64, 64, 64)));
        let height = config_state.header_height;
        let area = Rect::new(start_x, start_y, area.width, height);
        block.render(area, buf);

        // rendering the column values.
        let df_state = &app_state.dataframe_state;
        let headers = df_state.get_headers_in_col_slice();
        for (idx, header) in headers.iter().enumerate() {
            let y = start_y;
            let x = start_x + config_state.cell_width * (idx as u16);
            if x + config_state.cell_width > area.x + area.width {
                break;
            } // do not render beyond bounds
            let cell_area = get_header_area(config_state, x, y);
            let header_name = header.name.clone();
            render_text_centered_in_area(header_name, cell_area, buf);
        }

        // y pos of header text and next line
        // (height.saturating_sub(2), height)
    }

    fn get_column_uis_for_rendering(
        &self,
        area: Rect,
        state: &mut <TableUI as StatefulWidget>::State,
    ) -> Vec<ColumnUI> {
        let config_state = &state.config_state;
        // debug_render_area_bg(area, buf, Color::Cyan);

        // respect the area assigned to the widget.
        let start_x = area.x;
        let end_x = start_x + area.width;
        let start_y = area.y;

        let df_state = &state.dataframe_state;
        // columns
        let columns = df_state.get_columns_in_col_slice();
        let mut column_uis = Vec::new();
        for (idx, column) in columns.iter().enumerate() {
            let x_offset = start_x + config_state.cell_width * (idx as u16);
            let y_offset = start_y + config_state.cell_height * 1; // header
            // do not render beyond bounds
            if x_offset + config_state.cell_width > end_x {
                break;
            }

            let col_name = column.name().to_string();

            let col_index = idx as u16;
            let col_ui = ColumnUI::new(col_name, col_index);
            column_uis.push(col_ui);
        }
        column_uis
    }

    // render banners
    fn render_bottom_banner(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut <TableUI as StatefulWidget>::State,
    ) {
        let table_banner = TableBanner::new();

        table_banner.render(area, buf, state)
    }

    // vertically segment area into 3 section
    // table_banner_top
    // table_main
    // table_banner_bottom
    fn vertical_segment_area(area: Rect) -> [Rect; 3] {
        return Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);
    }

    fn try_update_table_area(
        main_table_area: Rect, // new table area
        app_state: &mut <TableUI as StatefulWidget>::State,
    ) {
        let df_state = &mut app_state.dataframe_state;
        let [curr_height, curr_width] = df_state.table_area;

        let config_state = &app_state.config_state;
        if !(main_table_area.height == curr_height && main_table_area.width == curr_width) {
            // update the table area
            df_state.table_area = [main_table_area.height, main_table_area.width];
            df_state.refresh_renderable_table_size(config_state);
        }
    }
}

impl StatefulWidget for TableUI {
    // only do this when u need access to more than 2 state objects
    // since you can only assign 1 object to this trait.
    type State = App; // cheat and assign to app state so we get access to everthing?

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [_table_banner_top, table_main, table_banner_bottom] =
            TableUI::vertical_segment_area(area);

        // update the table area if needed
        // handles terminal resizing.
        TableUI::try_update_table_area(table_main, state);

        let [line_number_area, table_main] = Layout::horizontal([
            Constraint::Length(state.config_state.line_number_cell_width),
            Constraint::Fill(1),
        ])
        .areas(table_main);

        // render bottom banner
        self.render_bottom_banner(table_banner_bottom, buf, state);

        // render borders
        self.render_table_borders(table_main, buf);

        //////////////////////////////
        // segment table_main area
        /////////////////////////////

        let [header_area, values_area] = Layout::vertical([
            Constraint::Length(state.config_state.header_height),
            Constraint::Fill(1),
        ])
        .areas(table_main);

        // header
        self.render_header(header_area, buf, state);

        // render line numbers
        let line_number_ui = LineNumberUI::new();
        line_number_ui.render(line_number_area, buf, state);

        // render column ui values;
        let column_ui_widgets = self.get_column_uis_for_rendering(values_area, state);
        for column_ui in column_ui_widgets.into_iter() {
            column_ui.render(values_area, buf, state);
        }
    }
}

// pub struct TableUIState {
//     pub dataframe: DataFrame
// }

// impl TableUIState {

// }
