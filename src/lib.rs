use mint::Point2;

#[derive(Clone, Debug)]
pub struct Grid
{
    pub dim: GridDim,
    pub array:  Vec<u8>,
}

#[derive(Clone, Debug, Copy)]
pub struct GridDim
{
    pub width: isize,
    pub height: isize,
}

impl GridDim
{
    pub fn  index_map(&self, p: impl Into<Point2<isize>>) -> isize
    {
        let p = p.into();
        p.x + p.y * self.width
    }
}

impl std::ops::Deref for Grid
{
    type Target = GridDim;

    fn deref(&self) -> &Self::Target 
    {
        &self.dim
    }
}

impl std::ops::DerefMut for Grid
{
    fn deref_mut(&mut self) -> &mut Self::Target 
    {
        &mut self.dim    
    }
}

impl std::fmt::Display for Grid
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "width: {} -- height: {}\n", self.width, self.height)?;
        
        for row in 0..self.height
        {
            for col in 0..self.width
            {
                write!(f, "{} ", self.array[self.index_map([col, row]) as usize])?;
            }
            write!(f, "\n")?;
        }
        
        Ok(())
    }
}

impl Grid
{
    pub fn new(width: isize, height: isize) -> Self
    {
        assert!(width > 0);
        assert!(height > 0);

        let dim = GridDim { width, height };

        Grid
        {
            dim,
            array: vec![0; (width * height) as usize],
        }
    }

    pub fn neighbors_of(p: impl Into<Point2<isize>>) -> impl Iterator<Item = Point2<isize>>
    {
        let p = p.into();
        let x = p.x;
        let y = p.y;
        [
            [x-1, y-1].into(),  [x, y-1].into(),    [x+1, y-1].into(),
            [x-1, y].into(),                        [x+1, y].into(),
            [x-1, y+1].into(),  [x, y+1].into(),    [x+1, y+1].into()
        ].into_iter()
    }

    // return neighbors only in 
    pub fn neighbors_of_limited(p: impl Into<Point2<isize>>) -> impl Iterator<Item = Point2<isize>>
    {
        let p = p.into();
        let x = p.x;
        let y = p.y;

        [
                                [x, y-1].into(),
            [x-1, y].into(),                        [x+1, y].into(),
                                [x, y+1].into(),     
        ].into_iter()
    }

    /// Return an iterator of all the points that are inbounds
    pub fn valid_neighbors_of<'a>
    (
        &'a self, 
        neighbors: impl Iterator<Item = Point2<isize>> + 'a,
    ) -> impl Iterator<Item = Point2<isize>> + 'a
    {
        // Grid::neighbors_of(p)
        neighbors
        .filter(|&e|
        {
            0 <= e.x && e.x < self.width &&
            0 <= e.y && e.y < self.height
        })
    }

    pub fn distance_chebyshev(a: impl Into<Point2<isize>>, b: impl Into<Point2<isize>>) -> isize
    {
        let a = a.into();
        let b = b.into();

        isize::max(isize::abs(a.x - b.x), isize::abs(a.y - b.y))
    }

    pub fn distance_cityblock(a: impl Into<Point2<isize>>, b: impl Into<Point2<isize>>) -> isize
    {
        let a = a.into();
        let b = b.into();

        isize::abs(a.x - b.x) + isize::abs(a.y - b.y)
    }

    pub fn index(&self, p: impl Into<Point2<isize>>) -> Option<&u8>
    {
        // &self.array[Self::index_map(self.width, self.height, i, j)]
        self.array.get(self.index_map(p) as usize)
    }

    pub fn index_mut(&mut self, p: impl Into<Point2<isize>>) -> Option<&mut u8>
    {
        self.array.get_mut(self.dim.index_map(p) as usize)
    }

    pub fn sum_neighbors_with_outside_dead(&self, p: impl Into<Point2<isize>>) -> u8
    {
        Self::neighbors_of(p)
        .fold(0, 
        |acc, p|
        {
            acc + if let Some(v) = self.index(p) { if *v > 0 { 1 } else { 0 } } else { 0 }
        })
    }

    /// start at end and trace back to start
    fn trace_back(
        start: Point2<isize>, 
        end: Point2<isize>, 
        prev: &std::collections::HashMap<Point2<isize>, Point2<isize>>
    ) -> std::collections::HashSet<Point2<isize>>
    {
        let mut pt = end;

        let mut ret = std::collections::HashSet::new();

        while pt != start
        {
            ret.insert(pt);

            pt = prev[&pt];
        }
        
        ret
    }

    pub fn find_path_with_a_star(&self, start: Point2<isize>, end: Point2<isize>) -> Option<std::collections::HashSet<Point2<isize>>>
    {
        use std::collections::{HashMap, HashSet};

        let mut prev = HashMap::new();
        prev.insert(start, start);


        // #[derive(Eq, PartialEq, PartialOrd)]
        // struct PointDist(Point2<isize>, isize);
        // // impl PartialEq for PointDist
        // // {
        // //     fn eq(&self, other: &Self) -> bool 
        // //     {
        // //         self.1.eq(&other.1)
        // //     }
        // // }

        // // impl PartialOrd for PointDist
        // // {
        // //     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
        // //     {
        // //         self.1.partial_cmp(&other.1)    
        // //     }
        // // }

        // impl Ord for PointDist
        // {
        //     fn cmp(&self, other: &Self) -> std::cmp::Ordering 
        //     {
        //         // self.1.cmp(&other.1) // put max value on top
        //         other.1.cmp(&self.1) // put min on top
        //     }
        // }

        let mut g_distance_from_start_to_or_inf = 
            HashMap::from([(start, 0)]);

        let mut f_dist_from_to_end = 
            HashMap::from([(start, Grid::distance_cityblock(start, end))]);

        let mut frontier = HashSet::from([start]);

        while !frontier.is_empty()
        {
            // find cheapest node
            let mut current = None;
            let mut cost = isize::MAX;
            for &node in &frontier
            {
                let this_cost = *f_dist_from_to_end.get(&node).unwrap_or(&isize::MAX);
                if this_cost < cost
                {
                    current = Some(node);
                    cost = this_cost;
                }
            }

            let current = current.unwrap();
            if current == end
            {
                return Some(Grid::trace_back(start, end, &prev));
            }

            frontier.remove(&current);

            // for neighbor in self.valid_neighbors_of(current)
            for neighbor in self.valid_neighbors_of(Grid::neighbors_of_limited(current))
            {
                if *self.index(neighbor).unwrap() > 0 { continue }

                let tentative_g_score = 
                    g_distance_from_start_to_or_inf.get(&current).unwrap_or(&(isize::MAX-1)) + 1;
                if tentative_g_score < *g_distance_from_start_to_or_inf.get(&neighbor).unwrap_or(&(isize::MAX))
                {
                    prev.insert(neighbor, current);
                    g_distance_from_start_to_or_inf.insert(neighbor, tentative_g_score);
                    f_dist_from_to_end.insert(neighbor, tentative_g_score + Grid::distance_cityblock(neighbor, end));

                    if !frontier.contains(&neighbor)
                    {
                        frontier.insert(neighbor);
                    }
                }
            }
        }

        // we need a priority queue of points with estimates of distances to the end
        // g(n) is the distance from start to curr, h(n) is an estimate of distance from curr to end
        
        
        
        None
    }

    pub fn find_path_of_zeroes(&self, start: Point2<isize>, end: Point2<isize>) -> Option<std::collections::HashSet<Point2<isize>>>
    {
        use std::collections::{HashMap, VecDeque};
        
        // TODO: Check that start and end are inside this grid
        // TODO: Check that start and end are at points with zeroes

        // depth-first search
        let mut stack = VecDeque::new();
        stack.push_back(start);

        let mut prev = HashMap::new();
        prev.insert(start, start);

        while !stack.is_empty()
        {
            let p = stack.pop_front().unwrap();
            if p == end
            {
                return Some(Grid::trace_back(start, end, &prev));
            }

            // for neighbor in self.valid_neighbors_of(p)
            for neighbor in self.valid_neighbors_of(Grid::neighbors_of_limited(p))
            {
                if !prev.contains_key(&neighbor) && *self.index(p).unwrap() == 0
                {
                    prev.insert(neighbor, p);

                    stack.push_back(neighbor);
                }
            }
        }

        None
    }    
}

pub trait IsRules
{
    fn get_birth(&self)     -> impl Iterator<Item = &u8>;
    fn get_surive(&self)    -> impl Iterator<Item = &u8>;
}

#[derive(Debug, Clone)]
pub struct ConstRules<const B: usize, const S: usize>
{
    birth:      [u8; B],
    survive:    [u8; S],
}

impl<const B: usize, const S: usize> IsRules for ConstRules<B, S>
{
    fn get_surive(&self)    -> impl Iterator<Item = &u8> 
    {
        self.survive.iter()
    }

    fn get_birth(&self)     -> impl Iterator<Item = &u8> 
    {
        self.birth.iter()
    }
}

impl<const B: usize, const S: usize> ConstRules<B, S>
{
    pub const SEEDS:        ConstRules<1, 0> = ConstRules::new([2], []);
    pub const LIFE:         ConstRules<2, 1> = ConstRules::new([2, 3], [3]);
    pub const MAZE:         ConstRules<1, 5> = ConstRules::new([3], [1, 2, 3, 4, 5]);
    pub const MAZECETRIC:   ConstRules<1, 4> = ConstRules::new([3], [1, 2, 3, 4]);

    pub const fn new(birth: [u8; B], survive: [u8; S]) -> Self
    {
        ConstRules
        {
            birth,
            survive
        }
    }
}

#[derive(Clone, Debug)]
pub struct DynamicRules
{
    birth:      Vec<u8>,
    survive:    Vec<u8>,
}

impl IsRules for DynamicRules
{
    fn get_surive(&self)    -> impl Iterator<Item = &u8> 
    {
        self.survive.iter()    
    }

    fn get_birth(&self)     -> impl Iterator<Item = &u8> 
    {
        self.birth.iter()
    }
}

impl DynamicRules
{
    pub fn new(birth: &[u8], survive: &[u8]) -> Self
    {
        DynamicRules
        {
            birth:      Vec::from(birth),
            survive:    Vec::from(survive),
        }
    }
}

impl std::str::FromStr for DynamicRules
{
    type Err = String;
    fn from_str(s_in: &str) -> Result<Self, Self::Err> 
    {
        // find index of B, find index of S
        let b_loc = s_in.chars().enumerate().find(|c|c.1.to_ascii_lowercase() == 'b');
        let s_loc = s_in.chars().enumerate().find(|c| c.1.to_ascii_lowercase() == 's');




        if let (Some((b_loc, _)), Some((s_loc, _))) = (b_loc, s_loc)
        {
            // B123S123
            // 01234567
            // while let Some(d) = s_in.chars().skip(b_loc+1).map(|c| c.to_digit(10))
            // {

            // }

            // FIXME: Why doesn't fuse work to stop after first failed to_digit?
            let birth_rules = 
                s_in.chars().take(s_loc - b_loc - 1)
                .skip(b_loc+1)
                .map(|c| c.to_digit(10))
                .fuse()
                .flatten()
                .map(|digit| digit as u8)
                .collect::<Vec<u8>>();

            let survive_rules = 
                s_in.chars()
                .skip(s_loc+1)
                .map(|c| c.to_digit(10))
                .fuse()
                .flatten()
                .map(|d| d as u8)
                .collect::<Vec<u8>>();

            // for c in s_in.chars().skip(b_loc+1).take(s_loc - b_loc - 1)
            // {
            //     let n: u8 = 
            //         c.to_digit(10)
            //         .ok_or(String::from("digit unable to be parsed"))? as u8;
            //     birth_rules.push(n);
            // }

            // let mut survive_rules = Vec::new();
            // // B123S123
            // // 01234567
            // for c in s_in.chars().skip(s_loc+1)
            // {
            //     let n: u8 = 
            //         c.to_digit(10)
            //         .ok_or(String::from("digit unable to be parsed"))? as u8;
            //     survive_rules.push(n);
            // }
            
            // let birth_rules = Vec::new();
            
            // let survive_rules = Vec::new();

            return Ok(DynamicRules::new(&birth_rules, &survive_rules));
        }
        else
        {
            return Err(String::from("B or S not present in rule string"));
        }
    }
}

impl<const B: usize, const S: usize> From<ConstRules<B, S>> for DynamicRules
{
    fn from(value: ConstRules<B, S>) -> Self 
    {
        DynamicRules::new(&value.birth, &value.survive)
    }
}

pub struct Automaton<R>
{
    pub grid: Grid,
    other_grid: Grid,
    pub rules: R,
}

impl<R> Automaton<R>
where
    R: IsRules
{
    pub fn new(grid: Grid, rules: R) -> Self
    {
        Automaton
        {
            grid: grid.clone(),
            other_grid: grid,
            rules,
        }
    }

    pub fn new_from_dims(width: isize, height: isize, rules: R) -> Self
    {
        let grid = Grid::new(width, height);
        Automaton::new(grid, rules)
    }

    pub fn step(&mut self)
    {
        for y in 0..self.grid.height
        {
            for x in 0..self.grid.width
            {
                let p = Point2 { x, y };
                let num_alive_neighbors = self.grid.sum_neighbors_with_outside_dead(p);
                match self.grid.index(p).unwrap()
                {
                    // dead
                    0 if self.rules.get_birth().any(|&e| e == num_alive_neighbors) =>
                    {
                        *self.other_grid.index_mut(p).unwrap() = 1;
                    },
                    // alive
                    1.. if !self.rules.get_surive().any(|&e|e == num_alive_neighbors) =>
                    {
                        *self.other_grid.index_mut(p).unwrap() = 0; // set to 1 for some cool effects
                    },
                    _ => (),
                };
            }
        }

        self.grid = self.other_grid.clone();
    }
}

fn _main()
{
    let mut at = Automaton::new_from_dims(32, 32, ConstRules::<1, 0>::SEEDS);
    at.grid.array[100] = 1;
    at.grid.array[101] = 1;
    at.grid.array[0] = 1;
    at.grid.array[1] = 1;

    println!("{}", at.grid);
    at.step();
    println!("{}", at.grid);
    at.step();
    println!("{}", at.grid);

    for _ in 0..100
    {
        at.step();
    }

    println!("{}", at.grid);

    let mut at = Automaton::new(at.grid, ConstRules::<2, 1>::LIFE);
    at.step();

    println!("{}", at.grid);

    for _ in 0..10
    {
        at.step();
    }

    println!("{}", at.grid);
}