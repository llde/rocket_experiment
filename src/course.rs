
struct Course{
    name : String,
    org : String,
    holder : String
}

impl Course{
    pub fn new(name : String, org : String, holder : String) -> Course{
        Course{name : name, org: org, holder:holder}
    }
}
