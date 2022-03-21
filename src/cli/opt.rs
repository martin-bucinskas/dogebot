use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "dogebot", about = "Dogebot usage commands.")]
pub struct Opt {
  #[structopt(short, long)]
  debug: bool,
}

impl Opt {
  pub fn from_args() -> Opt { <Opt as StructOpt>::from_args() }

  pub fn is_debug_enabled(&self) -> &bool { &self.debug }
}