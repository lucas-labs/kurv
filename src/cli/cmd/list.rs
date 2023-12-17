use {anyhow::Result, pico_args::Arguments};

use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::cli::cmd::api::Api;
use crate::common::str::ToString;
use crate::kurv::egg::EggStatus;
use crate::printth;
use cli_table::CellStruct;
use cli_table::{
    format::{
        Border, BorderBuilder, HorizontalLine, Justify, Separator, SeparatorBuilder, VerticalLine,
    },
    print_stdout, Cell, Color, Style, Table,
};

/// prints eggs state summary snapshot
pub fn run(_args: &mut Arguments) -> Result<()> {
    let (border, separator) = get_borders();
    let api = Api::new();
    let eggs_summary_list = api.eggs_summary()?;

    printth!("\n<white>⇝</white>  <dim>eggs</dim>\n");

    let rows: Vec<Vec<CellStruct>> = eggs_summary_list
        .0
        .iter()
        .map(|egg| {
            vec![
                egg.id.cell().bold(true).foreground_color(Some(Color::Blue)),
                egg.pid.cell(),
                egg.name.clone().cell(),
                egg.status
                    .str()
                    .to_lowercase()
                    .cell()
                    .bold(true)
                    .foreground_color(color_by_status(egg.status))
                    .dimmed(dim_by_status(egg.status)),
                egg.retry_count.cell().justify(Justify::Center),
                egg.uptime.clone().cell().justify(Justify::Center),
            ]
        })
        .collect();

    let table = rows
        .table()
        .title(vec![
            "#".cell().bold(true).foreground_color(Some(Color::Blue)),
            "pid".cell().bold(true).foreground_color(Some(Color::Blue)),
            "name".cell().bold(true).foreground_color(Some(Color::Blue)),
            "status"
                .cell()
                .bold(true)
                .foreground_color(Some(Color::Blue)),
            "↺"
                .cell()
                .bold(true)
                .foreground_color(Some(Color::Blue))
                .justify(Justify::Center),
            "uptime"
                .cell()
                .bold(true)
                .foreground_color(Some(Color::Blue))
                .justify(Justify::Center),
        ])
        .border(border.build())
        .separator(separator.build());

    print_stdout(table)?;
    Ok(())
}

fn color_by_status(status: EggStatus) -> Option<Color> {
    match status {
        EggStatus::Running => Some(Color::Green),
        EggStatus::Errored => Some(Color::Red),
        EggStatus::Stopped => Some(Color::Yellow),
        EggStatus::Pending => Some(Color::Blue),
    }
}

fn dim_by_status(status: EggStatus) -> bool {
    match status {
        EggStatus::Running => false,
        EggStatus::Errored => false,
        EggStatus::Stopped => false,
        EggStatus::Pending => true
    }
}

fn get_borders() -> (BorderBuilder, SeparatorBuilder) {
    let border = Border::builder()
        .bottom(HorizontalLine::new('└', '┘', '┴', '─'))
        .top(HorizontalLine::new('┌', '┐', '┬', '─'))
        .left(VerticalLine::new('│'))
        .right(VerticalLine::new('│'));

    let separator = Separator::builder()
        .column(Some(VerticalLine::new('│')))
        .row(Some(HorizontalLine::new('├', '┤', '┼', '─')));

    (border, separator)
}
