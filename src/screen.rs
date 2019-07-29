use crate::render::Render;
use gunma::{Config, Systems};
use quicksilver::{
    geom::Vector,
    input::{ButtonState, Key},
    lifecycle::{self, Event, Settings, State, Window},
    Result,
};

struct Screen {
    sys: Systems,
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
        let render = Render::new(window);
        self.sys.render(render);
        Ok(())
    }
}

pub fn run() {
    lifecycle::run::<Screen>("Chintama", Vector::new(800, 600), Settings::default());
}
