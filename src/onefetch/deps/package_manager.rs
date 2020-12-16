use strum::EnumIter;

#[derive(PartialEq, EnumIter)]
pub enum PackageManager {
    Cargo,
    GoModules,
    Npm,
    Pip,
    Pub,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PackageManager::Cargo => write!(f, "cargo"),
            PackageManager::GoModules => write!(f, "go modules"),
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Pip => write!(f, "pip"),
            PackageManager::Pub => write!(f, "pub"),
        }
    }
}
