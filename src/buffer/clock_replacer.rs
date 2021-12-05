use crate::buffer::Replacer;
use crate::common::FrameId;

#[derive(Copy, Clone)]
pub struct FrameEntry {
    pinned: bool,
    ref_bit: bool,
}

pub struct ClockReplacer {
    count: usize,
    size: usize,
    hand_index: usize,
    entries: Vec<FrameEntry>,
}

impl ClockReplacer {

    fn new(count: usize) -> Self {
        let entry = FrameEntry{
            pinned: true,
            ref_bit: false,
        };
        ClockReplacer{
            count,
            size: 0,
            hand_index: 0,
            entries: vec![entry; count],
        }
    }

}

impl Replacer for ClockReplacer {

    fn victim(&mut self) -> Option<FrameId> {
        if self.size == 0 {
            return None;
        }

        // check every item twice because we could have every entry with ref_bit = true
        for _ in 0..(1 + (self.count * 2)) {
            let try_index = self.hand_index;
            self.hand_index = (try_index + 1) % self.count;

            let entry = &mut self.entries[try_index];

            if !entry.pinned {
                let ref_bit = entry.ref_bit;
                entry.ref_bit = false;

                if !ref_bit {
                    self.size -= 1;
                    entry.pinned = true;

                    return Some(FrameId(try_index + 1));
                }
            }
        }

        None
    }

    fn pin(&mut self, frame_id: FrameId) {
        let FrameId(id) = frame_id;
        let entry = &mut self.entries[id - 1];

        if !entry.pinned {
            entry.pinned = true;
            self.size -= 1;
        }
    }

    fn unpin(&mut self, frame_id: FrameId) {
        let FrameId(id) = frame_id;
        let entry = &mut self.entries[id - 1];

        entry.ref_bit = true;
        if entry.pinned {
            entry.pinned = false;
            self.size += 1;
        }
    }

    fn size(&mut self) -> usize {
        self.size
    }

}

#[cfg(test)]
mod tests {
    use crate::buffer::Replacer;
    use crate::common::FrameId;
    use super::ClockReplacer;

    #[test]
    fn unpinning_6_elements_of_7_frames_makes_size_of_6() {
        let mut replacer = ClockReplacer::new(7);

        replacer.unpin(FrameId(1));
        replacer.unpin(FrameId(2));
        replacer.unpin(FrameId(3));
        replacer.unpin(FrameId(4));
        replacer.unpin(FrameId(5));
        replacer.unpin(FrameId(6));
        replacer.unpin(FrameId(1));

        assert_eq!(replacer.size(), 6);
    }

    #[test]
    fn unpinning_6_elements_of_7_frames_then_victimizing_1_2_3_returns_correct_values() {
        let mut replacer = ClockReplacer::new(7);

        replacer.unpin(FrameId(1));
        replacer.unpin(FrameId(2));
        replacer.unpin(FrameId(3));
        replacer.unpin(FrameId(4));
        replacer.unpin(FrameId(5));
        replacer.unpin(FrameId(6));

        let victim1 = replacer.victim();
        assert!(matches!(victim1, Some(FrameId(1))));

        let victim2 = replacer.victim();
        assert!(matches!(victim2, Some(FrameId(2))));

        let victim3 = replacer.victim();
        assert!(matches!(victim3, Some(FrameId(3))));
    }

    #[test]
    fn unpinning_6_elements_of_7_frames_then_victimizing_1_2_3_then_pinning_3_4_makes_size_of_2() {
        let mut replacer = ClockReplacer::new(7);

        replacer.unpin(FrameId(1));
        replacer.unpin(FrameId(2));
        replacer.unpin(FrameId(3));
        replacer.unpin(FrameId(4));
        replacer.unpin(FrameId(5));
        replacer.unpin(FrameId(6));

        assert_eq!(replacer.size(), 6);

        replacer.victim();
        replacer.victim();
        replacer.victim();

        assert_eq!(replacer.size(), 3);

        replacer.pin(FrameId(3));
        assert_eq!(replacer.size(), 3);
        replacer.pin(FrameId(4));
        assert_eq!(replacer.size(), 2);
    }

    #[test]
    fn unpinning_6_elements_of_7_frames_then_victimizing_1_2_3_then_pinning_3_4_then_unpinning_4_victimizes_5_6() {
        let mut replacer = ClockReplacer::new(7);

        replacer.unpin(FrameId(1));
        replacer.unpin(FrameId(2));
        replacer.unpin(FrameId(3));
        replacer.unpin(FrameId(4));
        replacer.unpin(FrameId(5));
        replacer.unpin(FrameId(6));

        assert_eq!(replacer.size(), 6);

        let victim1 = replacer.victim();
        assert!(matches!(victim1, Some(FrameId(1))));
        let victim2 = replacer.victim();
        assert!(matches!(victim2, Some(FrameId(2))));
        let victim3 = replacer.victim();
        assert!(matches!(victim3, Some(FrameId(3))));

        assert_eq!(replacer.size(), 3);

        replacer.pin(FrameId(3));
        assert_eq!(replacer.size(), 3);
        replacer.pin(FrameId(4));
        assert_eq!(replacer.size(), 2);

        // we expect that the reference bit of 4 will be set to 1
        replacer.unpin(FrameId(4));

        let victim5 = replacer.victim();
        assert!(matches!(victim5, Some(FrameId(5))));
        let victim6 = replacer.victim();
        assert!(matches!(victim6, Some(FrameId(6))));
        let victim4 = replacer.victim();
        assert!(matches!(victim4, Some(FrameId(4))));
    }

    #[test]
    fn unpinning_3_elements_of_5_then_victimizing_1_2_3_then_unpinning_2_3_returns_correct_values() {
        let mut replacer = ClockReplacer::new(5);

        replacer.unpin(FrameId(1));
        replacer.unpin(FrameId(2));
        replacer.unpin(FrameId(3));

        assert_eq!(replacer.size(), 3);

        let victim1 = replacer.victim();
        assert!(matches!(victim1, Some(FrameId(1))));
        let victim2 = replacer.victim();
        assert!(matches!(victim2, Some(FrameId(2))));
        let victim3 = replacer.victim();
        assert!(matches!(victim3, Some(FrameId(3))));

        assert_eq!(replacer.size(), 0);

        replacer.unpin(FrameId(2));
        replacer.unpin(FrameId(3));

        assert_eq!(replacer.size(), 2);

        let victim2 = replacer.victim();
        assert!(matches!(victim2, Some(FrameId(2))));
        assert_eq!(replacer.size(), 1);

        let victim3 = replacer.victim();
        assert!(matches!(victim3, Some(FrameId(3))));
        assert_eq!(replacer.size(), 0);
    }

    #[test]
    fn unpinning_3_elements_of_3_then_pinning_them_then_victimizing_returns_none() {
        let mut replacer = ClockReplacer::new(3);

        replacer.unpin(FrameId(1));
        replacer.unpin(FrameId(2));
        replacer.unpin(FrameId(3));

        assert_eq!(replacer.size(), 3);

        replacer.pin(FrameId(1));
        replacer.pin(FrameId(2));
        replacer.pin(FrameId(3));

        assert_eq!(replacer.size(), 0);

        let victim = replacer.victim();
        assert!(matches!(victim, None))
    }

}