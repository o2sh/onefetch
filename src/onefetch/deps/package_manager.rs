pub enum PackageManager {
    Cargo,
    GoModules,
    Npm,
    Pip,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PackageManager::Cargo => write!(f, "Cargo"),
            PackageManager::GoModules => write!(f, "Go Modules"),
            PackageManager::Npm => {
                if std::path::Path::new("./yarn.lock").exists() {
                    write!(f, "Yarn")
                } else {
                    write!(f, "Npm")
                }
            }
            PackageManager::Pip => write!(f, "Pip"),
        }
    }
}
