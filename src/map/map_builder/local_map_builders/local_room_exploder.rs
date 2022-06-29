use crate::prelude::*;
use bracket_lib::prelude::Rect;

pub struct RoomExploder {}

impl MetaLocalMapBuilder for RoomExploder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder)  {
        self.build(rng, build_data);
    }
}

impl RoomExploder {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomExploder> {
        Box::new(RoomExploder{})
    }

    fn build(&mut self, rng : &mut RandomNumberGenerator, build_data : &mut LocalMapBuilder) {
        let rooms : Vec<Rect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Room Explosions require a builder with room structures");
        }

        for room in rooms.iter() {
            let start = room.center();
            let n_diggers = rng.roll_dice(1, 20)-5;
            if n_diggers > 0 {
                for _i in 0..n_diggers {
                    let mut drunk_x = start.x;
                    let mut drunk_y = start.y;

                    let mut drunk_life = 20;
                    let mut did_something = false;

                    while drunk_life > 0 {
                        let drunk_idx = local_map_idx(drunk_x, drunk_y);
                        if build_data.map.tiles[drunk_idx] == LocalTileType::Wall {
                            did_something = true;
                        }
                        paint(&mut build_data.map, Symmetry::None, 1, drunk_x, drunk_y);
                        build_data.map.tiles[drunk_idx] = LocalTileType::DownStairs;

                        let stagger_direction = rng.roll_dice(1, 4);
                        match stagger_direction {
                            1 => { if drunk_x > 2 { drunk_x -= 1; } }
                            2 => { if drunk_x < LOCAL_MAP_WIDTH-2 { drunk_x += 1; } }
                            3 => { if drunk_y > 2 { drunk_y -=1; } }
                            _ => { if drunk_y < LOCAL_MAP_HEIGHT-2 { drunk_y += 1; } }
                        }

                        drunk_life -= 1;
                    }
                    if did_something { 
                        build_data.take_snapshot();
                    }

                    for t in build_data.map.tiles.iter_mut() {
                        if *t == LocalTileType::DownStairs {
                            *t = LocalTileType::Floor;
                        }
                    }
                }
            }
        }
    }
}