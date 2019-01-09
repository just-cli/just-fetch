use just_core::kernel::AvailableVersions;
use just_core::manifest::Manifest;
use just_core::result::BoxedResult;

pub struct Fetch<'a> {
    pub manifest: &'a Manifest,
    pub versions: &'a mut AvailableVersions,
}

impl<'a> Fetch<'a> {
    pub fn new(manifest: &'a Manifest, versions: &'a mut AvailableVersions) -> Self {
        Self { manifest, versions }
    }

    pub fn fetch_all_versions(&mut self) -> BoxedResult<()> {
        use log::debug;

        let name = self.manifest.package.name.as_str();
        debug!("Fetching all versions for package {}...", name);

        match self.manifest.versions {
            Some(ref versions) => {
                for version in just_versions::find_all_versions(versions) {
                    self.versions.add_version(&self.manifest.package, &version);
                }

                Ok(())
            }
            None => panic!(
                "Could not fetch versions for package {} because there is no way provided to fetch the versions",
                name
            ),
        }
    }

    pub fn needs_fetch(&self) -> bool {
        use log::debug;

        debug!(
            "Try deciding if package with alias {} needs an update",
            self.manifest.package.name.as_str()
        );

        self.versions
            .get_latest_versions_of(&self.manifest.package)
            .is_none()
    }
}
