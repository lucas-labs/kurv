use {
    crate::cli::{
        cmd::{api::Api, wants_help},
        components::{Component, Help},
    },
    crate::common::str::ToString,
    crate::kurv::EggStatus,
    crate::printth,
    anyhow::Result,
    cli_table::{
        format::{
            Border, BorderBuilder, HorizontalLine, Justify, Separator, SeparatorBuilder,
            VerticalLine,
        },
        print_stdout, Cell, CellStruct, Color, Style, Table,
    },
    indoc::indoc,
    pico_args::Arguments,
};

/// prints eggs state summary snapshot
pub fn run(args: &mut Arguments) -> Result<()> {
    if wants_help(args) {
        return help();
    }

    let (border, separator) = get_borders();
    let api = Api::new();
    let eggs_summary_list = api.eggs_summary()?;

    printth!("\n<white>🥚</white> <dim>eggs snapshot</dim>\n");

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
        .dimmed(true)
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
    println!("");

    Ok(())
}

fn help() -> Result<()> {
    printth!(
        "{}",
        Help {
            command: "kurv list",
            summary: Some(indoc! {
                "shows a snapshot table with a list of all collected
                eggs and their current statuses."
            }),
            error: None,
            options: Some(vec![("-h, --help", vec![], "Prints this help message"),]),
            subcommands: None
        }
        .render()
    );

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
        EggStatus::Pending => true,
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
        .row(None)
        .title(Some(HorizontalLine::new('├', '┤', '┼', '─')));

    (border, separator)
}
