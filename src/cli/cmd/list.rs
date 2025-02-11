use {
    crate::{
        cli::{
            cmd::{api::Api, wants_help, wants_raw},
            components::{Component, Help},
        },
        common::str::ToString,
        kurv::EggStatus,
        printth,
    },
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

    // if wants raw json output
    if wants_raw(args) {
        if eggs_summary_list.0.is_empty() {
            printth!("{}", "[]");
            return Ok(());
        }

        printth!("{}", serde_json::to_string_pretty(&eggs_summary_list)?);
        return Ok(());
    }

    if eggs_summary_list.0.is_empty() {
        printth!(indoc! {
            "\nthere are no <yellow>⬮</yellow>'s in the kurv <warn>=(</warn>
                
            <head>i</head> collect some <b>eggs</b> to get started:
              <dim>$</dim> <white>kurv</white> collect <green>my-egg.kurv</green>
            "
        });
        return Ok(());
    }

    printth!("\n<yellow>⬮</yellow> <dim>eggs snapshot</dim>\n");

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
    println!();

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
            options: Some(vec![
                ("-h, --help", vec![], "Prints this help message"),
                ("-j, --json", vec![], "Prints the response in json format")
            ]),
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
        EggStatus::PendingRemoval => Some(Color::Red),
        EggStatus::Restarting => Some(Color::Magenta),
    }
}

fn dim_by_status(status: EggStatus) -> bool {
    match status {
        EggStatus::PendingRemoval => true,
        EggStatus::Restarting => true,
        EggStatus::Pending => true,
        EggStatus::Running => false,
        EggStatus::Errored => false,
        EggStatus::Stopped => false,
    }
}

fn get_borders() -> (BorderBuilder, SeparatorBuilder) {
    let border = Border::builder()
        .bottom(HorizontalLine::new('╰', '╯', '┴', '─'))
        .top(HorizontalLine::new('╭', '╮', '┬', '─'))
        .left(VerticalLine::new('│'))
        .right(VerticalLine::new('│'));

    let separator = Separator::builder()
        .column(Some(VerticalLine::new('│')))
        .row(None)
        .title(Some(HorizontalLine::new('├', '┤', '┼', '─')));

    (border, separator)
}
