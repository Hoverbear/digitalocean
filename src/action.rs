pub trait Action {}

#[derive(Debug, Clone, Copy)]
pub struct List;
impl Action for List {}

#[derive(Debug, Clone, Copy)]
pub struct Get;
impl Action for Get {}

#[derive(Debug, Clone, Copy)]
pub struct Post;
impl Action for Post {}

#[derive(Debug, Clone, Copy)]
pub struct Delete;
impl Action for Delete {}
