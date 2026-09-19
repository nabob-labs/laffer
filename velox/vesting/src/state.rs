use {
    velox_types::vesting::{Position, Schedule},
    bolt::{Addr, Item, Map},
};

pub const UNLOCKING_SCHEDULE: Item<Schedule> = Item::new("unlocking_schedule");

pub const POSITIONS: Map<Addr, Position> = Map::new("position");
