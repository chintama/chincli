use gunma::components::*;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::Window,
};
use specs::prelude::*;

pub struct Render<'a> {
    window: &'a mut Window,
}

impl<'a> Render<'a> {
    pub fn new(window: &'a mut Window) -> Self {
        Self { window }
    }
}

impl<'a, 'b> System<'a> for Render<'b> {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, Block>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Dir>,
    );

    fn run(&mut self, (e, pos, siz, bullet, block, player, dir): Self::SystemData) {
        let center = self.window.screen_size() / 2.0;
        let center = Pos::new(center.x, 0.0);
        let mut origin = Pos::new(0.0, 0.0);
        for (pos, _) in (&pos, &player).join() {
            origin = Pos::new(pos.x, 0.0);
        }

        self.window.clear(Color::WHITE).unwrap();

        let size = self.window.screen_size();
        let mut drw = |pos: Pos, siz: Size, col| {
            let pos = (Pos::new(pos.x, size.y - pos.y - siz.y) - origin) + center;
            let pos = Vector::new(pos.x, pos.y);
            let siz = Vector::new(siz.x, siz.y);
            let rect = Rectangle::new(pos, siz);
            self.window.draw(&rect, col);
        };

        for (e, pos, siz) in (&e, &pos, &siz).join() {
            let col = if player.get(e).is_some() {
                Col(Color::GREEN)
            } else if bullet.get(e).is_some() {
                Col(Color::BLACK)
            } else if block.get(e).is_some() {
                Col(Color::BLUE)
            } else {
                Col(Color::RED)
            };

            drw(*pos, *siz, col);

            if player.get(e).is_some() {
                let d = dir.get(e).unwrap_or(&Dir(1.0));
                let d = if d.0 > 0.0 { siz.x } else { -10.0 };
                drw(
                    *pos + Pos::new(d, 0.0),
                    Size::new(10.0, 10.0),
                    Col(Color::BLACK),
                );
            }
        }
    }
}
