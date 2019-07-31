use crate::render::Render;
use gunma::{Config, Systems, components::Asset as AssetId};
use quicksilver::{
    prelude::*,
    graphics::Image,
    geom::Vector,
    input::{ButtonState, Key},
    lifecycle::{self, Event, Settings, State, Window},
    Result,
};
use std::{collections::HashMap, cell::RefCell};

pub type AssetsMap = HashMap<AssetId, Image>;

struct Screen {
    sys: Systems,
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

    assets.insert(AssetId(100), load_image("bubble.png"));
    assets.insert(AssetId(200), load_image("ground.png"));
    assets.insert(AssetId(900), load_image("gameover.png"));
    assets.insert(AssetId(901), load_image("beach.png"));

    assets
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            sys: Systems::new(
                Config::build()
                    .game_server("ws://127.0.0.1:8090/ws/")
                    .build(),
            )
                .unwrap(),
            assets: load_assets(),
        })
    }

    fn update(&mut self, _: &mut Window) -> Result<()> {
        self.sys.update();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.sys.fetch_action(|action| match *event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                action.left();
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                action.right();
            }
            Event::Key(Key::Up, ButtonState::Pressed) => {
                action.jump();
            }
            Event::Key(Key::Z, ButtonState::Pressed) => {
                action.take();
            }
            Event::Key(Key::X, ButtonState::Pressed) => {
                action.drop();
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        });

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
