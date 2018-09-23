use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
    App::new("clapex")
        .version("0.1.0")                       
        .author("myname <myname@mail.com>")     
        .about("Clap Example CLI")              
        .arg(Arg::with_name("pa")               
        .help("sample positional argument")     
        .required(true)                         
        )
        .subcommand(SubCommand::with_name("add")
            .about("add folder to list")
            .arg(Arg::with_name("Target Project path")
                .help("Target Project path [\".\" = current path]")
                .required(true)
                .takes_value(true) 
            )
            .arg(Arg::with_name("New Boilerplate name")
                .help("New Boilerplate name")
                .takes_value(true) 
            )
            .arg(Arg::with_name("force")
                .help("Overwrite if duplicates")
                .short("f")
                .long("force")
            )
        )
}