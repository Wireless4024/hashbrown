use core::{fmt, mem};

/// Single tag in a control group.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Tag(pub(super) u16);
impl Tag {
    /// Control tag value for an empty bucket.
    pub(crate) const EMPTY: Tag = Tag(0b1111_1111_1111_1111);

    /// Control tag value for a deleted bucket.
    pub(crate) const DELETED: Tag = Tag(0b1000_0000_0000_0000);
    
    pub(crate) const BITS: usize = 15;

    /// Checks whether a control tag represents a full bucket (top bit is clear).
    #[inline]
    pub(crate) const fn is_full(self) -> bool {
        self.0 & 0x8000 == 0
    }

    /// Checks whether a control tag represents a special value (top bit is set).
    #[inline]
    pub(crate) const fn is_special(self) -> bool {
        self.0 & 0x8000 != 0
    }

    /// Checks whether a special control value is EMPTY (just check 1 bit).
    #[inline]
    pub(crate) const fn special_is_empty(self) -> bool {
        debug_assert!(self.is_special());
        self.0 & 0x01 != 0
    }

    /// Creates a control tag representing a full bucket with the given hash.
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn full(hash: u64) -> Tag {
        // Constant for function that grabs the top 15 bits of the hash.
        const MIN_HASH_LEN: usize = if mem::size_of::<usize>() < mem::size_of::<u64>() {
            mem::size_of::<usize>()
        } else {
            mem::size_of::<u64>()
        };

        // Grab the top 15 bits of the hash. While the hash is normally a full 64-bit
        // value.
        let top15 = hash >> (MIN_HASH_LEN * 8 - Tag::BITS);
        Tag((top15 & 0x7fff) as u16) // truncation
    }
}
impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_special() {
            if self.special_is_empty() {
                f.pad("EMPTY")
            } else {
                f.pad("DELETED")
            }
        } else {
            f.debug_tuple("full").field(&(self.0 & 0x7FFF)).finish()
        }
    }
}

/// Extension trait for slices of tags.
pub(crate) trait TagSliceExt {
    /// Fills the control with the given tag.
    fn fill_tag(&mut self, tag: Tag);

    /// Clears out the control.
    #[inline]
    fn fill_empty(&mut self) {
        self.fill_tag(Tag::EMPTY)
    }
}

impl TagSliceExt for [Tag] {
    #[inline]
    fn fill_tag(&mut self, tag: Tag) {
        self.fill(tag)
    }

    #[inline]
    fn fill_empty(&mut self) {
        // SAFETY: We have access to the entire slice, so, we can write to the entire slice.
        //unsafe { self.as_mut_ptr().cast::<u8>().write_bytes(0xFF, size_of_val(self)) }
        self.fill_tag(Tag::EMPTY)
    }
}
