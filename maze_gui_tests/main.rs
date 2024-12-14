use mazes::*;

struct MainState
{
    screen: ggez::graphics::ScreenImage,
    grid: Grid,
    _quad: ggez::graphics::Quad,
    quad_batch: ggez::graphics::InstanceArray,
}

impl MainState
{
    fn new(context: &mut ggez::Context) -> Self
    {
        let grid = Grid::new(512, 512);
        let screen = ggez::graphics::ScreenImage::new(context, ggez::graphics::ImageFormat::Rgba8Unorm, 1.0, 1.0, 1);
        let quad = ggez::graphics::Quad;
        let quad_batch = ggez::graphics::InstanceArray::new(context, None);

        println!("main state created");

        MainState 
        {  
            screen,
            grid,
            _quad: quad,
            quad_batch,
        }
    }
}

impl ggez::event::EventHandler for MainState
{
    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> 
    {
        // println!("start of draw call {}", context.time.ticks());
        use ggez::graphics;

        let mut canvas = graphics::Canvas::from_screen_image(context, &mut self.screen, graphics::Color::WHITE);
        // println!("canvas created");

        // canvas.set_sampler(graphics::Sampler::nearest_clamp());
        let ref grid = self.grid;
        // let ref q = self.quad;

        let draw_params = 
        (0..grid.height).into_iter()
        .map(|r| 
        (0..grid.width).into_iter()
        .map(move |c|
        {
            let transform = graphics::Transform::Values 
            { 
                dest: [(2 * r) as f32, (2 * c) as f32].into(), 
                rotation: 0.0, 
                scale: [2.0, 2.0].into(), 
                offset: [0.0, 0.0].into(), 
            };

            let color = 
            match grid.index(c, r).unwrap()
            {
                0 =>
                {
                    graphics::Color::WHITE
                },
                1.. =>
                {
                    graphics::Color::BLACK
                },
            };

            let param = 
                graphics::DrawParam::new()
                .transform(transform.to_bare_matrix())
                .color(color);

            param
        })).flatten();

        // println!("params created");

        self.quad_batch.set(draw_params);
        // println!("batch params set");
        // draw grid
        // for row in 0..self.grid.height
        // {
        //     for col in 0..self.grid.width
        //     {
        //         // let col = row;

        //         if *self.grid.index(col, row).unwrap() != 0
        //         {
        //             let transform = graphics::Transform::Values { dest: [(2 * row) as f32, (2 * col) as f32].into(), rotation: 0.0, scale: [2.0, 2.0].into(), offset: [0.0, 0.0].into() };
        //             let param = 
        //                 graphics::DrawParam::new()
        //                 .transform(transform.to_bare_matrix())
        //                 .color(graphics::Color::BLACK);
        //             canvas.draw(q, param);
                    
        //         }
        //     }
        // }

        canvas.draw(&self.quad_batch, graphics::DrawParam::default());
        // println!("canvas.draw called");

        // canvas.draw(&self.quad_batch, param);

        canvas.finish(context)?;
        // println!("canvas.finish");

        context.gfx.present(&self.screen.image(context))?;

        ggez::timer::yield_now();
        
        // println!("draw call done");

        // println!("end of draw call {}", context.time.ticks());

        Ok(())    
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn key_down_event(
            &mut self,
            context: &mut ggez::Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> 
    {
        use ggez::input::keyboard::KeyCode::*;

        println!("start of key down event {}", context.time.ticks());

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
        
        println!("end of key down event {}", context.time.ticks());

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

    let (mut context, event_loop) = cb.build()?;

    let game = MainState::new(&mut context);

    event::run(context, event_loop, game);
}