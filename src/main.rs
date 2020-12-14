use crate::props::{startPage, car, road, crashSign};
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
    location: 	u16,
}
#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Road {
    location: 	u16,
}
#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct State {
    start: 	bool,
    quit: bool,
}


fn main()  {

 
//newing up moc classes
    let mut _car = Car {
        location: 80,
    };
    let mut _road = Road {
        location: 50,
    };
    let mut _state = State {
        start: false,
        quit: false,
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
        //i reckon this speaks for itself
        match c.unwrap() {
//start recognizing keys
            Key::Char('q') => { break;},
            Key::Char('s') => { road(_road.location);car(_car.location); _state.start = true},
            Key::Up => { 
                //road 
                let x = rng.gen_range(0,13); 
                _road.location -= 6;
                _road.location += x;
                if _road.location < 14 {
                    _road.location += 13;
                   }else if _road.location > 150{
                    _road.location -= 13; 
                   }
                   //start render
                   if _state.start == true{
                road(_road.location); car(_car.location);
                   }
                //crash
                if _road.location > _car.location || _road.location + 65  < _car.location {
                    _state.start=false; _road.location = 50; _car.location = 80;
                    crashSign();
                    thread::sleep_ms(2000);
                    print!("{}[2J", 27 as char);
                    break;
                }
            },

            Key::Left => {  
                //road 
                let x = rng.gen_range(0,13); 
                _road.location -= 6;
                _road.location += x;
                if _road.location < 14 {
                    _road.location += 13;
                   }else if _road.location > 150{
                    _road.location -= 13; 
                   }
                
                   //car
                 if _car.location > 6 && _car.location < 250{
                 _car.location-=8;
                 }else{
                    _car.location+=8;
                 }
                  //start render
                  if _state.start == true{
                    road(_road.location); car(_car.location);
                       }
                       //crash
                if _road.location > _car.location || _road.location + 65  < _car.location {
                    _state.start=false; _road.location = 50; _car.location = 80;
                    crashSign();
                    thread::sleep_ms(2000);
                    print!("{}[2J", 27 as char);
                    break;
                }
                },
            Key::Right => {  
                //road
               let x = rng.gen_range(0,13); 
                _road.location -= 6;
                _road.location += x;
                if _road.location < 14 {
                    _road.location += 13;
                   }else if _road.location > 150{
                    _road.location -= 13; 
                   }
               
                //car
                if _car.location > 2 && _car.location < 200{
                    _car.location+=8;
                    }else{
                        _car.location-=8;  
                    }
                //start render
                if _state.start == true{
                    road(_road.location); car(_car.location);
                       }
                    //crash
                if _road.location > _car.location || _road.location + 65  < _car.location {
                    _state.start=false; _road.location = 50; _car.location = 80;
                    crashSign();
                    thread::sleep_ms(2000);
                    print!("{}[2J", 27 as char);
                    break;
                }
                    },
                Key::Char('z') => {_state.start = true; },
           
            _ => (),

        }


        stdout.flush().unwrap();
    }
//end recognize key   


    
}

// fn roadMove(road_location, rand_num){
//   road_location 
// }
