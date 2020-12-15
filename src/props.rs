use rand::Rng;

pub fn startPage() {

    println!("{}░██████╗██████╗░███████╗███████╗██████╗░  ██████╗░░█████╗░░█████╗░███████╗██████╗░", termion::cursor::Goto(1, 1));
    println!("{}██╔════╝██╔══██╗██╔════╝██╔════╝██╔══██╗  ██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗", termion::cursor::Goto(1, 2));
    println!("{}╚█████╗░██████╔╝█████╗░░█████╗░░██║░░██║  ██████╔╝███████║██║░░╚═╝█████╗░░██████╔╝", termion::cursor::Goto(1, 2));
    println!("{}░╚═══██╗██╔═══╝░██╔══╝░░██╔══╝░░██║░░██║  ██╔══██╗██╔══██║██║░░██╗██╔══╝░░██╔══██╗", termion::cursor::Goto(1, 3));
    println!("{}██████╔╝██║░░░░░███████╗███████╗██████╔╝  ██║░░██║██║░░██║╚█████╔╝███████╗██║░░██║", termion::cursor::Goto(1, 4));
    println!("{}╚═════╝░╚═╝░░░░░╚══════╝╚══════╝╚═════╝░  ╚═╝░░╚═╝╚═╝░░╚═╝░╚════╝░╚══════╝╚═╝░░╚═╝", termion::cursor::Goto(1, 5));
    println!("{}press \"q\" to exit at any time or press \"s\" to start, full screen recomended, use left and right key as controles", termion::cursor::Goto(1, 8), );

 
}

pub fn car ( x: u16){
    const y: u16 = 40;
    println!("
    {} ██████
	{}██   ███
	{}█ ▄█▄ ███
	{}█ ▀█▀ ███
	{}██   ████
	{} ████████
	{}   ██████▄
	{}   █████▓▓█
	{}   █████▓▓▓▓█
	{}   █████▓▓▓▓█
	{}   █████▓▓▓▓█
	{}   █████▓▓▓▓█
	{} ███████▓▓▓▓█
	{}██   ████████
	{}█ ▄█▄ ███████
	{}█ ▀█▀ ██████
	{}██   ██████
	{} ████████
	{}   █▒▒███"
	, 
    termion::cursor::Goto(x+1, y + 1),
    termion::cursor::Goto(x+1, y + 2),
    termion::cursor::Goto(x+1, y + 3),
    termion::cursor::Goto(x+1, y + 4),
    termion::cursor::Goto(x+1, y + 5),
    termion::cursor::Goto(x+1, y + 6),
    termion::cursor::Goto(x+1, y + 7),
    termion::cursor::Goto(x+1, y + 8),
    termion::cursor::Goto(x+1, y + 9),
    termion::cursor::Goto(x+1, y + 10),
    termion::cursor::Goto(x+1, y + 11),
    termion::cursor::Goto(x+1, y + 12),
    termion::cursor::Goto(x+1, y + 13),
    termion::cursor::Goto(x+1, y + 14),
    termion::cursor::Goto(x+1, y + 15),
    termion::cursor::Goto(x+1, y + 16),
    termion::cursor::Goto(x+1, y + 17),
    termion::cursor::Goto(x+1, y + 18),
    termion::cursor::Goto(x+1, y + 19));
    
}

pub fn enemy ( x: u16, tp: String){


    if (tp == "CAR"){
    const y: u16 = 30;
    println!("
    {} ██████
	{}██   ███
	{}█ ▄█▄ ███
	{}█ ▀█▀ ███
	{}██   ████
	{} ████████
	{}   ██████▄
	{}   █████▓▓█
	{}   █████▓▓▓▓█
	{}   █████▓▓▓▓█
	{}   █████▓▓▓▓█
	{}   █████▓▓▓▓█
	{} ███████▓▓▓▓█
	{}██   ████████
	{}█ ▄█▄ ███████
	{}█ ▀█▀ ██████
	{}██   ██████
	{} ████████
	{}   █▒▒███"
	, 
    termion::cursor::Goto(x+1, y + 1),
    termion::cursor::Goto(x+1, y + 2),
    termion::cursor::Goto(x+1, y + 3),
    termion::cursor::Goto(x+1, y + 4),
    termion::cursor::Goto(x+1, y + 5),
    termion::cursor::Goto(x+1, y + 6),
    termion::cursor::Goto(x+1, y + 7),
    termion::cursor::Goto(x+1, y + 8),
    termion::cursor::Goto(x+1, y + 9),
    termion::cursor::Goto(x+1, y + 10),
    termion::cursor::Goto(x+1, y + 11),
    termion::cursor::Goto(x+1, y + 12),
    termion::cursor::Goto(x+1, y + 13),
    termion::cursor::Goto(x+1, y + 14),
    termion::cursor::Goto(x+1, y + 15),
    termion::cursor::Goto(x+1, y + 16),
    termion::cursor::Goto(x+1, y + 17),
    termion::cursor::Goto(x+1, y + 18),
    termion::cursor::Goto(x+1, y + 19));
    }

    if (tp == "TIRE"){
        const y: u16 = 30;
        println!("
        {}  ███
        {}██   ██
        {}█ ▄█▄ █
        {}█ ▀█▀ █
        {}██   ██
        {}  ███
        "
        , 
        termion::cursor::Goto(x+1, y + 1),
        termion::cursor::Goto(x+1, y + 2),
        termion::cursor::Goto(x+1, y + 3),
        termion::cursor::Goto(x+1, y + 4),
        termion::cursor::Goto(x+1, y + 5),
        termion::cursor::Goto(x+1, y + 6),
        );
        }
    if (tp == "CAT"){
        const y: u16 = 30;
        println!("
{}   ██          ██
{}   █▒█        █▒█
{}  █▒███    ███▒█
{}  █▒████████▒▒█
{}  █▒████▒▒█▒▒██
{}  ████▒▒▒▒▒████
{}   █▒▒▒▒▒▒▒████
{}  █▒▒▒▒▒▒▒▒████     █
{} ██▒█▒▒▒▒▒█▒▒████  █▒█
{} █▒█●█▒▒▒█●█▒▒███ █▒▒█
{} █▒▒█▒▒▒▒▒█▒▒▒██ █▒▒█
{}  █▒▒▒=▲=▒▒▒▒███ ██▒█
{}  ██▒▒█♥█▒▒▒▒███  ██▒█
{}    ███▒▒▒▒████    █▒█
{}      ██████        ███
{}       █▒▒████      ██
{}       █▒▒▒▒▒████  ██
{}       █▒██▒██████▒█
{}       █▒███▒▒▒█████
{}     █▒████▒▒▒▒████
{}      █▒███▒██████ 
", 
        termion::cursor::Goto(x+1, y + 1),
        termion::cursor::Goto(x+1, y + 2),
        termion::cursor::Goto(x+1, y + 3),
        termion::cursor::Goto(x+1, y + 4),
        termion::cursor::Goto(x+1, y + 5),
        termion::cursor::Goto(x+1, y + 6),
        termion::cursor::Goto(x+1, y + 7),
        termion::cursor::Goto(x+1, y + 8),
        termion::cursor::Goto(x+1, y + 9),
        termion::cursor::Goto(x+1, y + 10),
        termion::cursor::Goto(x+1, y + 11),
        termion::cursor::Goto(x+1, y + 12),
        termion::cursor::Goto(x+1, y + 13),
        termion::cursor::Goto(x+1, y + 14),
        termion::cursor::Goto(x+1, y + 15),
        termion::cursor::Goto(x+1, y + 16),
        termion::cursor::Goto(x+1, y + 17),
        termion::cursor::Goto(x+1, y + 18),
        termion::cursor::Goto(x+1, y + 19),
        termion::cursor::Goto(x+1, y + 20),
        termion::cursor::Goto(x+1, y + 21),
    );
        }
}

pub fn crashSign (){
    const x: u16 = 30;
    const y: u16 = 20;
    println!("
    
{}██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
{}█░░░░░░░░██░░░░░░░░█░░░░░░░░░░░░░░█░░░░░░██░░░░░░████░░░░░░░░░░░░░░█░░░░░░░░░░░░░░░░███░░░░░░░░░░░░░░█░░░░░░░░░░░░░░█░░░░░░██░░░░░░█░░░░░░░░░░░░░░█░░░░░░░░░░░░███
{}█░░▄▀▄▀░░██░░▄▀▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░████░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀▄▀▄▀░░███░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀░░░░█
{}█░░░░▄▀░░██░░▄▀░░░░█░░▄▀░░░░░░▄▀░░█░░▄▀░░██░░▄▀░░████░░▄▀░░░░░░░░░░█░░▄▀░░░░░░░░▄▀░░███░░▄▀░░░░░░▄▀░░█░░▄▀░░░░░░░░░░█░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░█░░▄▀░░░░▄▀▄▀░░█
{}███░░▄▀▄▀░░▄▀▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░████░░▄▀░░█████████░░▄▀░░████░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░█████████░░▄▀░░██░░▄▀░░█░░▄▀░░█████████░░▄▀░░██░░▄▀░░█
{}███░░░░▄▀▄▀▄▀░░░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░████░░▄▀░░█████████░░▄▀░░░░░░░░▄▀░░███░░▄▀░░░░░░▄▀░░█░░▄▀░░░░░░░░░░█░░▄▀░░░░░░▄▀░░█░░▄▀░░░░░░░░░░█░░▄▀░░██░░▄▀░░█
{}█████░░░░▄▀░░░░█████░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░████░░▄▀░░█████████░░▄▀▄▀▄▀▄▀▄▀▄▀░░███░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░█
{}███████░░▄▀░░███████░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░████░░▄▀░░█████████░░▄▀░░░░░░▄▀░░░░███░░▄▀░░░░░░▄▀░░█░░░░░░░░░░▄▀░░█░░▄▀░░░░░░▄▀░░█░░▄▀░░░░░░░░░░█░░▄▀░░██░░▄▀░░█
{}███████░░▄▀░░███████░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░████░░▄▀░░█████████░░▄▀░░██░░▄▀░░█████░░▄▀░░██░░▄▀░░█████████░░▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀░░█████████░░▄▀░░██░░▄▀░░█
{}███████░░▄▀░░███████░░▄▀░░░░░░▄▀░░█░░▄▀░░░░░░▄▀░░████░░▄▀░░░░░░░░░░█░░▄▀░░██░░▄▀░░░░░░█░░▄▀░░██░░▄▀░░█░░░░░░░░░░▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░█░░▄▀░░░░▄▀▄▀░░█
{}███████░░▄▀░░███████░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░████░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀▄▀░░░░█
{}███████░░░░░░███████░░░░░░░░░░░░░░█░░░░░░░░░░░░░░████░░░░░░░░░░░░░░█░░░░░░██░░░░░░░░░░█░░░░░░██░░░░░░█░░░░░░░░░░░░░░█░░░░░░██░░░░░░█░░░░░░░░░░░░░░█░░░░░░░░░░░░███
{}██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████"
	, 
    termion::cursor::Goto(x+1, y + 1),
    termion::cursor::Goto(x+1, y + 2),
    termion::cursor::Goto(x+1, y + 3),
    termion::cursor::Goto(x+1, y + 4),
    termion::cursor::Goto(x+1, y + 5),
    termion::cursor::Goto(x+1, y + 6),
    termion::cursor::Goto(x+1, y + 7),
    termion::cursor::Goto(x+1, y + 8),
    termion::cursor::Goto(x+1, y + 9),
    termion::cursor::Goto(x+1, y + 10),
    termion::cursor::Goto(x+1, y + 11),
    termion::cursor::Goto(x+1, y + 12),
    termion::cursor::Goto(x+1, y + 13),
    );
    
}

pub fn road(x: u16, direction: i32){
    // let mut rng = rand::thread_rng();
    
   if direction == 3 {
    println!("
   {}          |                                  |                                      |
   {}         |                                  |                                      |
   {}        |                                                                         |
   {}      |                                                                         |
   {}      |                                  |                                      |
   {}      |                                  |                                      |
   {}      |                                                                         |
   {}      |                                                                         |
   {}      |                                  |                                      |
   {}    |                                  |                                      |
   {}    |                                                                         |
   {}      |                                                                         |
   {}      |                                  |                                      |
   {}  |                                  |                                      |
   {}  |                                                                         |
   {} |                                                                         |
   {} |                                  |                                      |
   {} |                                  |                                      |
   {} |                                                                         |
   {} |                                                                         |
   {}|                                  |                                      |
   {}|                                  |                                      |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                  |                                      |
   {}|                                  |                                      |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                  |                                      |
   {}|                                  |                                      |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                  |                                      |
   {}|                                  |                                      |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                  |                                      |
   {}|                                  |                                      |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                  |                                      |
   {}|                                  |                                      |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                  |                                      |
   {}|                                  |                                      |
   {}|                                                                         |
   {}|                                                                         |
   {}|                                  |                                      |", 
    
    termion::cursor::Goto(x+1, 100+ 1),
    termion::cursor::Goto(x+1, 100+ 2),
    termion::cursor::Goto(x+1, 100+ 3),
    termion::cursor::Goto(x+1, 100+ 4),
    termion::cursor::Goto(x+1, 100+ 5),
    termion::cursor::Goto(x+1, 100+ 6),
    termion::cursor::Goto(x+1, 100+ 7),
    termion::cursor::Goto(x+1, 100+ 8),
    termion::cursor::Goto(x+1, 100+ 9),
    termion::cursor::Goto(x+1, 100+ 10),
    termion::cursor::Goto(x+1, 100+ 11),
    termion::cursor::Goto(x+1, 100+ 12),
    termion::cursor::Goto(x+1, 100+ 13),
    termion::cursor::Goto(x+1, 100+ 14),
    termion::cursor::Goto(x+1, 100+ 15),
    termion::cursor::Goto(x+1, 100+ 16),
    termion::cursor::Goto(x+1, 100+ 17),
    termion::cursor::Goto(x+1, 100+ 18),
    termion::cursor::Goto(x+1, 100+ 19),
    termion::cursor::Goto(x+1, 100+ 20),
    termion::cursor::Goto(x+1, 100+ 21),
    termion::cursor::Goto(x+1, 100+ 22),
    termion::cursor::Goto(x+1, 100+ 23),
    termion::cursor::Goto(x+1, 100+ 24),
    termion::cursor::Goto(x+1, 100+ 25),
    termion::cursor::Goto(x+1, 100+ 26),
    termion::cursor::Goto(x+1, 100+ 27),
    termion::cursor::Goto(x+1, 100+ 28),
    termion::cursor::Goto(x+1, 100+ 29),
    termion::cursor::Goto(x+1, 100+ 30),
    termion::cursor::Goto(x+1, 100+ 31),
    termion::cursor::Goto(x+1, 100+ 32),
    termion::cursor::Goto(x+1, 100+ 33),
    termion::cursor::Goto(x+1, 100+ 34),
    termion::cursor::Goto(x+1, 100+ 35),
    termion::cursor::Goto(x+1, 100+ 36),
    termion::cursor::Goto(x+1, 100+ 37),
    termion::cursor::Goto(x+1, 100+ 38),
    termion::cursor::Goto(x+1, 100+ 39),
    termion::cursor::Goto(x+1, 100+ 40),
    termion::cursor::Goto(x+1, 100+ 41),
    termion::cursor::Goto(x+1, 100+ 42),
    termion::cursor::Goto(x+1, 100+ 43),
    termion::cursor::Goto(x+1, 100+ 44),
    termion::cursor::Goto(x+1, 100+ 45),
    termion::cursor::Goto(x+1, 100+ 46),
    termion::cursor::Goto(x+1, 100+ 47),
    termion::cursor::Goto(x+1, 100+ 48),
    termion::cursor::Goto(x+1, 100+ 49),
    termion::cursor::Goto(x+1, 100+ 50),
    
 );
} else if direction == 2 {
    println!("
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {}|                                  |                                      |
    {}|                                  |                                      |", 
     
     termion::cursor::Goto(x+1, 100+ 1),
     termion::cursor::Goto(x+1, 100+ 2),
     termion::cursor::Goto(x+1, 100+ 3),
     termion::cursor::Goto(x+1, 100+ 4),
     termion::cursor::Goto(x+1, 100+ 5),
     termion::cursor::Goto(x+1, 100+ 6),
     termion::cursor::Goto(x+1, 100+ 7),
     termion::cursor::Goto(x+1, 100+ 8),
     termion::cursor::Goto(x+1, 100+ 9),
     termion::cursor::Goto(x+1, 100+ 10),
     termion::cursor::Goto(x+1, 100+ 11),
     termion::cursor::Goto(x+1, 100+ 12),
     termion::cursor::Goto(x+1, 100+ 13),
     termion::cursor::Goto(x+1, 100+ 14),
     termion::cursor::Goto(x+1, 100+ 15),
     termion::cursor::Goto(x+1, 100+ 16),
     termion::cursor::Goto(x+1, 100+ 17),
     termion::cursor::Goto(x+1, 100+ 18),
     termion::cursor::Goto(x+1, 100+ 19),
     termion::cursor::Goto(x+1, 100+ 20),
     termion::cursor::Goto(x+1, 100+ 21),
     termion::cursor::Goto(x+1, 100+ 22),
     termion::cursor::Goto(x+1, 100+ 23),
     termion::cursor::Goto(x+1, 100+ 24),
     termion::cursor::Goto(x+1, 100+ 25),
     termion::cursor::Goto(x+1, 100+ 26),
     termion::cursor::Goto(x+1, 100+ 27),
     termion::cursor::Goto(x+1, 100+ 28),
     termion::cursor::Goto(x+1, 100+ 29),
     termion::cursor::Goto(x+1, 100+ 30),
     termion::cursor::Goto(x+1, 100+ 31),
     termion::cursor::Goto(x+1, 100+ 32),
     termion::cursor::Goto(x+1, 100+ 33),
     termion::cursor::Goto(x+1, 100+ 34),
     termion::cursor::Goto(x+1, 100+ 35),
     termion::cursor::Goto(x+1, 100+ 36),
     termion::cursor::Goto(x+1, 100+ 37),
     termion::cursor::Goto(x+1, 100+ 38),
     termion::cursor::Goto(x+1, 100+ 39),
     termion::cursor::Goto(x+1, 100+ 40),
     termion::cursor::Goto(x+1, 100+ 41),
     termion::cursor::Goto(x+1, 100+ 42),
     termion::cursor::Goto(x+1, 100+ 43),
     termion::cursor::Goto(x+1, 100+ 44),
     termion::cursor::Goto(x+1, 100+ 45),
     termion::cursor::Goto(x+1, 100+ 46),
     termion::cursor::Goto(x+1, 100+ 47),
     termion::cursor::Goto(x+1, 100+ 48),
     termion::cursor::Goto(x+1, 100+ 49),
     termion::cursor::Goto(x+1, 100+ 50));
} else if direction == 1 {
    println!("
    {}|                                  |                                      |
    {}|                                                                         |
    {}|                                                                         |
    {} |                                  |                                      |
    {} |                                  |                                      |
    {} |                                                                         |
    {}  |                                                                         |
    {}  |                                  |                                      |
    {}   |                                  |                                      |
    {}    |                                                                         |
    {}    |                                                                         |
    {}    |                                  |                                      |
    {}     |                                  |                                      |
    {}     |                                                                         |
    {}     |                                                                         |
    {}       |                                  |                                      |
    {}       |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |
    {}      |                                                                         |
    {}      |                                                                         |
    {}      |                                  |                                      |
    {}      |                                  |                                      |", 
     
     termion::cursor::Goto(x+1, 100+ 1),
     termion::cursor::Goto(x+1, 100+ 2),
     termion::cursor::Goto(x+1, 100+ 3),
     termion::cursor::Goto(x+1, 100+ 4),
     termion::cursor::Goto(x+1, 100+ 5),
     termion::cursor::Goto(x+1, 100+ 6),
     termion::cursor::Goto(x+1, 100+ 7),
     termion::cursor::Goto(x+1, 100+ 8),
     termion::cursor::Goto(x+1, 100+ 9),
     termion::cursor::Goto(x+1, 100+ 10),
     termion::cursor::Goto(x+1, 100+ 11),
     termion::cursor::Goto(x+1, 100+ 12),
     termion::cursor::Goto(x+1, 100+ 13),
     termion::cursor::Goto(x+1, 100+ 14),
     termion::cursor::Goto(x+1, 100+ 15),
     termion::cursor::Goto(x+1, 100+ 16),
     termion::cursor::Goto(x+1, 100+ 17),
     termion::cursor::Goto(x+1, 100+ 18),
     termion::cursor::Goto(x+1, 100+ 19),
     termion::cursor::Goto(x+1, 100+ 20),
     termion::cursor::Goto(x+1, 100+ 21),
     termion::cursor::Goto(x+1, 100+ 22),
     termion::cursor::Goto(x+1, 100+ 23),
     termion::cursor::Goto(x+1, 100+ 24),
     termion::cursor::Goto(x+1, 100+ 25),
     termion::cursor::Goto(x+1, 100+ 26),
     termion::cursor::Goto(x+1, 100+ 27),
     termion::cursor::Goto(x+1, 100+ 28),
     termion::cursor::Goto(x+1, 100+ 29),
     termion::cursor::Goto(x+1, 100+ 30),
     termion::cursor::Goto(x+1, 100+ 31),
     termion::cursor::Goto(x+1, 100+ 32),
     termion::cursor::Goto(x+1, 100+ 33),
     termion::cursor::Goto(x+1, 100+ 34),
     termion::cursor::Goto(x+1, 100+ 35),
     termion::cursor::Goto(x+1, 100+ 36),
     termion::cursor::Goto(x+1, 100+ 37),
     termion::cursor::Goto(x+1, 100+ 38),
     termion::cursor::Goto(x+1, 100+ 39),
     termion::cursor::Goto(x+1, 100+ 40),
     termion::cursor::Goto(x+1, 100+ 41),
     termion::cursor::Goto(x+1, 100+ 42),
     termion::cursor::Goto(x+1, 100+ 43),
     termion::cursor::Goto(x+1, 100+ 44),
     termion::cursor::Goto(x+1, 100+ 45),
     termion::cursor::Goto(x+1, 100+ 46),
     termion::cursor::Goto(x+1, 100+ 47),
     termion::cursor::Goto(x+1, 100+ 48),
     termion::cursor::Goto(x+1, 100+ 49),
     termion::cursor::Goto(x+1, 100+ 50));
}
}