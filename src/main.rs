use std::io::Write;

use clap::Parser;
use iw::assets::{SOD, load_all_graphics};
use iw::loader::DiskLoader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    files: Option<String>,

    #[arg(short, long)]
    pic: Option<usize>,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let variant = &SOD;

    let loader = DiskLoader {
        variant, // TODO: derive this from the files given?
        data_path: cli.files.unwrap_or("./".to_string()).into(),
        patch_path: None,
    };

    let (graphic, _font, _tiles, _texts) = load_all_graphics(&loader, &None)?;

    if let Some(pic_num) = cli.pic {
        let pic_ix = pic_num - variant.start_pics;
        if pic_ix < graphic.len() {
            std::io::stdout().write(&graphic[pic_ix].data).unwrap();
            std::io::stdout().flush().unwrap();
        } else {
            println!(
                "ERROR: pic {} does not exist, only {} in this file",
                pic_ix, variant.num_pics
            );
        }
    }

    Ok(())
}
