use just_core::blueprint::Blueprint;
use just_core::kernel::AvailableVersions;
use just_core::result::BoxedResult;

pub struct Fetch<'a> {
    pub blueprint: &'a Blueprint,
    pub versions: &'a mut AvailableVersions,
}

impl<'a> Fetch<'a> {
    pub fn new(blueprint: &'a Blueprint, versions: &'a mut AvailableVersions) -> Self {
        Self {
            blueprint,
            versions,
        }
    }

    pub fn fetch_all_versions(&mut self) -> BoxedResult<()> {
        use log::debug;

        let name = self.blueprint.package.name.as_str();
        debug!("Fetching all versions for package {}...", name);

        match self.blueprint.versions {
            Some(ref versions) => {
                for version in just_versions::find_all_versions(versions) {
                    self.versions.add_version(&self.blueprint.package, &version);
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
            self.blueprint.package.name.as_str()
        );

        self.versions
            .get_latest_versions_of(&self.blueprint.package)
            .is_none()
    }
}
