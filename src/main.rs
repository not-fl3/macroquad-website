use std::process::Command;

const MACROQUAD_PATH: &str = "/home/zorax/testing/macroquad";
const RUST_EXTENSION: &str = ".rs";

fn take_screenshot(name: &str) {
    Command::new("cargo")
        .args(&["run", "--example", name])
        .env("RUSTFLAGS", "--cfg one_screenshot")
        .current_dir(MACROQUAD_PATH)
        .output()
        .unwrap();
    std::fs::copy(
        format!("{}/screenshot.png", MACROQUAD_PATH),
        format!("docs/images/{}.png", name),
    )
    .unwrap();
}

fn build_wasm(name: &str) {
    Command::new("cargo")
        .args(&[
            "build",
            "--example",
            name,
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .current_dir(MACROQUAD_PATH)
        .output()
        .unwrap();
    let new_name = format!("docs/wasms/{}.wasm", name);
    std::fs::copy(
        format!(
            "{}/target/wasm32-unknown-unknown/release/examples/{}.wasm",
            MACROQUAD_PATH, name
        ),
        &new_name,
    )
    .unwrap();
    Command::new("wasm-strip")
        .args(&[new_name])
        .output()
        .unwrap();
}

fn get_examples() -> Vec<String> {
    let mut result = std::fs::read_dir(format!("{}/examples/", MACROQUAD_PATH))
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .filter(|name| name.ends_with(RUST_EXTENSION))
        .map(|mut name| {
            name.truncate(name.len() - RUST_EXTENSION.len());
            name
        })
        .collect::<Vec<_>>();
    result.sort();
    result
}

fn create_html(examples: &[String]) {
    const INDEX: &str = include_str!("index.html");
    const INDEX_ENTRY: &str = include_str!("index_entry.html");
    const ENTRY_PAGE: &str = include_str!("entry_page.html");

    let entries = examples
        .iter()
        .map(|name| INDEX_ENTRY.replace("%%NAME%%", name))
        .collect::<Vec<_>>()
        .join("\n");

    let index = INDEX.replace("%%ENTRIES%%", &entries);
    std::fs::write("docs/index.html", index).unwrap();

    for example in examples {
        let page = ENTRY_PAGE.replace("%%NAME%%", example);
        std::fs::write(format!("docs/{}.html", example), page).unwrap();
    }
}

fn copy_other_files() {
    let other_files = std::fs::read_dir(format!("{}/examples/", MACROQUAD_PATH))
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .filter(|name| !name.ends_with(RUST_EXTENSION))
        .map(|name| format!("{}/examples/{}", MACROQUAD_PATH, name))
        .collect::<Vec<_>>();

    fs_extra::copy_items(
        &other_files,
        "docs/examples",
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            ..fs_extra::dir::CopyOptions::new()
        },
    )
    .unwrap();
}

fn copy_assets() {
    let assets = std::fs::read_dir("assets")
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .map(|name| format!("assets/{}", name))
        .collect::<Vec<_>>();

    fs_extra::copy_items(
        &assets,
        "docs",
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            ..fs_extra::dir::CopyOptions::new()
        },
    )
    .unwrap();
}

fn copy_overrided_images() {
    let assets = std::fs::read_dir("images")
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .map(|name| format!("images/{}", name))
        .collect::<Vec<_>>();

    fs_extra::copy_items(
        &assets,
        "docs/images",
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            ..fs_extra::dir::CopyOptions::new()
        },
    )
    .unwrap();
}

fn main() {
    drop(std::fs::create_dir("docs"));
    drop(std::fs::create_dir("docs/examples"));
    drop(std::fs::create_dir("docs/images"));
    drop(std::fs::create_dir("docs/wasms"));

    let examples = get_examples();

    println!("[[taking screenshots]]");
    for example in &examples {
        println!("{}", example);
        take_screenshot(&example);
    }
    println!("---");

    println!("[[buildng wasms]]");
    for example in &examples {
        println!("{}", example);
        build_wasm(&example);
    }
    println!("---");

    println!("[[create html]]");
    create_html(&examples);

    println!("[[copy assets]]");
    copy_assets();

    println!("[[copy overrided imagesu]]");
    copy_overrided_images();

    println!("[[copy other files]]");
    copy_other_files();
}
