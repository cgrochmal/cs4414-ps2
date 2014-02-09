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
use std::os::pipe
; 




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


            print!("{:s}",self.cmd_prompt);

           

            io::stdio::flush();
            
            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim().to_owned();
            let program = cmd_line.splitn(' ', 1).nth(0).expect("no program");

            //check for pipes

	        let pipeLine: ~[~str] =
	        	cmd_line.split
	        	('|').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();

	        let pipes = self.handle_pipes(pipeLine); 
            
            if pipes == false{
	            match program {
	                ""      =>  { continue; }
	                "exit"  =>  { return; }
	                _       =>  { self.run_cmdline(cmd_line, 0, 1, 2); }
	            }
        	}
        }
    }
    
    fn run_cmdline(&mut self, cmd_line: &str, in_fd: libc::c_int, out_fd: libc::c_int, err_fd: libc::c_int) {
    	self.history.push(cmd_line.to_owned()); 

        let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();


        if argv.len() > 0 {
            let program: ~str = argv.remove(0);
            self.run_cmd(program, argv, in_fd, out_fd, err_fd);
        }
    }
    
    fn run_cmd(&mut self, program: &str, argv: &[~str], mut in_fd: libc::c_int, mut out_fd: libc::c_int, mut err_fd: libc::c_int) {
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
            		println!("{:s}","invalid path!")
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
			        if (argv[i] == ~">") {
			            argv.remove(i);
			            //out_fd = get_fd(argv.remove(i), "w");
			            let output = argv.remove(i);
			            //self.redirect_output(program, argv, output);
			            in_fd = 0;
			            out_fd = self.get_fd(output, "w");
			            err_fd = 2; 
			            //self.redirect(program, argv, input, "w");
			        } else if (argv[i] == ~"<") {
			            argv.remove(i);
			            //in_fd = get_fd(argv.remove(i), "r");
			            let input = argv.remove(i);
			      
			            //self.redirect_input(program, argv, input);
			            in_fd =  self.get_fd(input, "r");
			            out_fd = 1; 
			            err_fd = 2; 
			            //self.redirect(program, argv, output, "r");
			        }
			        i += 1;
			    }

            	//if argv[0] == ~"<"{
            	//	self.redirect_input(program, argv);
            	//}
            	//else if argv[0] == ~">"{
            	//	self.redirect_output(program, argv);
            	//}

            	self.run_process(program, argv, in_fd, out_fd, err_fd);

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
/*
    fn redirect(&mut self, program: &str, argv: &[~str], file: &str, kind: &str){
    	let mut out_fd = 0;
    	let mut in_fd = 1;
    	let err_fd = 2;

    	if kind = "r"{
    		in_fd = self.get_fd(file, "r");
    	}
    	else if kind = "w"{
    		out_fd = self.get_fd(file, "w");

    	}

    }
    */ 

    fn redirect_input(&mut self, program: &str, argv: &[~str], file: &str ){
    	

			let in_fd = self.get_fd(file, "r");


			/*
			let mut process = run::Process::new(program, argv, run::ProcessOptions {
                                 env: None,
                                 dir: None,
                                 in_fd: Some(in_fd),
                                 out_fd: Some(1),
                                 err_fd: Some(2)
                                     }).unwrap();
     

			process.finish(); 
			*/

			self.run_process(program, argv, in_fd, 1, 2); 
    }

    fn redirect_output(&mut self, program: &str, argv: &[~str], file: &str ){
    	



			let out_fd = self.get_fd(file, "w");


			/*
			let mut process = run::Process::new(program, argv, run::ProcessOptions {
                                 env: None,
                                 dir: None,
                                 in_fd: Some(0),
                                 out_fd: Some(out_fd),
                                 err_fd: Some(2)
                                     }).unwrap();
     

			process.finish(); 
			*/

			self.run_process(program, argv, 0, out_fd, 2);


    }

    fn run_process(&mut self, program: &str, argv: &[~str], in_fd: libc::c_int, out_fd: libc::c_int, err_fd: libc::c_int){

    		let mut process = run::Process::new(program, argv, run::ProcessOptions {
                                 env: None,
                                 dir: None,
                                 in_fd: Some(in_fd),
                                 out_fd: Some(out_fd),
                                 err_fd: Some(err_fd)
                                     }).unwrap();
     

			process.finish(); 

			 if in_fd != 0 {os::close(in_fd);}
		     if out_fd != 1 {os::close(out_fd);}
		     if err_fd != 2 {os::close(err_fd);}

    }

    fn handle_pipes(&mut self, progs: ~[~str]) -> bool {

    		 //let progs: ~[~str] =
        //cmd_line.split_iter('|').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
    
    let mut pipes: ~[os::Pipe] = ~[];
    
    // create pipes
    if (progs.len() > 1) {
        for _ in range(0, progs.len()-1) {
            pipes.push(os::pipe());
        }
    }
        
    if progs.len() == 1 {
        //if bg_flag == false { handle_cmd(progs[0], 0, 1, 2); }
        //else {task::spawn_sched(task::SingleThreaded, ||{handle_cmd(progs[0], 0, 1, 2)});}
        return false; 
    } else {
        for i in range(0, progs.len()) {
            let prog = progs[i].to_owned();
            
            if i == 0 {
                let pipe_i = pipes[i];
              //  task::spawn_sched(task::SingleThreaded, ||{handle_cmd(prog, 0, pipe_i.out, 2)});
             
            } else if i == progs.len() - 1 {
                let pipe_i_1 = pipes[i-1];
            //  if bg_flag == true {
              //      task::spawn_sched(task::SingleThreaded, ||{handle_cmd(prog, pipe_i_1.input, 1, 2)});
             //   } else {
                    self.run_cmdline(prog, pipe_i_1.input, 1, 2);
           //     }
                

            } else {
                let pipe_i = pipes[i];
                let pipe_i_1 = pipes[i-1];
               // task::spawn_sched(task::SingleThreaded, ||{handle_cmd(prog, pipe_i_1.input, pipe_i.out, 2)});
              

            }
        }
    
        return true; 


   	 }

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
        Some(cmd_line) => Shell::new("").run_cmdline(cmd_line, 0, 1, 2),
        None           => Shell::new("gash > ").run()
    }
}
