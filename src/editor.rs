use crossterm::event::{read, Event::Key, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor{

}

impl Editor{
    pub fn default() -> Self{
        Editor{}
    }
    pub fn run(&self){
        enable_raw_mode().unwrap();
        loop{
            match read(){
                Ok(Key(event)) => {
                    println!("{:?}", event);
                    match event.code{
                        KeyCode::Char('q') => {
                            disable_raw_mode().unwrap();
                            break;
                        }
                        _ => {}
                        }
                },
                Err(err) => println!("Error: {:?}", err),
                _ => ()    
            }
        }
    }
}