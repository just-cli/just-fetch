mod fetch;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fetch")]
struct Opt {
    #[structopt(long = "package")]
    pub package: Option<String>,
}

fn main() {
    use self::fetch::Fetch;
    use just_core::blueprint::Blueprints;
    use just_core::kernel::Kernel;

    let opt: Opt = Opt::from_args();
    if let Some(pkg) = opt.package {
        if let Some(blueprint) = Blueprints::scan().load(&pkg) {
            let mut kernel = Kernel::load();
            let mut fetch = Fetch::new(&blueprint, &mut kernel.versions);
            if let Err(e) = fetch.fetch_all_versions() {
                println!("Error: {:?}", e);
            }
        } else {
            println!("Package {:?} does not exists", pkg);
        }
    }
}
