use std::env::home_dir;

include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

pub fn get_home_dir() -> String {
    let home_config = home_dir().expect("No Home directory");
    home_config.as_path()
        .to_str()
        .expect("Should have a home directory to turn into a str")
        .to_string()
}

#[derive(PartialEq, Debug)]
pub enum StackItem {
    Statement {
        command: Command,
        next: Option<(String, Box<Statement>)>,
    },
}

#[derive(PartialEq, Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub pipe: Option<Box<Command>>,
    pub redirect: Option<Redirect>,
    pub vars: Vec<(String, Option<String>)>,
}

#[derive(PartialEq, Debug)]
pub struct Statement {
    pub command: Command,
    pub next: Option<(String, Box<Statement>)>,
}

#[derive(PartialEq, Debug)]
pub enum Redirect {
    Fd(Option<i32>, String, String),
    DuplicateFd(Option<i32>, String, i32),
    MoveFd(Option<i32>, String, i32),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line() {
        assert_eq!(script("").unwrap(), None);
    }

    #[test]
    fn test_single_command() {
        assert_eq!(
            script("echo").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "echo".into(),
                        args: vec![],
                        pipe: None,
                        redirect: None,
                        vars: vec![],
                    },
                    next: None,
                },
                vec![],
                None
            ))
        );
    }

    #[test]
    fn test_command_with_args() {
        assert_eq!(
            script("echo 1 2 3").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "echo".into(),
                        args: vec!["1".into(), "2".into(), "3".into()],
                        pipe: None,
                        redirect: None,
                        vars: vec![],
                    },
                    next: None,
                },
                vec![],
                None
            ))
        );
    }

    #[test]
    fn test_multiple_commands() {
        assert_eq!(
            script("echo 1; echo 2").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "echo".into(),
                        args: vec!["1".into()],
                        pipe: None,
                        redirect: None,
                        vars: vec![],
                    },
                    next: None,
                },
                vec![(
                    ";".into(),
                    Statement {
                        command: Command {
                            name: "echo".into(),
                            args: vec!["2".into()],
                            pipe: None,
                            redirect: None,
                            vars: vec![],
                        },
                        next: None,
                    },
                )],
                None
            ))
        );
    }

    #[test]
    fn test_redirect() {
        assert_eq!(
            script("echo 2>&1").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "echo".into(),
                        args: vec![],
                        pipe: None,
                        redirect: Some(Redirect::DuplicateFd(Some(2), ">&".into(), 1)),
                        vars: vec![],
                    },
                    next: None,
                },
                vec![],
                None
            ))
        )
    }

    #[test]
    fn test_pipe() {
        assert_eq!(
            script("cat file | less").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "cat".into(),
                        args: vec!["file".into()],
                        pipe: Some(Box::new(Command {
                            name: "less".into(),
                            args: vec![],
                            pipe: None,
                            redirect: None,
                            vars: vec![],
                        })),
                        redirect: None,
                        vars: vec![],
                    },
                    next: None,
                },
                vec![],
                None
            ))
        );
    }

    #[test]
    fn test_multiple_pipes() {
        assert_eq!(
            script("cat file | grep hello | less").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "cat".into(),
                        args: vec!["file".into()],
                        pipe: Some(Box::new(Command {
                            name: "grep".into(),
                            args: vec!["hello".into()],
                            pipe: Some(Box::new(Command {
                                name: "less".into(),
                                args: vec![],
                                pipe: None,
                                redirect: None,
                                vars: vec![],
                            })),
                            redirect: None,
                            vars: vec![],
                        })),
                        redirect: None,
                        vars: vec![],
                    },
                    next: None,
                },
                vec![],
                None
            ))
        )
    }

    #[test]
    fn test_and() {
        assert_eq!(
            script("echo && echo").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "echo".into(),
                        args: vec![],
                        pipe: None,
                        redirect: None,
                        vars: vec![],
                    },
                    next: Some((
                        "&&".into(),
                        Box::new(Statement {
                            command: Command {
                                name: "echo".into(),
                                args: vec![],
                                pipe: None,
                                redirect: None,
                                vars: vec![],
                            },
                            next: None,
                        }),
                    )),
                },
                vec![],
                None
            ))
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            script("echo || echo").unwrap(),
            Some((
                Statement {
                    command: Command {
                        name: "echo".into(),
                        args: vec![],
                        pipe: None,
                        redirect: None,
                        vars: vec![],
                    },
                    next: Some((
                        "||".into(),
                        Box::new(Statement {
                            command: Command {
                                name: "echo".into(),
                                args: vec![],
                                pipe: None,
                                redirect: None,
                                vars: vec![],
                            },
                            next: None,
                        }),
                    )),
                },
                vec![],
                None
            ))
        );
    }
}
