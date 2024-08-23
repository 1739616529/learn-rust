
fn main() {
    
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    println!("this is windows x64 platform");
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    println!("this is windows x86 platform");
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    println!("this is windows aarch64 platform");

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    println!("this is macos x64 platform");
    #[cfg(all(target_os = "macos", target_arch = "x86"))]
    println!("this is macos x86 platform");
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    println!("this is macos aarch64 platform");

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    println!("this is linux x64 platform");
    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    println!("this is linux x86 platform");
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    println!("this is linux aarch64 platform");


}