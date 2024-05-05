use clap::Parser;
use rcli01::{Opts, SubCommand, process_csv};


fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => 
            process_csv(&opts.input, &opts.output)?,
    }
    Ok(())

}