use doryen_rs::*;
use specs::prelude::*;
use systems::render::render_doryen;

pub struct GameWorld {
    world: World,
}

impl GameWorld {
    pub fn new(world: World) -> Self {
        GameWorld { world }
    }
}

impl Engine for GameWorld {
    fn update(&mut self, _api: &mut DoryenApi) {}
    fn render(&mut self, api: &mut DoryenApi) {
        let mut renderer = render_doryen(api);
        use specs::RunNow;
        renderer.run_now(&self.world.res);
    }
}
