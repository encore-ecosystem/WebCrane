mod init;
mod pull;
mod push;
mod common;

pub async fn enter_webcrane_context(args: &[String], shift: usize) {
    if shift >= args.len() {
        println!("[Error] Enter push/pull for merge context");
        std::process::exit(-1);
    }

    match args[shift].as_str() {
        "init" => {
            init::enter_init(args).await;
        }
        "push" => {
            push::push(args, shift + 1).await;
        }
        "pull" => {
            pull::pull(args, shift + 1).await;
        }
        _ => {
            println!("Error: unknown mode <{}>", args[shift]);
        }
    }
}
