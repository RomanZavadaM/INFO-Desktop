use info_desktop_cli::{execute, parse_args, EXIT_OK, EXIT_USAGE, USAGE};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cli = match parse_args(&args) {
        Ok(cli) => cli,
        Err(error) => {
            eprintln!("{error}\n\n{USAGE}");
            std::process::exit(EXIT_USAGE);
        }
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("не вдалося запустити виконавець запитів");
    let (code, text) = runtime.block_on(execute(&cli));
    if code == EXIT_OK {
        println!("{text}");
    } else {
        eprintln!("{text}");
    }
    std::process::exit(code);
}
