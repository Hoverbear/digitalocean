//! Marker types for requests.

pub trait Action {}

#[derive(Debug, Clone, Copy)]
pub struct List;
impl Action for List {}

#[derive(Debug, Clone, Copy)]
pub struct Get;
impl Action for Get {}

#[derive(Debug, Clone, Copy)]
pub struct Create;
impl Action for Create {}

#[derive(Debug, Clone, Copy)]
pub struct Update;
impl Action for Update {}

#[derive(Debug, Clone, Copy)]
pub struct Delete;
impl Action for Delete {}
