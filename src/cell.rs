
pub struct Cell {
    cell:  Vec<Vec<bool>>,
    status: bool,
    n_dead: i32, 
    n_alive: i32,
}

impl Cell {
    
    pub fn new(cell: Vec<Vec<bool>>) -> Cell {
        let mut dead = 0;
        let mut alive = 0;
        for i in 0..cell.len() {
            for j in 0..cell[i].len() {
                if i != 1 || j != 1 {
                    if cell[i][j] == true {
                        alive += 1;
                    } else {
                        dead += 1;
                    }
                }
            }
        }

        Cell{
            cell: cell.clone(),
            status: cell[1][1],
            n_dead: dead,
            n_alive: alive, 
        }
    }

    pub fn new_status(self) -> bool {
        self.n_alive > self.n_dead
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cell() {
        let t:Vec<Vec<bool>> = vec![
            vec![false, true, false], 
            vec![true, false, true],
            vec![false, true, false]
        ];
        
        let c = Cell::new(t);
        assert_eq!(c.n_alive, 4);
        assert_eq!(c.n_dead, 4);
    }

    #[test]
    fn new_status() {
        let t:Vec<Vec<bool>> = vec![
            vec![false, false, false], 
            vec![false, false, true],
            vec![true, true, true]
        ];
        let mut s = t.clone(); 
        
        let mut c = Cell::new(t);
        let mut ns = c.new_status();
        assert!(!ns);
        s[0][0] = true;
        c = Cell::new(s);
        ns = c.new_status();
        assert!(ns);

    }
}