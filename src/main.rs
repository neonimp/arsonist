use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// Burn a file into a device or another file
struct Arson {
    /// input file (can be block or char dev)
    #[argh(option, short='i')]
    _if: String,
    /// output file (can be dev)
    #[argh(option, short='o')]
    of: String,
}

fn main() {
    let opts: Arson = argh::from_env();
    println!("{:?}", opts);
}
