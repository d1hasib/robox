use structopt::StructOpt;

mod core;

fn main() {
    let robox = core::Robox::from_args();
    robox.draw();
}
