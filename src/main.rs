#![allow(dead_code, unused_imports, unused_variables)]


use crossterm::{
    cursor::{Hide, Show}, 
    event::{self, Event, KeyCode, poll}, 
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, ClearType},
    ExecutableCommand,
};
use std::{io::{Write, stdout, Stdout}, error::Error};


mod openai;
use crate::openai::*;

use reqwest::{Client, Response, header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{


    // open ai set up
    let mut openai_client = OpenAIClient::new()?;

    // terminal set up
    let mut stdout: Stdout;
    match setup_terminal() {
        Ok(r) => {
            stdout = r;
        },
        Err(e) => {
            println!("error initializing terminal with error {e}");
            return Err(e);
        }
    }

    writeln!(stdout, "starts chatting, exit with `ESC` key. \r")?;
    write!(stdout, "\n\rYou: ")?;
    stdout.flush()?;

    let mut user_input: String = String::new();

    'chat: loop {

        if let Event::Key(key_event) = event::read()? {

            match key_event.code {
                KeyCode::Char(c) => {
                    write!(stdout, "{}", c)?;
                    stdout.flush()?;
                    user_input.push(c);
                },
                KeyCode::Enter => {
                    if user_input.is_empty() {
                        let message = String::from("\rPlease enter some thing!");
                        write!(stdout, "{}", message)?;
                        stdout.flush()?;
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        for _ in 0..message.len(){
                            write!(stdout, "\x08 \x08")?;
                        }
                        write!(stdout, "\rYou: ")?;
                        stdout.flush()?;
                        continue;
                    }
                    write!(stdout, "\n\r ..... Please wait")?;
                    stdout.flush()?;                    

                    let assistant_message = openai_client.send_message(&user_input).await?;
                    write!(stdout, "\rAssistant: {}", assistant_message)?;
                    stdout.flush()?;
                    write!(stdout, "\n\rYou: ")?;
                    stdout.flush()?;

                    user_input = String::from("");

                }, 
                KeyCode::Backspace => {
                    write!(stdout, "\x08 \x08")?;
                    stdout.flush()?;
                    user_input.pop();
                },
                KeyCode::Esc => {
                    break 'chat;
                }, 
                _ => {},   
            }
        }


    };


    // terminal clean up

    match cleanup_terminal(stdout) {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            println!("error cleaning up terminal with error {e}");
            return Err(e);
        },
    }

}


fn setup_terminal() -> Result<Stdout, Box<dyn Error>> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    // stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    return Ok(stdout);
}

fn cleanup_terminal(mut stdout: Stdout) -> Result<(), Box<dyn Error>> {
    stdout.execute(Show)?;
    // stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    return Ok(());

}

