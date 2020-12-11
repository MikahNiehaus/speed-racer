use crate::props::{startPage, car};

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
mod props;

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Car {
    location: 	u16,
}


// impl Left for Car {
//     fn age(self: &mut Self){
//         if self.location != 1{
//             self.location -= 5;

//         }
//     }
// }
// impl Right for Car {
//     fn age(self: &mut Self){
//         if self.location != 100{
//             self.location += 5;

//         }
//     }
// }

fn main()  {

    let mut _car = Car {
        location: 1,
    };

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
            Key::Char('q') => break,
            Key::Char('s') => {car(_car.location);},
            Key::Char('a') => {_car.location-=2; car(_car.location);},
            Key::Char('d') => {_car.location+=2; car(_car.location);},
            // Key::Char('5') => wrath(),
            // Key::Char('6') => chooseLust(),
            // Key::Char('7') => chooseLust(),

            _ => (),
        }

        stdout.flush().unwrap();
    }
}


