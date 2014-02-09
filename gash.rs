//
// gash.rs
//
// Starting code for PS2
// Running on Rust 0.9
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

/*
possible feaure 6: up arrow gives last command
*/

extern mod extra;

use std::{io, run, os, path, libc};
use std::io::buffered::BufferedReader;
use std::io::stdin;
use std::io::stdio;
use std::io::buffered::BufferedWriter;
use std::io::File;
use std::io::stdio::StdWriter; 
use extra::getopts;




struct  Shell {
    cmd_prompt: ~str,

    //array of past commands
    history: ~[~str]
}



impl Shell {
    fn new(prompt_str: &str) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),

            //past commands initialized to empty
            history: ~[]
        }
    }
   
  

    fn run(&mut self) {
        let mut stdin = BufferedReader::new(stdin());
        
        loop {
            print!({},self.cmd_prompt);
            io::stdio::flush();
            
            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim().to_owned();
            let program = cmd_line.splitn(' ', 1).nth(0).expect("no program");
            
            match program {
                ""      =>  { continue; }
                "exit"  =>  { return; }
                _       =>  { self.run_cmdline(cmd_line); }
            }
        }
    }
    
    fn run_cmdline(&mut self, cmd_line: &str) {
    	self.history.push(cmd_line.to_owned()); 

        let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
    


        if argv.len() > 0 {
            let program: ~str = argv.remove(0);
            self.run_cmd(program, argv);
        }
    }
    
    fn run_cmd(&mut self, program: &str, argv: &[~str]) {
        if self.cmd_exists(program) {

			
			let mut argv = argv.to_owned(); 
        	//self.history.push(program.to_owned()); 
            

            //TODO: put this code in cd function and call it here
            if program == "cd" {

            	let mut pathName = "";
            	// ^ is this ok ?? - not sure how to get home directory so just 
            	//doing root if no input for directory



            	if argv.len() != 0 {
            	 pathName = argv[0].trim(); 
            	}
            	

           
    			//creat path object and cd to it

                let path = Path::new(pathName);

                if path.exists(){
            	os::change_dir(&path); 
            	}
            	else{
            		println!({},"invalid path!")
            	}
            	
            }

            //iterate over history array and print contents
            else if program == "history"{
            	for x in self.history.iter() {
    			println!("{}", *x);
				}

            }
            else{

            	if argv.len() > 0 {

            	let mut i = 0; 

	    		 while (i < argv.len()) {
			        if (argv[i] == ~"<") {
			            argv.remove(i);
			            //out_fd = get_fd(argv.remove(i), "w");
			            let input = argv.remove(i);
			            self.redirect_input(program, argv, input);
			        } else if (argv[i] == ~">") {
			            argv.remove(i);
			            //in_fd = get_fd(argv.remove(i), "r");
			            let output = argv.remove(i);
			            self.redirect_output(program, argv, output);
			        }
			        i += 1;
			    }

            	//if argv[0] == ~"<"{
            	//	self.redirect_input(program, argv);
            	//}
            	//else if argv[0] == ~">"{
            	//	self.redirect_output(program, argv);
            	//}

            	}
            	else{
            		run::process_status(program, argv);
            	}

            }
        }
        else {
            println!("{:s}: command not found", program);
        }	
    }
    
    fn cmd_exists(&mut self, cmd_path: &str) -> bool {
    	if cmd_path == "history" {return true; }

        let ret = run::process_output("which", [cmd_path.to_owned()]);
        return ret.expect("exit code error.").status.success();
    }

    fn redirect_input(&mut self, program: &str, argv: &[~str], file: &str ){
    	

			let in_fd = self.get_fd(file, "r");


			let mut process = run::Process::new(program, argv, run::ProcessOptions {
                                 env: None,
                                 dir: None,
                                 in_fd: Some(in_fd),
                                 out_fd: None,
                                 err_fd: None
                                     }).unwrap();
     

			process.finish(); 
    }

    fn redirect_output(&mut self, program: &str, argv: &[~str], file: &str ){
    	
	


			let out_fd = self.get_fd(file, "w");


			let mut process = run::Process::new(program, argv, run::ProcessOptions {
                                 env: None,
                                 dir: None,
                                 in_fd: None,
                                 out_fd: Some(out_fd),
                                 err_fd: None
                                     }).unwrap();
     

			process.finish(); 


    }

      fn get_fd(&mut self, fpath: &str, mode: &str) -> libc::c_int {

	    unsafe{
	        let fpathbuf = fpath.to_c_str().unwrap();
	        let modebuf = mode.to_c_str().unwrap();
	        return libc::fileno(libc::fopen(fpathbuf, modebuf));
	    }
	}

    
}


fn get_cmdline_from_args() -> Option<~str> {
    /* Begin processing program arguments and initiate the parameters. */
    let args = os::args();
    
    let opts = ~[
        getopts::optopt("c")
    ];
    
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    
    if matches.opt_present("c") {
        let cmd_str = match matches.opt_str("c") {
                                                Some(cmd_str) => {cmd_str.to_owned()}, 
                                                None => {~""}
                                              };
        return Some(cmd_str);
    } else {
        return None;
    }
}

fn main() {
	 

    let opt_cmd_line = get_cmdline_from_args();
    
    match opt_cmd_line {
        Some(cmd_line) => Shell::new("").run_cmdline(cmd_line),
        None           => Shell::new("gash > ").run()
    }
}



