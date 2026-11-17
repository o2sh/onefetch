/// An author's name and email, after mailmap resolution.
#[derive(Debug, Hash, PartialOrd, Ord, Eq, PartialEq)]
pub struct Identity {
    pub name: gix::bstr::BString,
    pub email: gix::bstr::BString,
}
