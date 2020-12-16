use rand::Rng;

pub fn startPage() {

    println!("{}░██████╗██████╗░███████╗███████╗██████╗░  ██████╗░░█████╗░░█████╗░███████╗██████╗░", termion::cursor::Goto(1, 1));
    println!("{}██╔════╝██╔══██╗██╔════╝██╔════╝██╔══██╗  ██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗", termion::cursor::Goto(1, 2));
    println!("{}╚█████╗░██████╔╝█████╗░░█████╗░░██║░░██║  ██████╔╝███████║██║░░╚═╝█████╗░░██████╔╝", termion::cursor::Goto(1, 2));
    println!("{}░╚═══██╗██╔═══╝░██╔══╝░░██╔══╝░░██║░░██║  ██╔══██╗██╔══██║██║░░██╗██╔══╝░░██╔══██╗", termion::cursor::Goto(1, 3));
    println!("{}██████╔╝██║░░░░░███████╗███████╗██████╔╝  ██║░░██║██║░░██║╚█████╔╝███████╗██║░░██║", termion::cursor::Goto(1, 4));
    println!("{}╚═════╝░╚═╝░░░░░╚══════╝╚══════╝╚═════╝░  ╚═╝░░╚═╝╚═╝░░╚═╝░╚════╝░╚══════╝╚═╝░░╚═╝", termion::cursor::Goto(1, 5));
    println!("{}press \"q\" to exit at any time or press \"s\" to start, full screen needed, use left, right, and space key as controles", termion::cursor::Goto(1, 8), );

 
}

pub fn car ( x: u16){
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
    termion::cursor::Goto(x+1,40 +1),
    termion::cursor::Goto(x+1,40 +2),
    termion::cursor::Goto(x+1,40 +3),
    termion::cursor::Goto(x+1,40 +4),
    termion::cursor::Goto(x+1,40 +5),
    termion::cursor::Goto(x+1,40 +6),
    termion::cursor::Goto(x+1,40 +7),
    termion::cursor::Goto(x+1,40 +8),
    termion::cursor::Goto(x+1,40 +9),
    termion::cursor::Goto(x+1,40 +10),
    termion::cursor::Goto(x+1,40 +11),
    termion::cursor::Goto(x+1,40 +12),
    termion::cursor::Goto(x+1,40 +13),
    termion::cursor::Goto(x+1,40 +14),
    termion::cursor::Goto(x+1,40 +15),
    termion::cursor::Goto(x+1,40 +16),
    termion::cursor::Goto(x+1,40 +17),
    termion::cursor::Goto(x+1,40 +18),
    termion::cursor::Goto(x+1,40 +19));
    
}

pub fn enemy ( x: u16, tp: String){


    if (tp == "CAR"){
   
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
    termion::cursor::Goto(x+1,30 +1),
    termion::cursor::Goto(x+1,30 +2),
    termion::cursor::Goto(x+1,30 +3),
    termion::cursor::Goto(x+1,30 +4),
    termion::cursor::Goto(x+1,30 +5),
    termion::cursor::Goto(x+1,30 +6),
    termion::cursor::Goto(x+1,30 +7),
    termion::cursor::Goto(x+1,30 +8),
    termion::cursor::Goto(x+1,30 +9),
    termion::cursor::Goto(x+1,30 +10),
    termion::cursor::Goto(x+1,30 +11),
    termion::cursor::Goto(x+1,30 +12),
    termion::cursor::Goto(x+1,30 +13),
    termion::cursor::Goto(x+1,30 +14),
    termion::cursor::Goto(x+1,30 +15),
    termion::cursor::Goto(x+1,30 +16),
    termion::cursor::Goto(x+1,30 +17),
    termion::cursor::Goto(x+1,30 +18),
    termion::cursor::Goto(x+1,30 +19));
    }

    if (tp == "TIRE"){
        println!("
        {}  ███
        {}██   ██
        {}█ ▄█▄ █
        {}█ ▀█▀ █
        {}██   ██
        {}  ███
        "
        , 
        termion::cursor::Goto(x+1,30 +1),
        termion::cursor::Goto(x+1,30 +2),
        termion::cursor::Goto(x+1,30 +3),
        termion::cursor::Goto(x+1,30 +4),
        termion::cursor::Goto(x+1,30 +5),
        termion::cursor::Goto(x+1,30 +6),
        );
        }
    if (tp == "CAT"){
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
        termion::cursor::Goto(x+1,30 +1),
        termion::cursor::Goto(x+1,30 +2),
        termion::cursor::Goto(x+1,30 +3),
        termion::cursor::Goto(x+1,30 +4),
        termion::cursor::Goto(x+1,30 +5),
        termion::cursor::Goto(x+1,30 +6),
        termion::cursor::Goto(x+1,30 +7),
        termion::cursor::Goto(x+1,30 +8),
        termion::cursor::Goto(x+1,30 +9),
        termion::cursor::Goto(x+1,30 +10),
        termion::cursor::Goto(x+1,30 +11),
        termion::cursor::Goto(x+1,30 +12),
        termion::cursor::Goto(x+1,30 +13),
        termion::cursor::Goto(x+1,30 +14),
        termion::cursor::Goto(x+1,30 +15),
        termion::cursor::Goto(x+1,30 +16),
        termion::cursor::Goto(x+1,30 +17),
        termion::cursor::Goto(x+1,30 +18),
        termion::cursor::Goto(x+1,30 +19),
        termion::cursor::Goto(x+1,30 +20),
        termion::cursor::Goto(x+1,30 +21),
    );
        }
}

pub fn crashSign (){
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
    termion::cursor::Goto(30+1,20 +1),
    termion::cursor::Goto(30+1,20 +2),
    termion::cursor::Goto(30+1,20 +3),
    termion::cursor::Goto(30+1,20 +4),
    termion::cursor::Goto(30+1, 20 +5),
    termion::cursor::Goto(30+1, 20 +6),
    termion::cursor::Goto(30+1, 20 +7),
    termion::cursor::Goto(30+1, 20 +8),
    termion::cursor::Goto(30+1, 20 +9),
    termion::cursor::Goto(30+1, 20 +10),
    termion::cursor::Goto(30+1, 20 +11),
    termion::cursor::Goto(30+1, 20 +12),
    termion::cursor::Goto(30+1, 20 +13),
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