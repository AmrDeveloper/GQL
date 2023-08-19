pub struct Arguments {
    pub repositories: Vec<String>,
    pub show_analyisis: bool,
}

pub fn parse_arguments(args: Vec<String>) -> Result<Arguments, &'static str> {
    let argc = args.len();
    if argc < 2 {
        return Err("Small number of arguments");
    }

    let mut repositories: Vec<String> = vec![];
    let mut show_analyisis = false;

    // If number of args is 2, we expect to match `<program> <repository>`
    if argc == 2 {
        repositories.push(args[1].to_owned());
        return Ok(Arguments {
            repositories,
            show_analyisis,
        });
    }

    let mut i = 1;
    loop {
        if i >= argc {
            break;
        }

        let arg = &args[i];
        if arg.eq("repo") {
            if i + 1 < argc {
                i += 1;
                // Make sure all repositories are uniques
                if repositories.contains(&args[i]) {
                    return Err("You can't add the same repository twice");
                }

                repositories.push(args[i].to_owned());

                i += 1;
                continue;
            }

            return Err("Expect repository url after -r | --repo argument");
        }

        if arg.eq("analysis") {
            show_analyisis = true;
            i += 1;
            continue;
        }

        return Err("Unexpected argument, please check help command");
    }

    return Ok(Arguments {
        repositories,
        show_analyisis,
    });
}
