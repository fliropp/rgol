pub struct Rgol {
    pub game:  Vec<Vec<bool>>,
}

impl Rgol{
    pub fn new(x: usize, y: usize) -> Rgol {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        for _ in 0..x {
            let mut r: Vec<bool> = Vec::new();
            for _ in 0..y {
                r.push(false);
            }
            grid.push(r);
        }
        Rgol { game: grid }
    }

    pub fn update(&mut self, x: usize, y: usize) {
        self.game[x][y] = !self.game[x][y]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let g = Rgol::new(3,6);
        for i in 0..3 {
            for j in 0..6 {
                assert!(!g.game[i][j]);
            }
        }
        let q = Rgol::new(6,3);
        for i in 0..6 {
            for j in 0..3 {
                assert!(!q.game[i][j]);
            }
        }
    }

    #[test]
    fn update() {
        let mut g = Rgol::new(3,5);
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