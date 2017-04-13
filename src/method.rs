pub trait Method {}

#[derive(Debug, Clone, Copy)]
pub struct Get;
impl Method for Get {}

#[derive(Debug, Clone, Copy)]
pub struct Post;
impl Method for Post {}

#[derive(Debug, Clone, Copy)]
pub struct Delete;
impl Method for Delete {}
