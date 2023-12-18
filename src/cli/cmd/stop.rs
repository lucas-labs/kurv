use anyhow::anyhow;

use {
    crate::cli::{
        cmd::{api::Api, wants_help, is_option_or_flag},
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

/// stops a runnig eg
pub fn run(args: &mut Arguments) -> Result<()> {
    if wants_help(args) {
        return help();
    }

    // let (border, separator) = get_borders();
    let api = Api::new();
    // let eggs_summary_list = api.eggs_summary()?;

    let cmd_arg: Result<Option<String>> = args.opt_free_from_str().map_err(|_| anyhow!("wrong usage"));

    match cmd_arg {
        Ok(maybe_arg) => {
            match maybe_arg {
                Some(id) => {
                    if is_option_or_flag(&id) {
                        return Err(anyhow!("wrong usage"));
                    }

                    printth!("\n<white>🥚</white> <dim>stopping egg {}</dim>\n", id);

                    let response = api.stop_egg(id);

                    match response {
                        Ok(egg) => {
                            printth!(indoc! {
                                "egg <green>{}</green> has been scheduled to be stopped
                                 
                                <head><b>i</b></head> you can check its status by running:
                                  <dim>$</dim> <white><b>kurv</b></white> stat <green>1</green>
                                "
                            }, egg.name);
                        },
                        _ => {}
                    }

                    Ok(())
                }
                None => Ok(())
            }
        }
        Err(e) => Err(e)
    }
}

fn help() -> Result<()> {
    printth!(
        "{}",
        Help {
            command: "kurv stop",
            summary: Some(indoc! {
                "schedules an egg to be stopped by the kurv server
                
                <head><b>example:</b></head>
                  <dim>-> if we want to stop process #<green>1</green> (you can use <white>kurv list</white>
                  to check ids):</dim>
                  
                  <dim>$</dim> <white><b>kurv</b></white> stop <green>1</green>"
            }),
            error: None,
            options: Some(vec![
                ("-h, --help", vec![], "Prints this help message"),
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
