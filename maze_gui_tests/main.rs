use std::{collections::{HashMap, HashSet}, i32};
use ggez::mint::Point2;

use mazes::*;

struct MainState
{
    screen: ggez::graphics::ScreenImage,
    grid: Grid,
    _quad: ggez::graphics::Quad,
    quad_batch: ggez::graphics::InstanceArray,
}



/// Returns a set of points involved in the cheapest path,
/// which should give me a fast lookup in my draw calls
// fn a_star(grid: Grid, start: Point2<i32>, end: Point2<i32>) //-> HashSet<Point2<i32>>
// {
//     // ensure start and end are within grid?

//     // return Chebyshev distance from curr to end
//     #[inline]
//     fn min_dist_to_end(curr: Point2<i32>, end: Point2<i32>) -> i32
//     {
//         i32::max(i32::abs(curr.x - end.x), i32::abs(curr.y - end.y))
//     }

//     let mut open_set = HashSet::new();
//     open_set.insert(start);
    
//     let mut came_from = 
//         HashMap::<Point2<i32>, Point2<i32>>::new();
    
//     // cost from start to current, default to "infinity"
//     // read a None from get or get_mut as Infinity
//     let mut g_score = HashMap::new();
//     g_score.insert(start, 0);

//     let g_score_of = |pt: Point2<i32>| -> i32
//     {
//         if let Some(&g) = (&mut g_score).get(&pt)
//         {
//             g
//         }
//         else 
//         {
//             i32::MAX    
//         }
//     };
    
//     // return h + g
//     let f = |curr: Point2<i32>| -> i32
//     {
//         let h = min_dist_to_end(curr, end);
//         if let Some(g) = g_score.get(&curr)
//         {
//             h + *g
//         }
//         else
//         {
//             i32::MAX
//         }
//     };

//     while !open_set.is_empty()
//     {
//         // find point with minimum f
//         let mut min_cost = i32::MAX;
//         let mut min_pt: Option<Point2<i32>> = None;

//         // open_set.iter().min_by(|a, b|)
//         for pt in &open_set
//         {
//             let cost = f(*pt);
//             if cost < min_cost
//             {
//                 min_cost = cost;
//                 min_pt = Some(*pt);
//             }
//         }

//         // for every neighbor of the cheapest point
//         // look at grid and all neighbors (including diagonals)
//         let min_pt = min_pt.unwrap();
//         if min_pt == end
//         {
//             println!("end! {:?}", came_from);
//         }

//         open_set.remove(&min_pt);

//         let a = Grid::neighbors_of(min_pt.x as isize, min_pt.y as isize)
//         .filter(|&(x, y)|
//         {
//             matches!(grid.index(x, y), Some(_))
//         })
//         .for_each(|(x, y)|
//         {
//             let neighbor = [x as i32, y as i32].into();

//             let tentative_g_score = g_score_of(min_pt) + 1;
//             if tentative_g_score < g_score_of(neighbor)
//             {
//                 came_from.insert(neighbor, min_pt);
//                 g_score.insert(neighbor, tentative_g_score);
//             }
            
//         });
//     }

//     todo!()
// }

impl MainState
{
    fn new(context: &mut ggez::Context) -> Self
    {
        let grid = Grid::new(400, 400);
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

        let rotation = 0.0;
        let scale = [2.0, 2.0].into();
        let offset = [0.0, 0.0].into();

        let draw_params = 
        (0..grid.height).into_iter()
        .map(|r| 
        (0..grid.width).into_iter()
        .map(move |c|
        {
            let transform = 
            graphics::Transform::Values { 
                dest: [(2 * c) as f32, (2 * r) as f32].into(), 
                rotation, 
                scale, 
                offset, 
            };

            let color = 
            match grid.index([c, r]).unwrap()
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
                    let rules = ConstRules::<1, 4>::MAZECETRIC;

                    rs = Some(rules.into());
                },
                Up =>
                {
                    let rules = ConstRules::<2, 1>::LIFE;

                    rs = Some(rules.into())
                },
                Left =>
                {
                    let rules = ConstRules::<1, 5>::MAZE;

                    rs = Some(rules.into());
                },
                _ => (),
            };

            if let Some(r) = rs
            {
                let mut at = 
                Automaton::new(self.grid.clone(), r);

                at.grid.array[0] = 1;
                at.grid.array[1] = 1;
                *at.grid.index_mut([100, 100]).unwrap() = 1;
                *at.grid.index_mut([100, 101]).unwrap() = 1;

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
    .window_mode(conf::WindowMode::default().dimensions(800.0, 800.0));

    let (mut context, event_loop) = cb.build()?;

    let game = MainState::new(&mut context);

    event::run(context, event_loop, game);
}