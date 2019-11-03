/// Custom error type
pub enum Error {
    /// Sourcecode could be located
    SourceCodeNotFound,
    /// Git is not installed or did not function properly
    GitNotInstalled,
    /// Did not find any git data in the directory
    NoGitData,
    /// An IO error occoured while reading ./
    ReadDirectory,
    /// Not in a Git Repo
    NotGitRepo,
    /// Error while getting branch info
    BareGitRepo,
    /// Repository is a bare git repo
    ReferenceInfoError,
    /// Image probably doesn't exist or has wrong format
    ImageLoadError,
    /// Could not initialize the license detector
    LicenseDetectorError,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let content = match self {
            Error::SourceCodeNotFound => "Could not find any source code in this directory",
            Error::GitNotInstalled => "Git failed to execute",
            Error::NoGitData => "Could not retrieve git configuration data",
            Error::ReadDirectory => "Could not read directory",
            Error::NotGitRepo => "Could not find a valid git repo on the current path",
            Error::BareGitRepo => "Unable to run onefetch on bare git repos",
            Error::ReferenceInfoError => "Error while retrieving reference information",
            Error::ImageLoadError => "Could not load the specified image",
            Error::LicenseDetectorError => "Could not initialize the license detector",
        };
        write!(f, "{}", content)
    }
}
