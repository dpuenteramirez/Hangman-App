#[derive(Debug)]
pub struct Word {
    pub(crate) id:i32,
    pub(crate) word:String,
    pub(crate) language:String,
    pub(crate) type_:String,
    pub(crate) group:String,
    pub(crate) size:i32,
}