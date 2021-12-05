mod disk;

use crate::common::PageId;

pub struct Page {
    id: Option<PageId>,
    pin_count: usize,
    is_dirty: bool
}

impl Page {

    fn new() -> Self {
        Page {
            id: None,
            pin_count: 0,
            is_dirty: false
        }
    }

    fn set_page_id(&mut self, id: PageId) {
        self.id = Some(id);
    }

    fn invalidate_page_id(&mut self) {
        self.id = None;
    }

    fn get_page_id(&self) -> Option<PageId> {
        self.id
    }
}
