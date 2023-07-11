use libloading;

fn main() -> Result< (), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("C:\\Users\\jakegallow\\dev\\dll_loading_ex\\target\\debug\\dll_loading_ex.dll")?;
        let func1: libloading::Symbol<unsafe extern fn() -> usize> = lib.get(b"no_args")?;
        let func2: libloading::Symbol<unsafe extern fn(usize, usize) -> usize> = lib.get(b"add")?;
        println!("func output = {:?}", func1());
        println!("func output = {:?}", func2(1, 1));
    }
    Ok(())
}
