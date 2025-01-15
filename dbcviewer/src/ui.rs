// This example demonstrates how to use the selection list widget
//
// It was written by Héctor Ramón Jiménez <hector0193@gmail.com> and Andrew Wheeler <genusistimelord@gmail.com>

use crate::common;
use anyhow::anyhow;
use can_dbc::{Error, Signal, DBC};
use iced::widget::{container, responsive, scrollable, text, Button, Column, Row, Text};
use iced::{widget::Container, Element, Font, Length, Renderer, Size, Task, Theme};
use iced_aw::{selection_list::SelectionList, style::selection_list::primary};
use iced_table::table;
use native_dialog::FileDialog;

pub fn run() -> iced::Result {
    let size: Size = Size::new(1470f32, 700f32);

    iced::application(Example::title, Example::update, Example::view)
        .window_size(size)
        .run()
}

fn load(file: &str) -> anyhow::Result<DBC> {
    let buffer = common::read_to_utf8(file)?;
    let dbc = DBC::from_slice(&buffer);

    if let Err(e) = dbc {
        if let Error::Incomplete(dbc, ..) = e {
            return Ok(dbc);
        }
    } else {
        let dbc = dbc.map_err(|e| anyhow!("{:?}", e))?;
        return Ok(dbc);
    }
    Err(anyhow!("dbc does not support reading"))
}

struct Example {
    vec: Vec<String>,
    signals: Vec<String>,
    selected_message: String,
    selected_index: usize,
    manual_select: Option<usize>,
    dbc: anyhow::Result<DBC>,

    title: String,

    table_columns: Vec<TableColumn>,
    table_rows: Vec<TableRow>,
    header: scrollable::Id,
    body: scrollable::Id,
}

impl Default for Example {
    fn default() -> Self {
        let default_file = "./wm_uss_can2.dbc";
        let mut example = Self {
            title: "dbc viewer".to_string(),
            vec: vec![],
            signals: Vec::new(),
            selected_message: "".to_string(),
            selected_index: 0,
            manual_select: None,
            dbc: load(default_file),
            table_columns: vec![
                TableColumn::new(ColumnKind::SignalName),
                TableColumn::new(ColumnKind::Offset),
                TableColumn::new(ColumnKind::StartBit),
                TableColumn::new(ColumnKind::SignalSize),
                TableColumn::new(ColumnKind::ByteOrder),
                TableColumn::new(ColumnKind::ValueType),
                TableColumn::new(ColumnKind::Factor),
                TableColumn::new(ColumnKind::Min),
                TableColumn::new(ColumnKind::Max),
                TableColumn::new(ColumnKind::Unit),
            ],
            table_rows: vec![],
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
        };
        example.invalidate();
        if example.dbc.is_ok() {
            example.title = default_file.to_string();
        }
        example
    }
}

#[derive(Debug, Clone)]
enum Message {
    MessageSelected(usize, String),
    //SignalSelected(usize, String),
    OpenFile,
    UpdateUI,
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
}

impl Example {
    fn invalidate(&mut self) {
        let v = &mut self.vec;
        v.clear();
        self.signals.clear();
        self.table_rows.clear();
        match &self.dbc {
            Ok(dbc) => {
                for m in dbc.messages() {
                    v.push(format!(
                        "{}(0X{:X})",
                        m.message_name().clone(),
                        m.message_id().raw()
                    ));
                }
            }
            Err(e) => {
                v.push(e.to_string());
            }
        }
    }

    fn title(&self) -> String {
        format!("dbc viewer - {}", self.title)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MessageSelected(index, msg) => {
                self.selected_message = msg.clone();
                self.selected_index = index;
                self.manual_select = None;
                self.signals.clear();
                self.table_rows.clear();

                if let Ok(dbc) = &self.dbc {
                    for m in dbc.messages() {
                        let result = match &msg.find('(') {
                            Some(index) => &msg[..*index],
                            None => &msg,
                        };
                        if m.message_name() == result {
                            let signals = m.signals();

                            for s in signals {
                                self.signals.push(format!("{:<40} {:?} {:<10} {:<10} {:?} {:?} {:<10} {:<20} {:<20} {:>20} {:>20}", s.name(), s.multiplexer_indicator(), s.start_bit, s.signal_size,
                                                          s.byte_order(), s.value_type(), s.factor, s.offset, s.min, s.max,s.unit()));
                                self.table_rows.push(TableRow::generate(s));
                            }
                        } else {
                            //println!("{}  {}, {}", result, m.message_name(), msg);
                        }
                    }
                } // end if
            }
            //Message::SignalSelected(_index, _msg) => {}
            Message::OpenFile => {
                let file_path = FileDialog::new()
                    .add_filter("dbc Files", &["txt", "dbc", "*"])
                    .show_open_single_file();

                if let Ok(path) = file_path {
                    if let Some(path) = path {
                        let s = path.to_str();
                        if let Some(path) = s {
                            println!("{}", path);
                            self.title = path.to_string();
                            self.dbc = load(path);
                            let _ = self.update(Message::UpdateUI);
                        }
                    }
                }
            }
            Message::UpdateUI => {
                self.invalidate();
                let _ = Task::perform(async { 32 }, |_| {});
            }
            Message::SyncHeader(offset) => {
                return Task::batch(vec![scrollable::scroll_to(self.header.clone(), offset)])
            }
            Message::Resizing(index, offset) => {
                if let Some(column) = self.table_columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
            }
            Message::Resized => self.table_columns.iter_mut().for_each(|column| {
                if let Some(offset) = column.resize_offset.take() {
                    column.width += offset;
                }
            }),
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let selection_list = SelectionList::new_with(
            &self.vec[..],
            Message::MessageSelected,
            12.0,
            5.0,
            primary,
            self.manual_select,
            Font::default(),
        )
        .width(Length::Fixed(330.0))
        .height(Length::Fill);
        let left = Column::new()
            .spacing(10)
            .push(
                Button::new(Text::new("Open File"))
                    .width(Length::Fixed(330.0))
                    .on_press(Message::OpenFile),
            )
            .push(selection_list);
        /*
        let signal_list = SelectionList::new_with(
            &self.signals[..],
            Message::SignalSelected,
            12.0,
            5.0,
            primary,
            self.manual_select,
            Font::default(),
        )
        .width(Length::Fill)
        .height(Length::Fill); */

        let table = responsive(|size| {
            let mut table = table(
                self.header.clone(),
                self.body.clone(),
                &self.table_columns,
                &self.table_rows,
                Message::SyncHeader,
            );
            table = table.on_column_resize(Message::Resizing, Message::Resized);
            table = table.min_width(size.width);
            table.into()
        });

        //let table2 = Container::new(table).width(Length::Fill).height(Length::Fill);

        let row = Row::new()
            .spacing(10)
            .push(left)
            .push(table)
            .width(Length::Fill);
        Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

struct TableColumn {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}

impl TableColumn {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::SignalName => 335.0,
            ColumnKind::Offset => 70.0,
            ColumnKind::StartBit => 60.0,
            ColumnKind::SignalSize => 100.0,
            ColumnKind::ByteOrder => 115.0,
            ColumnKind::ValueType => 105.0,
            ColumnKind::Factor => 85.0,
            ColumnKind::Min => 80.0,
            ColumnKind::Max => 80.0,
            ColumnKind::Unit => 100.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

enum ColumnKind {
    SignalName,
    Offset,
    StartBit,
    SignalSize,
    ByteOrder,
    ValueType,
    Factor,
    Min,
    Max,
    Unit,
}

struct TableRow {
    signal: Signal,
}

impl TableRow {
    fn generate(signal: &Signal) -> Self {
        Self {
            signal: signal.clone(),
        }
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for TableColumn {
    type Row = TableRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::SignalName => "Signal name",
            ColumnKind::Offset => "Offset",
            ColumnKind::StartBit => "StartBit",
            ColumnKind::SignalSize => "SignalSize",
            ColumnKind::ByteOrder => "ByteOrder",
            ColumnKind::ValueType => "ValueType",
            ColumnKind::Factor => "Factor",
            ColumnKind::Unit => "Unit",
            ColumnKind::Min => "Min",
            ColumnKind::Max => "Max",
        };

        container(text(content))
            .width(Length::Fill)
            .center_y(24)
            .into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        _row_index: usize,
        row: &'a TableRow,
    ) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::SignalName => text(row.signal.name()).into(),
            ColumnKind::Offset => text(row.signal.offset).into(),
            ColumnKind::StartBit => text(row.signal.start_bit).into(),
            ColumnKind::Min => text(row.signal.min).into(),
            ColumnKind::Max => text(row.signal.max).into(),
            ColumnKind::SignalSize => text(row.signal.signal_size).into(),
            ColumnKind::ValueType => text(format!("{:?}", row.signal.value_type())).into(),
            ColumnKind::Factor => text(row.signal.factor()).into(),
            ColumnKind::Unit => text(row.signal.unit()).into(),
            ColumnKind::ByteOrder => text(format!("{:?}", row.signal.byte_order())).into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn footer(&'a self, _col_index: usize, _rows: &'a [TableRow]) -> Option<Element<'a, Message>> {
        None
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
