use crate::props::{startPage, car, road};
use rand::Rng;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod props;

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
        location: 100,
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
            Key::Char('q') => {_state.quit = true; break;},
            Key::Char('s') => { road(_road.location);car(_car.location); _state.start = true},
            Key::Char(' ') => { _road.location -= 8;  _road.location += rng.gen_range(0,15); road(_road.location);car(_car.location);},

            Key::Left => { _road.location -= 3;  _road.location += rng.gen_range(0,7); 
                 road(_road.location);
                 if _car.location > 2 && _car.location < 200{
                 _car.location-=8;
                 }else{
                    _car.location+=8;
                 }
                  car(_car.location);
                },
            Key::Right => {  _road.location -= 3;  _road.location += rng.gen_range(0,7); 
                road(_road.location);
                if _car.location > 2 && _car.location < 200{
                    _car.location+=8;
                    }else{
                        _car.location-=8;  
                    }
                car(_car.location);},
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
