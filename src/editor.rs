use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor{
    should_quit: bool,
}

impl Editor{
    pub fn default() -> Self{
        Editor{should_quit: false}
    }
    pub fn run(&mut self){
        if let Err(err) = self.repl(){
            panic!("{err:#?}");
        }
        print!("Exiting editor...\r\n");
    }
    
    fn repl(&mut self) -> Result<(), std::io::Error>{
        enable_raw_mode()?;
        loop{
            if let Key(KeyEvent{
                code,modifiers,kind,state
            }) = read()?{
                print!("Key Event: {:?} {:?} {:?} {:?}\r\n",code,modifiers,kind,state);
                match code{
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }
            if self.should_quit{
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
