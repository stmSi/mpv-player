use std::{env, process::exit};

fn main() {
    let mut arg = env::args();
    if arg.len() != 2 {
        println!("Usage: my-mpv <filename>");
        exit(-1);
    }
    let filepath = arg.nth(1).unwrap();
    let filepath = filepath.as_str();
    

    let mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    let mut mpv_handler = mpv_builder.build().expect("Failed to build MPV Handler.");
    let command_result = mpv_handler.command(&[
        "loadfile",
        &filepath,
    ]);

    match command_result {
        Ok(_) => (),
        Err(err) =>  {
            println!("ERROR in 'loadfile {}' command: {}", filepath, err);
            exit(-1);
        }
    }

    loop {
        if let Some(event) = mpv_handler.wait_event(0.1) {
            match event {
                mpv::Event::Shutdown => {
                    println!("Shutdown Emit!");
                    exit(0);
                },
                mpv::Event::LogMessage{ prefix, level, text, log_level} => println!("Log Message: {}", text), 
                mpv::Event::StartFile => println!("Started Playing"),
                mpv::Event::EndFile(reason_result) => println!("End File: {:?}", reason_result),
                _ => println!("other events!"),
            }
        }
    };
}
