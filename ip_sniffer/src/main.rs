use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream}; //IpAddr is enum 
use std::str::FromStr; //arguments ko we as a user want to see as string 
use std::process; 
use std::sync::mpsc::{Sender, channel};
use std::thread; 

 const MAX: u16 = 65535;
 struct Arguments {
    _flag: String, 
    ipaddr: IpAddr,
    threads: u16,
 }

 impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str>{ //'static str is to give string error if we are not able to return arguments
    if args.len() < 2{
        return Err("not enough arguments");
    } else if args.len() > 4 {
        return Err("too many arguments");
    } 
    let f = args[1].clone();
    if let Ok(ipaddr) = IpAddr::from_str(&f) {
        return Ok(Arguments {_flag: String::from(""), ipaddr, threads: 4});
    } else {
        let _flag = args[1].clone();
        if _flag.contains("-h") || _flag.contains("-help") && args.len()  == 2{
            println!("Usage: -j to select how many threads you want 
            \r\n       -h or -help to show this help message");
            return Err("help");
        } else if _flag.contains("-h") || _flag.contains("-help") {
            return Err("too many arguments");
        } else if _flag.contains("-j") {
            let ipaddr = match IpAddr::from_str(&args[3]){
                Ok(s) => s,
                Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
            };
            let threads = match args[2].parse::<u16>(){
                Ok(s) => s,
                Err(_)  => return Err("failed to parse thread number")
            };
            return Ok(Arguments{threads, _flag, ipaddr});
        } else {
            return Err("invalid syntax");
        }
    }
    }
 }


 fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port +1; 
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if {MAX - port} <= num_threads {
            break;
        }
        port += num_threads;
    }
 }

fn main() {
    let args: Vec<String> = env::args().collect(); //takeing all 
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let _addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out = vec![]; 
    drop(tx); 
    for p in rx {
        out.push(p); 
    }

    println!("");
    out.sort(); 
    for v in out {
        println!("{} is open", v);
    }
}
// ip_sniffer.exe -h //open help 
// ip_sniffer.exe -j 100 192.168.1.1 
// ip_sniffer.exe 192.168.1.1 //calling tool on this ip address
