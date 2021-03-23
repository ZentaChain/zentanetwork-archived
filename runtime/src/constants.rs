
/// Money matters.
pub mod currency {
    use node_primitives::Balance;
    pub const MICRO_CHAIN: Balance = 1_000_000_000_000; // 10−6 	0.000001
    pub const MILLI_CHAIN: Balance = 1_000 * MICRO_CHAIN; // 10−3 	0.001
    pub const CENTI_CHAIN: Balance = 10 * MILLI_CHAIN; // 10−2 	0.01
    pub const CHAIN: Balance = 100 * CENTI_CHAIN;
}

/// Time.
pub mod time {
    use node_primitives::{BlockNumber, Moment};
    pub const MILLISECS_PER_BLOCK: Moment = 6000;
    pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    // 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 6 * HOURS;
    pub const EPOCH_DURATION_IN_SLOTS: u64 = {
        const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

        (EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
    };

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
}
