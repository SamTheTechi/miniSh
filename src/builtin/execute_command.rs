use std::ffi::CString;

pub fn run(command: &str) {
    let args: Vec<&str> = command.split_whitespace().collect();
    unsafe {
        let pid = libc::fork();

        match pid {
            -1 => {
                eprint!("minish: Eroe while forking");
            }
            0 => {
                let c_command = CString::new(args[0]).unwrap();
                let c_args: Vec<CString> = args
                    .iter()
                    .map(|&args| CString::new(args).unwrap())
                    .collect();
                let mut c_args_ptrs: Vec<*const libc::c_char> =
                    c_args.iter().map(|c_str| c_str.as_ptr()).collect();
                c_args_ptrs.push(std::ptr::null());

                let err = libc::execvp(c_command.as_ptr(), c_args_ptrs.as_ptr());

                if err == -1 {
                    println!("minish: command not found: {:?}", c_command.as_c_str());
                    libc::exit(1);
                }
            }
            _ => {
                let mut status: libc::c_int = 0;
                let err = libc::waitpid(pid, &mut status, 0);
                if err == -1 {
                    eprint!("minish: error while waiting");
                }
            }
        }
    }
}
