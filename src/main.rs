use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
#[command(group(ArgGroup::new("input").required(true)))]
struct Args {
    /// TBW value. Setting this option outputs the DWPD.
    #[arg(short, long, group = "input")]
    tbw: Option<usize>,
    /// DWPD value. Setting this option outputs the TBW.
    #[arg(short, long, group = "input")]
    dwpd: Option<usize>,
    /// Size of drive in GB
    capacity: f32,
    /// Warranty in years
    #[arg(short, long)]
    warranty: f32,
}

/// A little app to easily convert TBW (terabytes written) to DWPD
/// (drive writes per day).
fn main() {
    let args = Args::parse();

    if let Some(tbw) = args.tbw {
        let dwpd: f32 = (tbw as f32 * 1000f32) / (args.warranty * 365f32);
        println!("{}", dwpd/args.capacity);
    }
    else if let Some(dwpd) = args.dwpd {
        let tbw: f32 = dwpd as f32 * args.capacity * args.warranty * 365f32;
        println!("TBW: {:.0}", tbw/1000f32);
    }
}
