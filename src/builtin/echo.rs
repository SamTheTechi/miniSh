use crate::strip_quotes;

pub fn run(args: &str) {
    if (args.starts_with("'") && args.ends_with("'"))
        || (args.starts_with('"') && args.ends_with('"'))
    {
        let refined_args = strip_quotes(args);
        println!("{}", refined_args);
    } else {
        println!("{}", args);
    }
}
