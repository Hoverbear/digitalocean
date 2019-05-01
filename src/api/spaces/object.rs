use http::header::HeaderValue;

lazy_static! {
    /// ACL header value for read-only public access.
    pub static ref ACL_PUBLIC: HeaderValue = HeaderValue::from_static("public-read");
    /// ACL header value for private access.
    pub static ref ACL_PRIVATE: HeaderValue = HeaderValue::from_static("private");
    /// Content-Disposition header value for displaying objects inline.
    pub static ref DISPLAY_INLINE: HeaderValue = HeaderValue::from_static("inline");
    /// Content-Disposition header value for displaying objects as attachments.
    pub static ref DISPLAY_ATTACHMENT: HeaderValue = HeaderValue::from_static("attachment");
}

/// Access rule for an object.
#[derive(Clone, Copy)]
pub enum ObjectACL {
    PublicRead,
    Private,
}

/// Flag to indicate how the content should be displayed for an object.
#[derive(Clone, Copy)]
pub enum ContentDisposition {
    Attachment,
    Inline,
}

/* Helper impls */

impl ObjectACL {
    #[inline]
    pub(crate) fn header(&self) -> &'static HeaderValue {
        match *self {
            ObjectACL::PublicRead => &*ACL_PUBLIC,
            ObjectACL::Private => &*ACL_PRIVATE,
        }
    }
}

impl ContentDisposition {
    #[inline]
    pub(crate) fn header(&self) -> &'static HeaderValue {
        match *self {
            ContentDisposition::Inline => &*DISPLAY_INLINE,
            ContentDisposition::Attachment => &*DISPLAY_ATTACHMENT,
        }
    }
}

/* Default impls */

impl Default for ObjectACL {
    #[inline]
    fn default() -> Self {
        ObjectACL::Private
    }
}

impl Default for ContentDisposition {
    #[inline]
    fn default() -> Self {
        ContentDisposition::Attachment
    }
}
