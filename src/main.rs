mod fetch;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fetch")]
struct JustFetch {
    pub package: String,
}

fn main() {
    use self::fetch::Fetch;
    use just_core::kernel::Kernel;
    use just_core::manifest::ManifestFiles;

    let opt: JustFetch = JustFetch::from_args();
    let mut kernel = Kernel::load();

    if let Some(manifest) = ManifestFiles::scan(&kernel).load_manifest(&opt.package) {
        let mut fetch = Fetch::new(&manifest, &mut kernel.versions);
        if let Err(e) = fetch.fetch_all_versions() {
            println!("Error: {:?}", e);
        }
    } else {
        println!("Package {:?} does not exists", opt.package);
    }
}
