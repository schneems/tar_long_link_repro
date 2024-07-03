use std::path::PathBuf;

fn main() {
    let output = source_dir().join("tmp").join("rust-extracted");

    // From https://repo1.maven.org/maven2/org/jruby/jruby-dist/9.4.8.0/jruby-dist-9.4.8.0-bin.tar.gz
    let tar_path = source_dir()
        .join("fixtures")
        .join("jruby-dist-9.4.8.0-bin.tar.gz");

    if output.exists() {
        std::fs::remove_dir_all(&output).unwrap();
    }

    std::fs::create_dir_all(&output).unwrap();

    let decoder = flate2::read::GzDecoder::new(std::fs::File::open(tar_path).unwrap());
    tar::Archive::new(decoder).unpack(&output).unwrap();

    let delegates_path = output
        .join("jruby-9.4.8.0/lib/ruby/stdlib/bundler/vendor/molinillo/lib/molinillo/delegates");
    let mut names = Vec::new();
    for entry in std::fs::read_dir(delegates_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        names.push(name.to_owned());
    }

    let expected = "specification_provider.rb";

    assert!(
        names.iter().any(|name| name == expected),
        "expected {names:?} to include {expected:?} but it did not"
    );
}

pub fn source_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .expect("Canonicalize source dir")
}
