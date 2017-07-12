use core::{Unit, ObjId};
use core::map::PosHex;

#[derive(Debug, Clone)]
pub enum Command {
    Create(Create),
    Attack(Attack),
    MoveTo(MoveTo),
}

// TODO: this is a test command, remove it later
#[derive(Debug, Clone)]
pub struct Create {
    pub id: ObjId,
    pub unit: Unit,
}

#[derive(Debug, Clone)]
pub struct Attack {
    pub attacker_id: ObjId,
    pub target_id: ObjId,
}

#[derive(Debug, Clone)]
pub struct MoveTo {
    pub id: ObjId,
    pub path: Vec<PosHex>,
}