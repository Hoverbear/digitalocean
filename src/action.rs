//! Marker types for requests.

/// A marker trait used by [`Request`](../request/struct.Request.html)
/// to signal which execution path should be taken.
pub trait Action {}

/// A list action uses a GET request with pagination.
#[derive(Debug, Clone, Copy)]
pub struct List;
impl Action for List {}

/// A get action uses a GET request.
#[derive(Debug, Clone, Copy)]
pub struct Get;
impl Action for Get {}

/// A create action uses a POST request.
#[derive(Debug, Clone, Copy)]
pub struct Create;
impl Action for Create {}

/// An update action uses a PUT request.
#[derive(Debug, Clone, Copy)]
pub struct Update;
impl Action for Update {}

/// A delete action uses a DELETE request.
#[derive(Debug, Clone, Copy)]
pub struct Delete;
impl Action for Delete {}
