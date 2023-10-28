use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "Your Name")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    debug: bool,

    #[clap(short, long)]
    verbose: bool,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "foo")]
    Foo(FooOpts),

    #[clap(name = "bar")]
    Bar(BarOpts),
}

#[derive(Clap)]
struct FooOpts {
    #[clap(short)]
    foo: bool,
}

#[derive(Clap)]
struct BarOpts {
    #[clap(short)]
    bar: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Foo(foo_opts) => {
            if opts.debug {
                println!("Debugging enabled");
            }

            if opts.verbose {
                println!("Running foo subcommand with options: {:?}", foo_opts);
            }
        }
        SubCommand::Bar(bar_opts) => {
            if opts.debug {
                println!("Debugging enabled");
            }

            if opts.verbose {
                println!("Running bar subcommand with options: {:?}", bar_opts);
            }
        }
    }
}
