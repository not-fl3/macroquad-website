use std::process::Command;

const MACROQUAD_PATH: &str = "/home/zorax/testing/macroquad";
const RUST_EXTENSION: &str = ".rs";

struct ToCompile {
    path: String,
    example_name: String,
    prefix: String,
}

impl ToCompile {
    fn compile_wasm(&self) {
        Command::new("cargo")
            .args(&[
                "build",
                "--example",
                &self.example_name,
                "--release",
                "--target",
                "wasm32-unknown-unknown",
            ])
            .current_dir(&self.path)
            .output()
            .unwrap();
        let new_name = format!("docs/wasms/{}{}.wasm", self.prefix, self.example_name);
        std::fs::copy(
            format!(
                "{}/target/wasm32-unknown-unknown/release/examples/{}.wasm",
                self.path, self.example_name
            ),
            &new_name,
        )
        .unwrap();
        Command::new("wasm-strip")
            .args(&[new_name])
            .output()
            .unwrap();
    }

    fn take_screenshot(&self) {
        Command::new("cargo")
            .args(&["run", "--example", &self.example_name])
            .env("RUSTFLAGS", "--cfg one_screenshot")
            .current_dir(&self.path)
            .output()
            .unwrap();
        std::fs::copy(
            format!("{}/screenshot.png", self.path),
            format!("docs/images/{}{}.png", self.prefix, self.example_name),
        )
        .unwrap();
    }

    fn to_html(&self, title: &str) -> (String, String) {
        const ENTRY_PAGE: &str = include_str!("entry_page.html");
        let full_name = format!("{}{}", self.prefix, self.example_name);
        let html = ENTRY_PAGE.replace("%%TITLE%%", title).replace("%%NAME%%", &full_name);
        (full_name, html)
    }
}

struct Entry {
    link: String,
    image: String,
    source: String,
    title: String,
    compile: Option<ToCompile>,
}

impl Entry {
    fn from_macroquad(name: &str) -> Self {
        Entry {
            link: format!("{}.html", name),
            image: format!("images/{}.png", name),
            source: format!("github.com/not-fl3/macroquad/blob/master/examples/{}.rs", name),
            title: name.replace('_', " "),
            compile: Some(ToCompile {
                path: MACROQUAD_PATH.into(),
                example_name: name.into(),
                prefix: "".into(),
            })
        }
    }

    fn from_particles(name: &str) -> Self {
        Entry {
            link: format!("particles_{}.html", name),
            image: format!("images/particles_{}.png", name),
            source: format!("github.com/not-fl3/macroquad/blob/master/particles/examples/{}.rs", name),
            title: name.replace('_', " "),
            compile: Some(ToCompile {
                path: format!("{}/particles", MACROQUAD_PATH),
                example_name: name.into(),
                prefix: "particles_".into(),
            })
        }
    }

    fn to_html(&self) -> String {
        const INDEX_ENTRY: &str = include_str!("index_entry.html");

        INDEX_ENTRY
            .replace("%%LINK%%", &self.link)
            .replace("%%IMAGE%%", &self.image)
            .replace("%%SOURCE%%", &self.source)
            .replace("%%TITLE%%", &self.title)
    }
}

fn get_examples(path: &str) -> Vec<String> {
    let mut result = std::fs::read_dir(format!("{}/examples/", path))
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

fn create_html(entries: Vec<Entry>) {
    const INDEX: &str = include_str!("index.html");

    let index_entries = entries
        .iter()
        .map(|entry| entry.to_html())
        .collect::<Vec<_>>()
        .join("\n");

    let index = INDEX.replace("%%ENTRIES%%", &index_entries);
    std::fs::write("docs/index.html", index).unwrap();

    for entry in &entries {
        if let Some(compile) = &entry.compile {
            let (name, content) = compile.to_html(&entry.title);
            std::fs::write(format!("docs/{}.html", name), content).unwrap();
        }
    }
}

fn copy_other_files(path: &str) {
    let other_files = std::fs::read_dir(format!("{}/examples/", path))
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .filter(|name| !name.ends_with(RUST_EXTENSION))
        .map(|name| format!("{}/examples/{}", path, name))
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

    let examples = get_examples(MACROQUAD_PATH);
    let particles_path = format!("{}/particles", MACROQUAD_PATH);
    let particles_examples = get_examples(&particles_path);

    let entries = {
        let mut result = Vec::new();
        result.extend(examples.iter().map(|name| Entry::from_macroquad(name)));
        result.extend(particles_examples.iter().map(|name| Entry::from_particles(name)));
        result.push(Entry {
            link: "https://fedorgames.itch.io/macroquad-particles".into(),
            image: "images/particles-editor.png".into(),
            source: "github.com/not-fl3/particles-editor".into(),
            title: "Particles editor".into(),
            compile: None,
        });
        result
    };

    println!("[[taking screenshots]]");
    for example in &entries {
        if let Some(compile) = &example.compile {
            println!("{}{}", compile.prefix, compile.example_name);
            compile.take_screenshot();
        }
    }
    println!("---");

    println!("[[buildng wasms]]");
    for example in &entries {
        if let Some(compile) = &example.compile {
            println!("{}{}", compile.prefix, compile.example_name);
            compile.compile_wasm();
        }
    }
    println!("---");

    println!("[[create html]]");
    create_html(entries);

    println!("[[copy assets]]");
    copy_assets();

    println!("[[copy overrided imagesu]]");
    copy_overrided_images();

    println!("[[copy other files]]");
    copy_other_files(MACROQUAD_PATH);
    copy_other_files(&particles_path);
}
