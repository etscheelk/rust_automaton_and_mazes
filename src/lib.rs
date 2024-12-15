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
    pub fn index_map(&self, p: impl Into<Point2<isize>>) -> isize
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

    // pub fn valid_neighbors_of(p: impl Into<Point2<isize>>) -> impl Iterator<Item = Point2<isize>>
    // {

    // }

    pub fn distance_chebyshev(a: impl Into<Point2<isize>>, b: impl Into<Point2<isize>>) -> isize
    {
        let a = a.into();
        let b = b.into();

        isize::max(isize::abs(a.x - b.x), isize::abs(a.y - b.y))
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

    pub fn find_path_of_zeroes(&self, start: Point2<isize>, end: Point2<isize>)
    {
        use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
        
        // TODO: Check that start and end are inside this grid
        // TODO: Check that start and end are at points with zeroes

        // depth-first search
        let mut q = VecDeque::<Point2<isize>>::new();
        q.push_back(start);

        while !q.is_empty()
        {
            let p = q.front().unwrap();

        }

        todo!()
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
                let alive_neighbors = self.grid.sum_neighbors_with_outside_dead(p);
                match self.grid.index(p).unwrap()
                {
                    // dead
                    0 =>
                    {
                        if self.rules.get_birth().any(|e| *e == alive_neighbors)
                        {
                            *self.other_grid.index_mut(p).unwrap() = 1;
                        }
                    },
                    // alive
                    1.. =>
                    {
                        if !self.rules.get_surive().any(|e|*e == alive_neighbors)
                        {
                            *self.other_grid.index_mut(p).unwrap() = 0;
                        }
                    }
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