use dialoguer::{
    Select,
    Input,
    Confirm,
    theme::ColorfulTheme,
    console::Term
};

use std::{
    process::Command, 
    io::Result
};


struct BruhCommit {
    title: String,
    items: Vec<String>
}

impl BruhCommit {
    fn new(title: &str, i: Vec<String>) -> BruhCommit {
        BruhCommit { 
            title: title.to_string(),
            items: i
        }
    }
    
    fn capitalize(&self, s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    fn get_commit_type(&self, default: usize) -> Result<usize> {
        let mut r#type: usize = 0;
        let sel = Select::with_theme(&ColorfulTheme::default())    
            .items(&self.items)
            .with_prompt(format!("{}", self.title))
            .default(default)
            .interact_on_opt(&Term::stderr());
        
        match sel.unwrap() {
            Some(i) => r#type = i,
            None => println!("No selection is made"),
        }

        Ok(r#type)

    }
    
    fn get_commit_msg(&self) -> String {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter commit message")
            .interact_text().unwrap()
        
    }

    /// Execute the selector and commit
    fn exec(&self) -> Result<()> {
        let commit_type = self.get_commit_type(0)?;
        let commit_msg = self.get_commit_msg();
        
        let commit = format!(
            "{}: {}", 
                self.items[commit_type].split(":")
                    .collect::<Vec<&str>>()[0], 
                self.capitalize(commit_msg.as_str())
        );
        
        let cmd = Command::new("git")
            .args(["commit", "-m", &commit])
            .output()
            .expect("There is an error executing the command");

        println!("\n{}", String::from_utf8(cmd.stdout).unwrap());

        let push = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to push?")
            .interact()?;

        if push {
            let mut out = String::from("");
            let cmd = Command::new("git")
                .arg("push")
                .output()
                .expect("Failed to execute the command");

            if cmd.stderr.is_empty() {
                out = String::from_utf8(cmd.stderr).unwrap()
            } else {
                out = String::from_utf8(cmd.stdout).unwrap()
            }

            println!("\n{}", out);
        }

        Ok(())    
    }
}

fn main() {
    let item = vec![
        String::from("🎉 init:  Initialize a new file / project"),
        String::from("✨ feat:  Introduce a new feature"),
        String::from("🚧 prog:  New features in progress"),
        String::from("⚒️  fix:   Fix a bug or any errors"),
        String::from("🎨 style: Format or style files"),
        String::from("♻️  ref:   Refactor code")
    ];
    let bruhcm = BruhCommit::new("Select commit type", item).exec();
    
}
