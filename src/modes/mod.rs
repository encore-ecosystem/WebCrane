pub mod init;
pub mod merge;


pub async fn enter_webcrane_context(args: &[String], shift: usize) {
    if shift >= args.len() {
        println!("[ERROR]: Please, provide mode"); 
        std::process::exit(-1);
    }

    match args[shift].as_str() {
        "merge" => {
            merge::enter_merge_context(args, shift + 1).await;
        },
        "init" => {
            init::enter_init(args).await;
        }
        _ => {
            println!("Error: unknown mode <{}>", args[shift])
        }
    }
}
