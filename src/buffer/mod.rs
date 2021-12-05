use crate::common::{FrameId, PageId};
use crate::storage::{Page};

mod clock_replacer;
mod buffer_pool_manager;

/// Replacer is an abstract class that tracks page usage.
trait Replacer {

    /// Remove the victim frame as defined by the replacement policy
    /// Some(FrameId) - victim was found, None - no victim
    fn victim(&mut self) -> Option<FrameId>;

    /// Pins a frame, indicating that it should not be victimized until it is unpinned
    fn pin(&mut self, frame_id: FrameId);

    /// Unpins a frame, indicating that it can now be victimized
    fn unpin(&mut self, frame_id: FrameId);

    /// Return the number of elements in the replacer that can be victimized
    fn size(&mut self) -> usize;

}

/// BufferPoolManager reads disk pages to and from its internal buffer pool.
trait BufferPoolManager<'a> {

    /// Fetch the requested page from the buffer pool
    fn fetch_page(page_id: PageId) -> Option<&'a Page>;

    /// Unpin the target page from the buffer pool.
    fn unpin_page(page_id: PageId, is_dirty: bool) -> bool;

    /// Flushes the target page to disk.
    fn flush_page(page_id: PageId) -> bool;

    /// Creates a new page in the buffer pool.
    fn new_page(page_id: PageId) -> &'a Page;

    /// Deletes a page from the buffer pool.
    fn delete_page(page_id: PageId) -> bool;

    /// Flushes all the pages in the buffer pool to disk
    fn flush_all_pages();

}