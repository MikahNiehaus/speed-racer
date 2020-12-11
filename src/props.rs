
pub fn startPage() {

    println!("{}░██████╗██████╗░███████╗███████╗██████╗░  ██████╗░░█████╗░░█████╗░███████╗██████╗░", termion::cursor::Goto(1, 1));
    println!("{}██╔════╝██╔══██╗██╔════╝██╔════╝██╔══██╗  ██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗", termion::cursor::Goto(1, 2));
    println!("{}╚█████╗░██████╔╝█████╗░░█████╗░░██║░░██║  ██████╔╝███████║██║░░╚═╝█████╗░░██████╔╝", termion::cursor::Goto(1, 2));
    println!("{}░╚═══██╗██╔═══╝░██╔══╝░░██╔══╝░░██║░░██║  ██╔══██╗██╔══██║██║░░██╗██╔══╝░░██╔══██╗", termion::cursor::Goto(1, 3));
    println!("{}██████╔╝██║░░░░░███████╗███████╗██████╔╝  ██║░░██║██║░░██║╚█████╔╝███████╗██║░░██║", termion::cursor::Goto(1, 4));
    println!("{}╚═════╝░╚═╝░░░░░╚══════╝╚══════╝╚═════╝░  ╚═╝░░╚═╝╚═╝░░╚═╝░╚════╝░╚══════╝╚═╝░░╚═╝", termion::cursor::Goto(1, 5));
    println!("{}press \"q\" to exit at any time or press \"s\" to start, full screen recomended, use a d as controles", termion::cursor::Goto(1, 8), );

 
}

pub fn car ( x: u16){
    const y: u16 = 30;
    println!("
    {}─███████▀
	{}██───████▀
	{}█─▀█▀─████
	{}█─▄█▄─████
	{}██───█████
	{}─█████████
	{}───███████
	{}───███████▄▀
	{}───███████▓▓▄▀
	{}───███████▓▓▓▓█
	{}───███████▓▓▓▓██
	{}───███████▓▓▓▓██
	{}───███████▓▓▓▓██
	{}───█▀▀▀▀▀███████
	{}───███████▓▓▓▓██
	{}─█████████▓▓▓▓██
	{}██───███████████
	{}█─▀█▀─█████████
	{}█─▄█▄─████████
	{}██───████████
	{}─███████████
	{}───████████
	{}───███████
	{}───█▒▒███"
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
    termion::cursor::Goto(x+1, y + 19),
    termion::cursor::Goto(x+1, y + 20),
    termion::cursor::Goto(x+1, y + 21),
    termion::cursor::Goto(x+1, y + 22),
    termion::cursor::Goto(x+1, y + 23),
    termion::cursor::Goto(x+1, y + 24));
    
}