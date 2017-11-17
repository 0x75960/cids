extern crate cid;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate walkdir;

use cid::ContentIdentifier;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
#[structopt(name = "cids", about = "Content Identifiers")]
struct Opt {
    /// A flag, true if used in the command line.
    #[structopt(short = "d", long = "dir", help = "Activate dir mode")]
    dir: bool,

    /// Needed parameter, the first on the command line.
    #[structopt(help = "target")]
    target: String,
}

fn proc_dir(opt: &Opt) {
    show_title();
    for entry in WalkDir::new(opt.target.as_str()) {
        let e = entry.unwrap();
        if e.path().is_dir() == false {
            proc_file(e.path().to_str().unwrap());
        }
    }
}

fn proc_file(file: &str) {
    let cid = ContentIdentifier::of_file(file).unwrap();
    println!("{}\t{}\t{}\t{}", file, cid.sha256, cid.sha1, cid.md5);
}

fn show_title() {
    println!("Path\tSHA256\tSHA1\tMD5");
}

fn main() {
    let opt = Opt::from_args();

    if opt.dir {
        proc_dir(&opt);
    } else {
        show_title();
        proc_file(opt.target.as_str());
    }
}
