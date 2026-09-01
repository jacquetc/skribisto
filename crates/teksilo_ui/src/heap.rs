// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Returning freed pages to the operating system.
//!
//! glibc lowers the program break only when the *top* chunk of an arena is free
//! and past its trim threshold. Everything freed below that stays mapped. So a
//! process that builds a large structure and then drops it keeps the pages, and
//! the writer who closes a project sees the number in their task manager stay
//! where it was. The memory really was freed; the operating system was simply
//! never told.
//!
//! Closing a project window frees tens of megabytes at once, and it is already a
//! rare and expensive moment: the close flow has just written or discarded a
//! bundle. That makes it the one place where walking every arena's free list is
//! proportionate. Measured on the bundled StarForgers example, a single call
//! there returns 20 to 29 MB per open/close cycle.
//!
//! Nothing here fixes a leak. It stops a correct free from being invisible.

/// Ask the allocator to return whatever free pages it can to the operating
/// system.
///
/// A no-op on every target that is not glibc. `malloc_trim` is a GNU extension,
/// and musl, macOS and Windows each release on a schedule of their own that this
/// has no way to ask about.
pub fn release_free_pages() {
    #[cfg(all(unix, target_env = "gnu"))]
    {
        // SAFETY: `malloc_trim` takes a pad in bytes, is safe to call from any
        // thread at any time, and only walks the allocator's own free lists. It
        // touches no memory the program still owns.
        unsafe {
            libc::malloc_trim(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What a unit test can honestly prove about this: that the call is reachable
    /// and harmless on the target it is being run on. Whether RSS actually falls
    /// depends on the allocator's arena state, which no test can arrange, and is
    /// measured instead by `scripts/automation_memory_profile.py`.
    #[test]
    fn releasing_free_pages_is_safe_to_call() {
        let scratch: Vec<u8> = vec![7; 8 << 20];
        assert_eq!(scratch.len(), 8 << 20);
        drop(scratch);
        release_free_pages();
        // Twice, because a second call with nothing left to release must also be
        // a no-op rather than an error path anyone has to think about.
        release_free_pages();
    }
}
