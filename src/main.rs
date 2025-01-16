use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "differ", bin_name = "differ", version, about, long_about = None)]
enum DifferCli {
    /// Create patch from an old and new versions of a file
    Patch(PatchArgs),

    /// Apply patch created with this differ
    Apply(ApplyArgs),
}

#[derive(clap::Args)]
struct PatchArgs {
    /// Path to the old file for the diff
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    old: PathBuf,

    /// Path to the new file for the diff
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    new: PathBuf,

    /// Path to the output patch file
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,
}

#[derive(clap::Args)]
struct ApplyArgs {
    /// Path to the file to which to apply the path
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    file: PathBuf,

    /// Path to the patch file
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    patch: PathBuf,

    /// Path to the result file
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    match DifferCli::parse() {
        DifferCli::Patch(args) => differ::create_patch(args.old, args.new, args.output),
        DifferCli::Apply(args) => differ::apply_patch(args.file, args.patch, args.output),
    }
}
