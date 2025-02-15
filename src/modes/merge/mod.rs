mod pull;
mod push;


pub async fn enter_merge_context(args: &[String], shift: usize) {
    if shift >= args.len() {
        println!("[Error] Enter push/pull for merge context");
        std::process::exit(-1);
    }
    
    match args[shift].as_str() {
        "pull" => {
            pull::merge_pull(args, shift + 1).await;
        },
        "push" => {
            push::merge_push(args, shift + 1).await;
        },
        _ => {
            println!("Error: unknown mode for merge context <{}>", args[shift]);
        },
    }
}
