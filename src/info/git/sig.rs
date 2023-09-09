#[derive(Hash, PartialOrd, Ord, Eq, PartialEq)]
pub struct Sig {
    pub name: gix::bstr::BString,
    pub email: gix::bstr::BString,
}

impl From<gix::actor::Signature> for Sig {
    fn from(gix::actor::Signature { name, email, .. }: gix::actor::Signature) -> Self {
        Self { name, email }
    }
}
