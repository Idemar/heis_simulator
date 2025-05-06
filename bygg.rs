use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(["src/motor1.c", "-c", "-fPIC", "-o"])
        .arg(format!("{}/motor1.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(["crus", "libmotor1.a", "motor1.o"])
        .current_dir(Path::new(&out_dir))
        .status()
        .unwrap();
    Command::new("gcc")
        .args(["src/motor2.c", "-c", "-fPIC", "-o"])
        .arg(format!("{}/motor2.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(["crus", "libmotor2.a", "motor2.o"])
        .current_dir(Path::new(&out_dir))
        .status()
        .unwrap();
    Command::new("gcc")
        .args(["src/motor3.c", "-c", "-fPIC", "-o"])
        .arg(format!("{}/motor3.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(["crus", "libmotor3.a", "motor3.o"])
        .current_dir(Path::new(&out_dir))
        .status()
        .unwrap();

    Command::new("gcc")
        .args(["src/heis1.c", "-c", "-fPIC", "-o"])
        .arg(format!("{}/heis1.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(["crus", "libheis1.a", "heis1.o"])
        .current_dir(Path::new(&out_dir))
        .status()
        .unwrap();

    Command::new("gcc")
        .args(["src/heis2.c", "-c", "-fPIC", "-o"])
        .arg(format!("{}/heis2.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(["crus", "libheis2.a", "heis2.o"])
        .current_dir(Path::new(&out_dir))
        .status()
        .unwrap();

    Command::new("gcc")
        .args(["src/heis3.c", "-c", "-fPIC", "-o"])
        .arg(format!("{}/heis3.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(["crus", "libheis3.a", "heis3.o"])
        .current_dir(Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=motor1");
    println!("cargo:rustc-link-lib=static=motor2");
    println!("cargo:rustc-link-lib=static=motor3");
    println!("cargo:rustc-link-lib=static=heis1");
    println!("cargo:rustc-link-lib=static=heis2");
    println!("cargo:rustc-link-lib=static=heis3");
}
