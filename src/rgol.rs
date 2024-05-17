use rand::Rng;
pub struct Rgol {
    pub game:  Vec<Vec<bool>>,
}

impl Rgol{
    pub fn new(x: usize, y: usize, rand: bool ) -> Rgol {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        for _ in 0..x {
            let mut r: Vec<bool> = Vec::new();
            for _ in 0..y {
                r.push(false);
            }
            grid.push(r);
        }
        if rand {
            grid = rand_init(grid);
        }
        Rgol { game: grid }
    }

    pub fn update(&mut self, x: usize, y: usize) {
        self.game[x][y] = !self.game[x][y]
    }

    pub fn init_glider(&mut self) {
        self.game[1][3] = true;
        self.game[2][1] = true;
        self.game[2][3] = true;
        self.game[3][2] = true;
        self.game[3][3] = true;

    }

    

    pub fn get_cell_value_wrap_around(&self, x_: i32, y_:i32) -> bool {
        let mut x = x_ as i32;
        let mut y = y_ as i32;
        if x < 0 {x = (self.game.len() - 1) as i32}
        if x > (self.game.len() - 1) as i32 {x = 0}
        if y < 0 {y = (self.game[0].len() - 1) as i32}
        if y > ((self.game[0].len() - 1) as i32) {y = 0}
        self.game[x as usize][y as usize]
    }

    pub fn get_alive_neighbours(&self, x_: usize, y_:usize) -> i32 {
        let mut alive_count = 0;
        let x = x_ as i32;
        let y = y_ as i32;
        for i in (x-1)..(x+2) {
            for j in (y-1)..(y+2) {
                if i == x && j == y {
                    continue
                }
                if self.get_cell_value_wrap_around(i, j) {
                    alive_count += 1;
                } 
            }
        }
        alive_count
    }

    pub fn run_the_rules(&mut self) {
        let mut nbs: i32;
        let mut swap_set = Rgol::new(self.game.len(), self.game[0].len(), false);
        for (i, r) in self.game.clone().iter_mut().enumerate() {
            for (j, _) in r.iter_mut().enumerate() {
                nbs = self.get_alive_neighbours(i,j);
                if nbs == 3 && !self.game[i][j] {
                    swap_set.game[i][j] = true;
                    continue
                }
                if nbs == 2 && self.game[i][j] {
                    swap_set.game[i][j] = true;
                    continue
                }
                if nbs == 3 && self.game[i][j] {
                    swap_set.game[i][j] = true;
                    continue
                }
                if nbs < 2 && self.game[i][j] {
                    swap_set.game[i][j] = false;
                    continue
                }
                if nbs < 3 && !self.game[i][j] {
                    swap_set.game[i][j] = false;
                    continue
                }
                if nbs > 3 {
                    swap_set.game[i][j] = false;
                }
               
            }
        }
        self.game = swap_set.game;
    }
}



fn rand_init(mut grid: Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut rng = rand::thread_rng();
    for x in grid.iter_mut() {
        for y in x.iter_mut() {
            if rng.gen_range(0..2) == 1 {
                *y = true;
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules() {
        let mut g: Rgol = Rgol::new(4,4, false);
        //#1 rule
        g.game[0][2] = true;
        g.game[1][2] = true;
        g.game[2][2] = true;
        g.game[0][2] = true;
        assert!(!g.game[1][1]);
        g.run_the_rules();
        assert!(g.game[1][1]);
        //#2 rule
        g = Rgol::new(4,4, false);
        g.game[0][2] = true;
        g.game[1][1] = true;
        g.game[1][2] = true;
        g.game[2][2] = true;
        g.game[0][2] = true;
        assert!(g.game[1][1]);
        g.run_the_rules();
        assert!(!g.game[1][1]);

        g = Rgol::new(4,4, false);
        g.game[0][2] = true;
        g.game[1][1] = true;
        g.game[2][2] = true;
        g.game[0][2] = true;
        assert!(g.game[1][1]);
        g.run_the_rules();
        assert!(!g.game[1][1]);

        //#3 rule
        g = Rgol::new(4,4, false);
        g.game[1][1] = true;
        g.game[2][2] = true;
        assert!(g.game[1][1]);
        g.run_the_rules();
        assert!(!g.game[1][1]);

        //#4 rule
        g = Rgol::new(4,4, false);
        g.game[1][2] = true;
        g.game[2][2] = true;
        assert!(!g.game[1][1]);
        g.run_the_rules();
        assert!(!g.game[1][1]);

        g = Rgol::new(6,6, false);
        g.game[0][0] = true;
        g.game[0][1] = true;
        g.game[0][2] = true;
        g.game[1][1] = true;
        g.game[1][2] = true;
        assert!(g.game[1][1]);
        g.run_the_rules();
        assert!(!g.game[1][1]);
    }

    #[test]
    fn run_the_jewels() {
        let mut g: Rgol = Rgol::new(4,4, false);
        assert!(!g.game[1][1]);
        g.game[0][0] = true;
        g.game[0][1] = true;
        g.game[0][2] = true;
        g.game[1][2] = true;
        g.run_the_rules();
        assert!(g.game[1][1]);
       
        g = Rgol::new(4,4, false);
        assert!(!g.game[0][3]);
        g.game[0][0] = true;
        g.game[3][0] = true;
        g.game[3][2] = true;
        g.game[3][3] = true;
        g.run_the_rules();
        assert!(g.game[0][3]);
    }

    #[test] 
    fn get_value_wrap_around() {
        let mut v:bool;
        let mut g: Rgol = Rgol::new(4,4, false);
        g.game[0][0] = true;
        g.game[0][3] = true;
        g.game[3][0] = true;
        g.game[3][1] = true;
        g.game[3][3] = true;
        for x in -1..1 as i32{
            for y in -1..1 as i32{
                v = g.get_cell_value_wrap_around(x, y);
                assert!(v)
            }
        }
        g = Rgol::new(4,4,false);
        g.game[0][0] = true;
        g.game[0][2] = true;
        g.game[0][3] = true;
        g.game[2][2] = true;
        g.game[2][3] = true;
        g.game[3][0] = true;
        g.game[3][2] = true;
        g.game[3][3] = true;

        for x in 2..4 as i32{
            for y in 2..4 as i32{
                v = g.get_cell_value_wrap_around(x, y);
                assert!(v)
            }
        }
    }

    #[test]
    fn get_neighbours_alive() {
        let mut g: Rgol = Rgol::new(4,4, false);
        g.game[2][3] = true;
        g.game[3][1] = true;
        g.game[3][2] = true;
        g.game[3][3] = true;

        let a_n = g.get_alive_neighbours(2, 2);
        let d_n = 8 - a_n;
        assert_eq!(a_n, 4);
        assert_eq!(d_n, 4);

        g = Rgol::new(4,3, false);
        g.game[0][2] = true;
        g.game[1][2] = true;
        g.game[3][0] = true;
        g.game[3][1] = true;
        g.game[3][2] = true;
        let a_n = g.get_alive_neighbours(0, 0);
        let d_n = 8 - a_n;
        assert_eq!(a_n, 5);
        assert_eq!(d_n, 3);

        g = Rgol::new(4,4, false);
        g.game[0][0] = true;
        g.game[0][2] = true;
        g.game[0][3] = true;
        g.game[3][0] = true;
        let a_n = g.get_alive_neighbours(3, 3);
        let d_n = 8 - a_n;
        assert_eq!(a_n, 4);
        assert_eq!(d_n, 4);
    }

    #[test]
    fn create() {
        let g = Rgol::new(3,6, false);
        for i in 0..3 {
            for j in 0..6 {
                assert!(!g.game[i][j]);
            }
        }
        let q = Rgol::new(6,3, false);
        for i in 0..6 {
            for j in 0..3 {
                assert!(!q.game[i][j]);
            }
        }
    }

    #[test]
    fn update() {
        let mut g = Rgol::new(3,5, false);
        assert_eq!(g.game.len(), 3);
        assert_eq!(g.game[0].len(), 5);
        assert_eq!(g.game[g.game.len() - 1].len(), 5);
        assert!(!g.game[1][2]);
        g.update(1,2);
        assert!(g.game[1][2]);
        assert!(!g.game[1][1]);
        assert!(!g.game[1][3]);
        assert!(!g.game[2][1]);
    }
}

