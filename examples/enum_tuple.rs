//! How to extract subcommands' args into external structs.

use bpaf::*;

#[derive(Debug, Clone)]
pub struct Foo {
    pub bar: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Command {
    Foo(Foo),
}

fn main() {
    let bar = short('b')
        .long("bar")
        .help("some bar command")
        .argument()
        .build()
        .optional();

    let bar_cmd = Info::default()
        .descr("This command will try to do foo given a bar argument")
        .for_parser(construct!(Foo: bar));
    let command = command("foo", "command for doing foo", bar_cmd).map(Command::Foo);

    let opt = run(Info::default().for_parser(command));
    println!("{:#?}", opt);
}