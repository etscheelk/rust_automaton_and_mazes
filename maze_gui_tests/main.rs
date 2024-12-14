use mazes::*;

struct MainState
{
    grid: Grid,
}

impl MainState
{
    fn new() -> Self
    {
        let grid = Grid::new(512, 512);

        MainState 
        {  
            grid,
        }
    }
}

impl ggez::event::EventHandler for MainState
{
    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> 
    {
        use ggez::graphics;

        let mut canvas = graphics::Canvas::from_frame(context, graphics::Color::WHITE);

        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        let q = graphics::Quad;

        // draw grid
        for row in 0..self.grid.height
        {
            for col in 0..self.grid.width
            {
                // let col = row;

                if *self.grid.index(col, row).unwrap() != 0
                {
                    let transform = graphics::Transform::Values { dest: [(2 * row) as f32, (2 * col) as f32].into(), rotation: 0.0, scale: [2.0, 2.0].into(), offset: [0.0, 0.0].into() };
                    let param = 
                        graphics::DrawParam::new()
                        .transform(transform.to_bare_matrix())
                        .color(graphics::Color::BLACK);
                    canvas.draw(&q, param);
                }
            }
        }

        canvas.finish(context)?;

        ggez::timer::yield_now();
        
        // println!("draw call done");

        Ok(())    
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> 
    {
        use ggez::input::keyboard::KeyCode::*;

        if let Some(kc) = input.keycode
        {
            let mut rs: Option<DynamicRules> = None;
            match kc
            {
                Space =>
                {
                    let rules = DynamicRules::new(&[2], &[]);

                    rs = Some(rules);
                },
                Right =>
                {
                    let rules = ConstRules::<1, 4>::MAZECETRIC.into();

                    rs = Some(rules);
                },
                _ => (),
            };

            if let Some(r) = rs
            {
                let mut at = 
                Automaton::new(self.grid.clone(), r);

                at.grid.array[0] = 1;
                at.grid.array[1] = 1;
                *at.grid.index_mut(100, 100).unwrap() = 1;
                *at.grid.index_mut(100, 101).unwrap() = 1;

                for _ in 0..10
                {
                    at.step();
                }

                self.grid = at.grid.clone();

                drop(at);
            }
        }
        
        Ok(())    
    }
}

fn main() -> ggez::GameResult
{
    use ggez::*;

    let cb = 
    ContextBuilder::new("maze_test", "Ethan Scheelk")
    .window_setup(conf::WindowSetup::default())
    .window_mode(conf::WindowMode::default().dimensions(1024.0, 1024.0));

    let (context, event_loop) = cb.build()?;

    let game = MainState::new();

    event::run(context, event_loop, game);
}