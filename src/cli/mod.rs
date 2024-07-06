use clap::{parser::ValuesRef, Arg, ArgAction, ArgMatches, Command};

use crate::repository::{food::FoodRepository, rate::RateRepository};
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct CLIInstance {
    pub cmd: Command,
}

impl CLIInstance {
    pub fn new() -> Self {
        let cmd = Command::new("")
            .disable_colored_help(true)
            .disable_help_subcommand(true)
            .disable_help_flag(true)
            .no_binary_name(true)
            .subcommand(clap::Command::new("echo").arg(Arg::new("msg").action(ArgAction::Append)))
            .subcommand(clap::Command::new("help"))
            .subcommand(clap::Command::new("ping").about("pong"))
            .subcommand(clap::Command::new("version").about("print version information"))
            .subcommand(
                clap::Command::new("food")
                    .alias("吃啥")
                    .about("random ping N foods from list")
                    .arg(
                        Arg::new("amount")
                            .help("number of foods to pick")
                            .value_name("N")
                            .default_value("1"),
                    ),
            );
        //.subcommand(
        //    Command::new("rate")
        //        .about("get exchange rate")
        //        .arg(Arg::new("currency"))
        //        .arg(Arg::new("amount")),
        //);

        Self { cmd }
    }
    pub async fn execute(self, args_string: String) -> String {
        if args_string.starts_with("!") {}

        let args: Vec<&str>;
        if let Some(v) = args_string.strip_prefix("!") {
            args = v.split(" ").into_iter().collect();
        } else {
            args = args_string.split(" ").into_iter().collect();
        }

        let arg_matches = self.cmd.clone().get_matches_from(args);
        let matches = match arg_matches.subcommand() {
            Some(("echo", matches)) => echo(matches),
            Some(("ping", _)) => "pong".to_string(),
            Some(("food", matches)) => food(matches),
            Some(("rate", matches)) => rate(matches).await,

            Some(("help", _)) => self.cmd.clone().render_long_help().to_string(),
            Some(("version", _)) => CARGO_PKG_VERSION.to_string(),
            _ => self.cmd.clone().render_long_help().to_string(),
        };
        return matches;
    }
}
fn echo(matches: &ArgMatches) -> String {
    let msgs: Vec<String> = matches
        .get_many::<String>("msg")
        .unwrap_or_default()
        .map(|x| x.to_string())
        .collect();

    return msgs.join(" ");
}
fn food(matches: &ArgMatches) -> String {
    let amount: usize = match matches.get_one::<String>("amount") {
        Some(v) => match v.parse::<usize>() {
            Ok(u) => u,
            Err(e) => return e.to_string(),
        },
        None => 1,
    };

    let foods = FoodRepository::get_food(amount);
    foods.join(",")
}

async fn rate(matches: &ArgMatches) -> String {
    let curr = match matches.get_one::<String>("currency") {
        Some(s) => s,
        None => return "?".to_string(),
    };
    let amount: f64 = match matches.get_one::<String>("amount") {
        Some(v) => match v.parse::<f64>() {
            Ok(u) => u,
            Err(e) => return e.to_string(),
        },
        None => 1.0,
    };

    let (result, rate) = RateRepository::get_rate(curr, amount).await;
    format!("{:.2}/{}\nrate: {:.5}", result, curr, rate)
}
