
/// Frame ID (page buffer in RAM)
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct FrameId(pub usize);

/// Page ID (physical page on Disk)
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct PageId(pub usize);

pub const PAGE_SIZE: usize = 4096;
