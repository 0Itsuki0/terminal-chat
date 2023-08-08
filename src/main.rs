use termimad::crossterm::{
    cursor::{Hide, Show}, 
    event::{self, Event, KeyCode}, 
    terminal::{self},
    ExecutableCommand,
    style::*,
};
use termimad::MadSkin;

use std::{io::{Write, stdout, Stdout}, error::Error};


mod openai;
use crate::openai::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    // set up skin
    let skin = setup_skin();

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

    skin.write_text("starts chatting, exit with `ESC` key. \r")?;
    skin.write_inline("\n\rYou: ")?;
    stdout.flush()?;

    let mut user_input: String = String::new();

    'chat: loop {

        if let Event::Key(key_event) = event::read()? {

            match key_event.code {
                KeyCode::Char(c) => {
                    skin.write_inline(&c.to_string())?;
                    stdout.flush()?;
                    user_input.push(c);
                },
                KeyCode::Enter => {
                    if user_input.is_empty() {
                        let message = String::from("\rPlease enter some thing!");
                        skin.write_inline(&message)?;
                        stdout.flush()?;
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        for _ in 0..message.len(){
                            skin.write_inline("\x08 \x08")?;
                        }
                        skin.write_inline("\rYou: ")?;
                        stdout.flush()?;
                        continue;
                    }

                    skin.write_inline("\n\r ..... Please wait")?;
                    stdout.flush()?;                    

                    let assistant_message = openai_client.send_message(&user_input).await?;
                    let mut message = String::from("\rAssistant: ");
                    message.push_str( &assistant_message);

                    terminal::disable_raw_mode()?;
                    skin.write_text(message.as_str())?;
                    terminal::enable_raw_mode()?;

                    skin.write_inline("\rYou: ")?;
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
    };

}


fn setup_terminal() -> Result<Stdout, Box<dyn Error>> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(Hide)?;
    return Ok(stdout);
}

fn cleanup_terminal(mut stdout: Stdout) -> Result<(), Box<dyn Error>> {
    stdout.execute(Show)?;
    terminal::disable_raw_mode()?;
    return Ok(());

}

// skin set up
fn setup_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Color::Red);
    skin.italic.add_attr(Attribute::Underlined);
    return skin;
}
