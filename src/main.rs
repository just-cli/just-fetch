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
    use just_core::kernel::Kernel;
    use just_core::manifest::ManifestFiles;

    let opt: Opt = Opt::from_args();
    if let Some(pkg_name) = opt.package {
        let mut kernel = Kernel::load();

        if let Some(manifest) = ManifestFiles::scan(&kernel).load_manifest(&pkg_name) {
            let mut fetch = Fetch::new(&manifest, &mut kernel.versions);
            if let Err(e) = fetch.fetch_all_versions() {
                println!("Error: {:?}", e);
            }
        } else {
            println!("Package {:?} does not exists", pkg_name);
        }
    }
}
