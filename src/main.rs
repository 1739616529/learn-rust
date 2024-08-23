
fn main() {
    
    #[cfg(windows)]
    println!("this is windows platform");

    #[cfg(target_os = "macos")]
    println!("this is macos platform");

    
    #[cfg(target_os = "linux")]
    println!("this is macos platform");

}