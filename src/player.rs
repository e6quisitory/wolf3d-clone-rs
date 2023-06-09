use std::f64::consts::PI;

use crate::inputs_buffer::{InputsBuffer, lookCommand_t, moveCommand_t};
use crate::map::Map;
use crate::tiles::{Tile, DoorStatus};
use crate::utils::dda::RayCursor;
use crate::utils::ray::Ray;
use crate::utils::vec2d::Vec2;
use super::utils::vec2d::Point2;
use crate::inputs_buffer::doorCommand_t;

pub struct Player {
    pub location: Point2,
    pub viewDir: Vec2,
    pub east: Vec2,
    pub west: Vec2,
}

impl Player {
    pub fn New(location: Point2) -> Self {
        let viewDir = Point2::New(1.0, 1.0).UnitVector();
        let east = viewDir.Rotate(-PI/2.0);
        let west = viewDir.Rotate(PI/2.0);

        Player {
            location,
            viewDir,
            east,
            west
        }
    }

    pub fn Update(&mut self, inputsBuffer: &InputsBuffer, map: &mut Map, moveIncr: f64, swivelIncr: f64) {

        let mut proposedLoc: Point2 = self.location;

        match inputsBuffer.moveCommand {
            moveCommand_t::NORTH => { proposedLoc = self.location + self.viewDir*moveIncr; }
            moveCommand_t::SOUTH => { proposedLoc = self.location - self.viewDir*moveIncr; }
            moveCommand_t::EAST => { proposedLoc = self.location + self.east*moveIncr; }
            moveCommand_t::WEST => { proposedLoc = self.location + self.west*moveIncr; }
            moveCommand_t::NORTH_EAST => { proposedLoc = self.location + self.viewDir*moveIncr*0.7071067 + self.east*moveIncr*0.7071067; }
            moveCommand_t::NORTH_WEST => { proposedLoc = self.location + self.viewDir*moveIncr*0.7071067 + self.west*moveIncr*0.7071067; }
            moveCommand_t::NONE => {}
        }
        self.MoveIfValid(proposedLoc, map);

        match inputsBuffer.lookCommand {
            lookCommand_t::RIGHT => { self.viewDir = self.viewDir.Rotate(-swivelIncr*inputsBuffer.mouseAbsXrel as f64); }
            lookCommand_t::LEFT => { self.viewDir = self.viewDir.Rotate(swivelIncr*inputsBuffer.mouseAbsXrel as f64); }
            lookCommand_t::NONE => {}
        }

        self.east = self.viewDir.Rotate(-PI/2.0);
        self.west = self.viewDir.Rotate(PI/2.0);

        match inputsBuffer.doorCommand {
            doorCommand_t::OPEN => {
                let mut rayCursor = RayCursor::New(Ray::New(self.location, self.viewDir), self.location);
                while map.WithinMap(rayCursor.hitTile) {
                    rayCursor.GoToNextHit();
                    if rayCursor.GetDistToHitPoint() > 4.0 {
                        break;
                    } else {
                        match map.GetMutTile(rayCursor.hitTile) {
                            Tile::EMPTY(_) => {
                                continue;
                            },
                            Tile::DOOR(hitDoor) => {
                                if hitDoor.status == DoorStatus::CLOSED || hitDoor.status == DoorStatus::CLOSING {
                                    (*hitDoor).status = DoorStatus::OPENING;
                                    break;
                                } else if hitDoor.status == DoorStatus::OPENING {
                                    break;
                                }
                            },
                            Tile::NONE => panic!(),
                            _ => {
                                break;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn MoveIfValid(&mut self, proposedLocation: Point2, map: &Map) {
        match map.GetTile(proposedLocation.into()) {
            crate::tiles::Tile::EMPTY(_) => {
                self.location = proposedLocation;
            },
            crate::tiles::Tile::DOOR(door) => {
                if !door.PlayerTileHit() {
                    self.location = proposedLocation;
                }
            },
            crate::tiles::Tile::NONE => panic!(),
            _ => {}
        }
    }
}