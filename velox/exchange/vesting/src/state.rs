use {
    velox_primitives::Addr,
    velox_storage::{Item, Map},
    velox_types::vesting::{Position, Schedule},
};

pub const UNLOCKING_SCHEDULE: Item<Schedule> = Item::new("unlocking_schedule");

pub const POSITIONS: Map<Addr, Position> = Map::new("position");
