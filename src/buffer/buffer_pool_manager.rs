use crate::buffer::{Replacer, BufferPoolManager};
use crate::common::PageId;
use crate::storage::Page;

struct BufferPoolManagerInstance<T>
    where T: Replacer
{
    replacer: T,
    pages: Vec<Page>
}

impl<T> BufferPoolManagerInstance<T>
    where T: Replacer
{

    fn new(replacer: T) -> Self {
        BufferPoolManagerInstance{
            replacer,
            pages: vec![]
        }
    }

}

impl<'a, T> BufferPoolManager<'a> for BufferPoolManagerInstance<T>
    where T: Replacer
{

    fn fetch_page(page_id: PageId) -> Option<&'a Page> {
        todo!()
    }

    fn unpin_page(page_id: PageId, is_dirty: bool) -> bool {
        todo!()
    }

    fn flush_page(page_id: PageId) -> bool {
        todo!()
    }

    fn new_page(page_id: PageId) -> &'a Page {
        todo!()
    }

    fn delete_page(page_id: PageId) -> bool {
        todo!()
    }

    fn flush_all_pages() {
        todo!()
    }

}
