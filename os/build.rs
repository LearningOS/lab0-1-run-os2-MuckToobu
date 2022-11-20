use std::fs::{read_dir, File};
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "../user/build/bin/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps = read_dir("../user/build/bin/")
        .unwrap().map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        }).collect::<Vec<_>>();
    println!("{:?}", apps);
    apps.sort();
    println!("{:?}", apps);
    writeln!(
        f,
        "\n\t\
         .align 3\n\t\
         .section .data\n\t\
         .global _num_app\n\
        _num_app:\n\t\
         .quad {}",
        apps.len()
    )?;
    for i in 0..apps.len() {
        writeln!(f, "\t.quad app_{}_start", i)?;
    }
    writeln!(f, "\t.quad app_{}_end", apps.len() - 1)?;
    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            "\n\t\
             .section .data\n\t\
             .global app_{0}_start\n\t\
             .global app_{0}_end\n\
            app_{0}_start:\n\t\
             .incbin \"{2}{1}.bin\"\n\
            app_{0}_end:",
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}