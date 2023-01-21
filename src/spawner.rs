use crate::prelude::*;
use crate::State;

pub fn spawn_player(ecs: &mut World, player_start: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
