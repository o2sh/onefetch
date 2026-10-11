/// An author's name and email, after mailmap resolution.
#[derive(Hash, PartialOrd, Ord, Eq, PartialEq)]
pub struct Identity {
    pub name: gix::bstr::BString,
    pub email: gix::bstr::BString,
}

impl From<gix::actor::Signature> for Identity {
    fn from(gix::actor::Signature { name, email, .. }: gix::actor::Signature) -> Self {
        Self { name, email }
    }
}
