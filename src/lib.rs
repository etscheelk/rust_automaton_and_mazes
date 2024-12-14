// use cellular_automaton::rule::Survival;
// use rand::Rng;



// fn main() 
// {
//     const HALF_DIM: isize = 16;

//     let mut arr = [false; (2*HALF_DIM*2*HALF_DIM) as usize]; // 512x512 grid of bytes (inefficient)
//     let mut other = arr.clone();

//     let index_map = 
//     |x: isize, y: isize| -> usize
//     {
//         // 0, 0 should return a pixel somewhat in the middle
//         // range [-256, 255] in x and y
//         // assert!(x >= -HALF_DIM as isize && x <= HALF_DIM-1 as isize);
//         // assert!(y >= -HALF_DIM as isize && y <= HALF_DIM-1 as isize);

//         let nx = x + HALF_DIM;
//         let ny = y + HALF_DIM;

//         let s = nx + ny * 2 * HALF_DIM;

//         s as usize
//     };

//     let mut rng = rand::thread_rng();
//     for x in -10..10
//     {
//         for y in -10..10
//         {
//             // arr[index_map(x, y)] = rng.gen();
//             if let Some(v) = arr.get_mut(index_map(x, y))
//             {
//                 *v = rng.gen();
//             }
//         }
//     }

//     fn get_neighbors(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)>
//     {
//         let v = vec![
//             (x-1, y-1), (x, y-1),   (x+1, y-1),
//             (x-1, y),               (x+1, y),
//             (x-1, y+1), (x, y+1),   (x+1, y+1)
//         ].into_iter();

//         v
//     }

//     // steps
//     for _ in 0..1000
//     {
//         // iterate over every cell
//         for x in -HALF_DIM..HALF_DIM
//         {
//             for y in -HALF_DIM..HALF_DIM
//             {

//                 let n = 
//                 get_neighbors(x, y)
//                 .map(|(x, y)| index_map(x, y))
//                 .map(|e| arr.get(e))
//                 .fold(0, 
//                 |acc, e|
//                 {
//                     acc + if e == Some(&true) { 1 } else { 0 }
//                 });

//                 let b = arr[index_map(x, y)];
//                 match b
//                 {
//                     true =>
//                     {

//                     },
//                     false =>
//                     {
//                         if n == 2
//                         {
//                             other[index_map(x, y)] = true;
//                         }
//                     }
//                 };

//                 std::mem::swap(&mut arr, &mut other);
//             }
//         }
//     }

//     for y in -HALF_DIM..HALF_DIM
//     {
//         for x in -HALF_DIM..HALF_DIM
//         {
//             print!("{} ", if arr[index_map(x, y)] { 1 } else { 0 });
//         }
//         println!();
//     }

//     let mut at = cellular_automaton::life_like::Automaton::new(32, 32);
//     at.set_cells_on(&[16, 16, 16, 17, 16, 18]);
    
//     let seeds_rule = cellular_automaton::ruleset::BSC::new(&[2], &[], 1);
//     let mut at = cellular_automaton::life_like::Automaton::new(32, 32);
//     at.rules = seeds_rule;
//     at.randomize_cells(0.5);
//     println!("{:?}", <Vec<Vec<_>>>::from(&at));
//     at.step();
//     println!("{:?}", <Vec<Vec<_>>>::from(&at));
// }


#[derive(Clone, Debug)]
pub struct Grid
{
    pub width:  isize,
    pub height: isize,
    pub array:  Vec<u8>,
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
                write!(f, "{} ", self.array[Grid::index_map(self.width, self.height, col, row) as usize])?;
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

        Grid
        {
            width,
            height,
            array: vec![0; (width * height) as usize],
        }
    }

    fn neighbors_of(i: isize, j: isize) -> impl Iterator<Item = (isize, isize)>
    {
        [
            (i-1, j-1), (i, j-1),   (i+1, j-1),
            (i-1, j),               (i+1, j),
            (i-1, j+1), (i, j+1),   (i+1, j+1)
        ].into_iter()
    }

    pub fn index_map(width: isize, _height: isize, i: isize, j: isize) -> isize
    {
        i + j * width
    }

    pub fn index(&self, i: isize, j: isize) -> Option<&u8>
    {
        // &self.array[Self::index_map(self.width, self.height, i, j)]
        self.array.get(Self::index_map(self.width, self.height, i, j) as usize)
    }

    pub fn index_mut(&mut self, i: isize, j: isize) -> Option<&mut u8>
    {
        // &mut self.array[Self::index_map(self.width, self.height, i, j)]
        self.array.get_mut(Self::index_map(self.width, self.height, i, j) as usize)
    }

    pub fn sum_neighbors_with_outside_dead(&self, i: isize, j: isize) -> u8
    {
        Self::neighbors_of(i, j)
        .fold(0, 
        |acc, (i, j)|
        {
            acc + if let Some(v) = self.index(i, j) { if *v > 0 { 1 } else { 0 } } else { 0 }
        })
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
        for j in 0..self.grid.height
        {
            for i in 0..self.grid.width
            {
                let alive_neighbors = self.grid.sum_neighbors_with_outside_dead(i, j);
                match self.grid.index(i, j).unwrap()
                {
                    // dead
                    0 =>
                    {
                        if self.rules.get_birth().any(|e| *e == alive_neighbors)
                        {
                            *self.other_grid.index_mut(i, j).unwrap() = 1;
                        }
                    },
                    // alive
                    1.. =>
                    {
                        if !self.rules.get_surive().any(|e|*e == alive_neighbors)
                        {
                            *self.other_grid.index_mut(i, j).unwrap() = 0;
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