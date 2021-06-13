use super::base::Base;
use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base::new(input_title, "pending");
        Self { super_struct: base }
    }
}

impl Create for Pending {}
impl Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
