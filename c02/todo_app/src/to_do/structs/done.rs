use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base::new(input_title, "done");
        Self { super_struct: base }
    }
}
