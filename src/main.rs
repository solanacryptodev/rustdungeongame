mod map;
mod player;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;
use crate::map::Map;

// enum GameMode {
//     Menu,
// }

// Modeling a struct called State
struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}
impl State {
    // a constructor that initializes a new State instance and sets the default mode to Menu
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        spawn_player(&mut ecs, map_builder.player_start);
        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

// implement a trait for the State struct
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        // TODO: render draw buffer
    }
}

fn main() -> BError {

    let context = BTermBuilder::new()
        .with_fps_cap(30.0)
        .with_title("Dragon Game")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32,32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT,"dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
