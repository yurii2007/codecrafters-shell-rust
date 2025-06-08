use std::process;

pub fn exit(args: &Vec<String>) {
    process::exit(args.get(0).map_or(0, |val| (val).parse().unwrap_or(0)));
}
