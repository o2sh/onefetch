#[derive(PartialEq)]
pub enum PackageManager {
    Cargo,
    GoModules,
    Npm,
    Pip,
    Yarn,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PackageManager::Cargo => write!(f, "Cargo"),
            PackageManager::GoModules => write!(f, "Go Modules"),
            PackageManager::Npm => write!(f, "Npm"),
            PackageManager::Pip => write!(f, "Pip"),
            PackageManager::Yarn => write!(f, "Yarn"),
        }
    }
}
