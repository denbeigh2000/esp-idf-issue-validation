use std::env::current_dir;

fn main() {
    let metadata = cargo_metadata::MetadataCommand::new()
        .current_dir(current_dir())
        .other_options(vec!["--locked".into(), filter_string])
        .exec()
        .unwrap();
    eprintln!("{:?}", metadata.root_package().unwrap().metadata);
    embuild::espidf::sysenv::output();
}
