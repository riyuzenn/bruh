mod bruh;

fn main() {
    let item = vec![
        String::from("🎉 init:  Initialize a new file / project"),
        String::from("✨ feat:  Introduce a new feature"),
        String::from("🚧 prog:  New features in progress"),
        String::from("⚒️  fix:   Fix a bug or any errors"),
        String::from("🎨 style: Format or style files"),
        String::from("♻️  ref:   Refactor code"),
        String::from("📝 doc:  Update any documentation related file")
    ];

    let bruh = bruh::BruhCommit::new("Select commit type to be use", item);
    bruh.exec().unwrap();

}
