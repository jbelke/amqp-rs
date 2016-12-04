#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(not(feature="clippy"), allow(unknown_lints))]

#[cfg(feature = "rustfmt")]
extern crate rustfmt;

#[cfg(any(feature = "amqp-build-specs", feature = "amqp-pregen-specs"))]
extern crate amqp_specgen as specgen;

fn main() {
    spec0_builder::build();
}

#[cfg(not(any(feature = "amqp-build-specs", feature = "amqp-pregen-specs")))]
mod spec0_builder {
    pub fn build() {
        println!("Skipping build (neither amqp-build-specs nor amqp-pregen-specs specified)");
    }
}

#[cfg(any(feature = "amqp-build-specs", feature = "amqp-pregen-specs"))]
mod spec0_builder {
    extern crate env_logger;

    use std::env;
    use std::fs::{self, File};
    use std::io::{BufWriter, Write};
    use std::path::PathBuf;

    #[cfg(feature = "rustfmt")]
    use rustfmt;

    use specgen::amqp0::{parse, write_generated};
    use specgen::amqp0::parser::Error as ParserError;

    pub fn build() {
        env_logger::init().unwrap();

        let root_out = env::var_os("OUT_DIR").map(PathBuf::from).expect("Error: OUT_DIR not set");
        let cwd = env::current_dir().expect("Unable to get current directory");
        let xml_dir = cwd.join("xml");

        // amqp0
        let out_file = root_out.join("amqp0.rs");
        {
            let file = File::create(out_file.clone()).expect("Failed to open spec.rs");
            let mut writer = BufWriter::new(file);

            writeln!(writer, "/// Generated by build script in amqp-specs").unwrap();
            writeln!(writer, "/// The file mod.pregen.rs is generated with: cargo build --features \"amqp-pregen-specs\" \n").unwrap();
            writeln!(writer, "/// The \"build-xml\" feature may be used to disable the use of the pregenerated file\n").unwrap();

            writeln!(writer, "// EDITORS BEWARE: Your modifications may be overridden\n").unwrap();

            let specs = amqp0_specs().into_iter()
                .map(|(name, filename)| {
                    let path = xml_dir.join(filename.to_string());
                    println!("cargo:rerun-if-changed={}", path.display());
                    let spec = try!(parse(path));
                    Ok((name, spec))
                })
                .collect::<Result<Vec<_>, ParserError>>()
                .unwrap();

            for &(name, ref spec) in &specs {
                {
                    let v = spec.version();
                    writeln!(writer, "pub fn {}0_{}_{}() -> Spec {{", name, v.minor(), v.revision()).unwrap();
                }
                write_generated(&mut writer, name, spec).unwrap();
                writeln!(writer, "}}").unwrap();
            }

            writeln!(writer, "pub fn specs() -> Vec<Spec> {{").unwrap();
            writeln!(writer, "vec![").unwrap();
            for &(name, ref spec) in &specs {
                let v = spec.version();
                writeln!(writer, "{}0_{}_{}(),", name, v.minor(), v.revision()).unwrap();
            }
            writeln!(writer, "]").unwrap();
            writeln!(writer, "}}").unwrap();
        }

        // file(s) needs to be dropped before reaching here
        if cfg!(feature = "rustfmt") {
            let files = vec![out_file.clone()];
            format_files(files.into_iter());
        }

        if cfg!(feature = "amqp-pregen-specs") {
            fs::copy(out_file, "src/amqp0/mod.pregen.rs").unwrap();
        }
    }

    fn amqp0_specs() -> Vec<(&'static str, &'static str)> {
        vec![
            ("amqp", "amqp0-9-1.stripped.xml"),
            ("amqp","amqp0-9.stripped.xml"),
            ("amqp","amqp0-8.stripped.xml"),
            ("rabbitmq", "amqp0-9-1.stripped.rabbitmq.xml"),
            ("qpid", "amqp0-9-qpid.stripped.xml"),
            ("qpid", "amqp0-8-qpid.stripped.xml"),
        ]
    }

    #[cfg(not(feature = "rustfmt"))]
    fn format_files<I>(_: I)
        where I: Iterator<Item = PathBuf>
    {}

    #[cfg(feature = "rustfmt")]
    fn format_files<I>(paths: I)
        where I: Iterator<Item = PathBuf>,
    {
        use rustfmt::Input;
        use rustfmt::config::{self as fmtconfig};

        let config = {
            let mut config = fmtconfig::Config::default();
            config.write_mode = fmtconfig::WriteMode::Overwrite;
            config
        };

        for path in paths {
            println!("Formatting {}", path.display());
            let summary = rustfmt::run(Input::File(path), &config);
            println!("Summary: {:?}", summary)
        }
    }
}