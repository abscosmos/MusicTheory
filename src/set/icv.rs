pub struct IntervalClassVector([u8; 6]);

impl IntervalClassVector {
    pub const fn new(arr: [u8; 6]) -> Option<Self> {
        // interval classes 1-5 (indices 0-4) can appear 0-12 times
        // interval class 6 (index 5, tritone) can only appear 0-6 times
        if arr[0] > 12
            || arr[1] > 12
            || arr[2] > 12
            || arr[3] > 12
            || arr[4] > 12
            || arr[5] > 6
        {
            return None;
        }
        Some(Self(arr))
    }
}