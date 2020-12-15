use crate::props::{startPage, car, road, crashSign, enemy};
use rand::Rng;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod props;
use std::thread;
// use std::time:: Duration;
// use std::thread;

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Car {
    location: u16,
}
#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Road {
    location: u16,
    direction: i32
}
#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct State {
    start: bool,
    quit: bool,
    distance: u16,
}
#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Enemy {
    location: u16,
    exists: bool,
    enemy: String,
}

fn main()  {

  
//newing up moc classes
    let mut _car = Car {
        location: 80,
    };
    let mut _road = Road {
        location: 50,
        direction: 2,
    };
    let mut _state = State {
        start: false,
        quit: false,
        distance: 0
    };
    let mut _enemy = Enemy {
        location: 150,
        exists: false,
        enemy: String::from(""),
    };
    let mut rng = rand::thread_rng();



//Start of Key recongnization
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
    startPage();
   //detecting keydown events
    for c in stdin.keys() {

        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        let ranNum = rng.gen_range(1,31); 
        //i reckon this speaks for itself
        match c.unwrap() {
//start recognizing keys
            Key::Char('q') => { break;},
            Key::Char('s') => { road(_road.location,2);car(_car.location); _state.start = true},
            Key::Char(' ') => { 
                //road 
                if _state.start == true{
                    if _road.location < 11{
                        _road.location += 10; 
                        _road.direction = 3;
                  } else if _road.location > 150 {
                        _road.location -= 10; 
                        _road.direction = 1;
                    } else{
                     if ranNum <= 10{
                    _road.location -= ranNum; 
                    _road.direction = 1;
                   } else if ranNum >= 20{
                    _road.location += ranNum-20; 
                    _road.direction = 3;
                   } else {
                    _road.direction = 2;
                   }
                }
                   //start render
                road(_road.location,_road.direction); car(_car.location);
                   
                //crash
                if _road.location > _car.location || _road.location + 65  < _car.location {
                    _state.start=false; _road.location = 50; _car.location = 80;
                    crashSign();
                    thread::sleep_ms(2000);
                    print!("{}[2J", 27 as char);
                   
                }
            }
                //enemy
                // if _enemy
            },

            Key::Left => {  
                //road 
                if _state.start == true{
                    if _road.location < 11{
                        _road.location += 10; 
                        _road.direction = 3;
                   } else if _road.location > 150 {
                        _road.location -= 10; 
                        _road.direction = 1;
                    } else{
                        if ranNum <= 10{
                            _road.location -= ranNum; 
                            _road.direction = 1;
                           } else if ranNum >= 20{
                            _road.location += ranNum-20; 
                            _road.direction = 3;
                           } else {
                            _road.direction = 2;
                           }
                }
                   //car
                 if _car.location > 6 && _car.location < 250{
                 _car.location-=8;
                 }else{
                    _car.location+=8;
                 }
                  //start render
                    road(_road.location, _road.direction); car(_car.location);
                       //crash
                if _road.location > _car.location || _road.location + 65  < _car.location {
                    _state.start=false; _road.location = 50; _car.location = 80;
                    crashSign();
                    thread::sleep_ms(2000);
                    print!("{}[2J", 27 as char);
                }
                }
            },
            Key::Right => {  
                //road
                if _state.start == true{
                    if _road.location < 11{
                        _road.location += 10; 
                        _road.direction = 3;
                  } else if _road.location > 150 {
                        _road.location -= 10; 
                        _road.direction = 1;
                    } else{
                        if ranNum <= 10{
                            _road.location -= ranNum; 
                            _road.direction = 1;
                           } else if ranNum >= 20{
                            _road.location += ranNum-20; 
                            _road.direction = 3;
                           } else {
                            _road.direction = 2;
                           }
                }
                //car
                if _car.location > 2 && _car.location < 200{
                    _car.location+=8;
                    }else{
                        _car.location-=8;  
                    }
                //start render
                road(_road.location, _road.direction); car(_car.location);
                    //crash
                if _road.location > _car.location || _road.location + 65  < _car.location {
                    _state.start=false; _road.location = 50; _car.location = 80;
                    crashSign();
                    thread::sleep_ms(1000);
                    print!("{}[2J", 27 as char);
                    
                }
              }  },
                Key::Char('z') => {_state.start = true; },
           
            _ => (),

        }


        stdout.flush().unwrap();
       if _state.start == false {
            startPage();
        } 
    }
//end recognize key   
}

// fn roadMove(road_location, rand_num){
//   road_location 
// }
