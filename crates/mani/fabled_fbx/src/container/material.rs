use crate::MaterialData;

pub struct Material<'a> {
    pub name: std::borrow::Cow<'a, str>,
    pub data: MaterialData,
}
