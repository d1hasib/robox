use colored::{ColoredString, Colorize};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "robox", about = "Greetings, Human!")]
pub struct Robox {
    /// Robotic message
    message: Option<String>,

    /// Color of the message [black | red | green | yellow | blue | magenta | cyan | white]
    #[structopt(short, long)]
    color: Option<String>,

    /// Set the mood of Robox [happy | sad | angry | wonder | crazy | wink | dead | pirate]
    #[structopt(short, long)]
    mood: Option<String>,
}

impl Robox {
    pub fn draw(&self) {
        let bubble = self.get_bubble();
        let body = self.get_body();
        println!("\n{}{}\n", bubble, body);
    }

    fn get_bubble(&self) -> String {
        let message = self.get_colored_message();
        format!("({m})\n{b:>3}\n{b:>4}", m = message, b = 'o')
    }

    fn get_body(&self) -> String {
        let eye = self.get_eye();
        format!(
            "
    +-----+
  O-| {e} |-O
    | +++ |
    +-----+",
            e = eye
        )
    }

    fn get_colored_message(&self) -> ColoredString {
        let mut message = "Greetings, Human!".normal();
        if let Some(m) = &self.message {
            match &self.color {
                Some(c) => match c.as_str() {
                    "black" => message = m.black(),
                    "red" => message = m.red(),
                    "green" => message = m.green(),
                    "yellow" => message = m.yellow(),
                    "blue" => message = m.blue(),
                    "magenta" => message = m.magenta(),
                    "cyan" => message = m.cyan(),
                    _ => message = m.normal(),
                },
                None => message = m.normal(),
            }
        }
        message
    }

    fn get_eye(&self) -> &str {
        let mut eye = "o o";
        if let Some(m) = &self.mood {
            match m.as_str() {
                "happy" => eye = "^ ^",
                "sad" => eye = "> <",
                "angry" => eye = "- -",
                "wonder" => eye = "O O",
                "crazy" => eye = "O o",
                "wink" => eye = "O <",
                "dead" => eye = "x x",
                "pirate" => eye = "o x",
                _ => (),
            }
        }
        eye
    }
}
