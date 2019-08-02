use crate::render::Render;
use gunma::{
    components::{Asset as AssetId, Player, Pos, Size, CLASS_CHIBA},
    prelude::*,
    resources::Action,
    Config, Io, Systems,
};
use quicksilver::{
    geom::Vector,
    graphics::Image,
    input::{ButtonState, Key},
    lifecycle::{self, Event, Settings, State, Window},
    prelude::*,
    Result,
};
use std::{cell::RefCell, collections::HashMap};

pub type AssetsMap = HashMap<AssetId, Image>;

struct Screen {
    sys: Systems,
    action: Action,
    assets: AssetsMap,
}

fn load_image(s: &str) -> Image {
    Image::load(s).wait().unwrap()
}

fn load_assets() -> AssetsMap {
    let mut assets = HashMap::new();

    assets.insert(AssetId(1), load_image("ferris.png"));
    assets.insert(AssetId(2), load_image("ferris-f.png"));
    assets.insert(AssetId(3), load_image("cpp.png"));
    assets.insert(AssetId(4), load_image("bjarne.png"));

    assets.insert(AssetId(100), load_image("bubble.png"));
    assets.insert(AssetId(200), load_image("ground.png"));
    assets.insert(AssetId(900), load_image("gameover.png"));
    assets.insert(AssetId(901), load_image("beach.png"));

    assets
}

impl State for Screen {
    fn new() -> Result<Screen> {
        let mut sys = Systems::new().unwrap();
        let mut io = Io::new(
            Config::build()
                .game_server("ws://127.0.0.1:8090/ws/")
                .build(),
        )
        .unwrap();

        for t in io.get_all_terrain().unwrap() {
            sys.create_entity()
                .create_terrain_block(t.pos, t.size, t.asset);
        }

        let ack = io.login(CLASS_CHIBA).unwrap();

        sys.create_entity()
            .create_user(ack.spawn, Size::new(50.0, 40.0), ack.player, AssetId(1));

        Ok(Screen {
            action: Action::default(),
            sys,
            assets: load_assets(),
        })
    }

    fn update(&mut self, _: &mut Window) -> Result<()> {
        self.sys.add_action(self.action.clone());
        self.action.clear();
        self.sys.update();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                self.action.left();
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                self.action.right();
            }
            Event::Key(Key::Up, ButtonState::Pressed) => {
                self.action.jump();
            }
            Event::Key(Key::Z, ButtonState::Pressed) => {
                self.action.take();
            }
            Event::Key(Key::X, ButtonState::Pressed) => {
                self.action.drop();
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let render = Render::new(window, self.assets.clone());
        self.sys.render(render);
        Ok(())
    }
}

pub fn run() {
    lifecycle::run::<Screen>("Chintama", Vector::new(800, 600), Settings::default());
}
