extern crate rand;
// extern crate text_io;

// use std::vec;
use rand::Rng;
use std::io;
// use std::io::*;
use std::fmt;
// use text_io::read;

// #[macro_use] extern crate text_io;
fn main() {

    println!("This is the game of life!");

    let _width: u32 = 10;
    let _height: u32 = 10;
    let _start_cell_number: u32 = 25;

    println!("I have the variables");
    let map = generate_map(_width, _height);
    println!("I have the map");

    let mut game = Game {
        width: _width,
        height: _height,
        start_cell_number: _start_cell_number,
        map: map,
    };

    println!("I have the game");

    game.generate_alive();

    println!("Alive generated!");

    let mut ticks: u32 = 0;

    let mut input = String::new();

    loop {
        
        println!("Press enter");

        println!("{}", game);

        println!("Printed game");

        io::stdin().read_line(&mut input).expect("error: unable to read user input");

        println!("{}", input);

        if input != "\n" {
            break;
        }

        input.clear();
        
        game.tick();


        ticks += 1;

        println!("Game ticks: {}", ticks);

    }


}

struct Game {
    width: u32,
    height: u32,
    start_cell_number: u32,
    map: Vec <Vec<bool>>,
}

fn generate_map(_width: u32, _height: u32) -> Vec<Vec<bool>> {

    let mut map: Vec<Vec<bool>> = Vec::new();

    for _ in 0.._width {
        let mut new_vec: Vec<bool> = Vec::new();
        for _ in 0.._height {
            new_vec.push(false);
        }

        map.push(new_vec);
    }
    map
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut ret = String::from("");

        for _x in 0..self.width {
            for _y in 0..self.height {
                if self.map[_x as usize][_y as usize] {
                    ret.push('O');
                }
                else {
                    ret.push('X');
                }
            }
            ret.push('\n');
        }

        write!(f,"{}", ret)
    }
}

impl Game {

    fn generate_alive(&mut self){
        
        let mut rng = rand::thread_rng();
        for _ in 0..self.start_cell_number {
            let mut x = rng.gen_range(0 ,self.width);
            let mut y = rng.gen_range(0 ,self.height);

            while self.map[x as usize][y as usize] {

                x = rng.gen_range(0 ,self.width);
                y = rng.gen_range(0 ,self.height);
            }

            self.map[x as usize][y as usize] = true;
        }
    }

    fn tick(&mut self) {
        let mut new_map = self.map.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                new_map[x as usize][y as usize] = self.check_cell(&x, &y);
            }
        }
        self.map = new_map;
    }

    fn check_cell(&self, x: &u32, y: &u32) -> bool {

        let mut alive = 0;
        let mut i: i32 = -1;
        let mut a: i32;

        while i <= 1  {
            a = -1;
            while a <= 1 {
                if (*x as i32 + i) > 0 
                && (*x as i32 + i) < self.width as i32
                && (*y as i32 + a) > 0 
                && (*y as i32 + a) < self.height as i32
                && !(a == 0 && i == 0 ) {

                    if self.map[(*x as i32 + i) as usize][(*y as i32 + a) as usize] {
                        alive += 1;
                    }
                }
                a += 1;
            }
            i += 1;
        }

        match alive {
            2 => return self.map[*x as usize][*y as usize],
            3 => return true,
            _ => return false,
        }

 
    }
}
