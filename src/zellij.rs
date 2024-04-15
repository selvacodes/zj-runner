use std::process::Command;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Zellij {
    sessions: Vec<ZellijSession>,
}

#[derive(Debug, Clone)]
pub struct ZellijSession {
    name: String,
    created_before: String,
}

impl ZellijSession {
    // pub fn builder() -> SessionBuilder<NoName, NoCreated> {
    //     SessionBuilder::new()
    // }

    pub fn new(name: String, created_before: String) -> Self {
        ZellijSession {
            name,
            created_before,
        }
    }
}

struct ZellijsLsOut(String);

impl Zellij {
    pub fn new() -> Result<Self> {
        let mut ze = Command::new("zellij");
        ze.arg("list-sessions");
        ze.arg("--no-formatting");
        let x = ze.output().context("Errorn in executing zellij command")?;
        let y = String::from_utf8(x.stdout).context("Unable to make session names from output")?;
        let sessions: Vec<String> = y.lines().map(String::from).collect();
        dbg!(sessions.clone());
        let y = sessions
            .iter()
            .map(|session| {
                // let new_session = ZellijSession::builder();

                let session_c = session.clone();
                let space_position = session_c.chars().position(|c| c == ' ').unwrap();
                let name = String::from(&session_c[0..space_position]);
                let open_brace_position = session_c.chars().position(|c| c == '[').unwrap();
                let close_brace_position = session_c.chars().position(|c| c == ']').unwrap();
                let created =
                    String::from(&session_c[open_brace_position + 9..close_brace_position]);
                let open_brace_position = session_c.chars().position(|c| c == '(').unwrap();
                ZellijSession::new(name, created)

                // let mut split_itreator = session.split_whitespace();
                // let name_from_cmd = split_itreator.next().unwrap_or("DUMMY SESSION");
                // let session_with_name = new_session.name(String::from(name_from_cmd));
                // let created_from_cmd = split_itreator.next().unwrap_or("DUMMY CREATED");
                // let session_with_all_data =
                //     session_with_name.created_before(String::from(created_from_cmd));
                // session_with_all_data.build()
            })
            .collect::<Vec<_>>();

        Ok(Zellij { sessions: y })
    }
}

// #[derive(Debug, Clone)]
// pub struct SessionBuilder<T, U> {
//     name: T,
//     created_before: U,
// }
//
// #[derive(Debug, Clone)]
// pub struct NoCreated;
// #[derive(Debug, Clone)]
// pub struct Created(String);
//
// #[derive(Debug, Clone)]
// pub struct NoName;
// #[derive(Debug, Clone)]
// pub struct Name(String);
//
// impl SessionBuilder<NoName, NoCreated> {
//     pub fn new() -> Self {
//         SessionBuilder {
//             name: NoName,
//             created_before: NoCreated,
//         }
//     }
// }
//
// impl<N> SessionBuilder<NoName, N> {
//     pub fn name(self, name: String) -> SessionBuilder<Name, N> {
//         SessionBuilder {
//             name: Name(name),
//             created_before: self.created_before,
//         }
//     }
// }
//
// impl<T> SessionBuilder<T, NoCreated> {
//     pub fn created_before(self, created_before: String) -> SessionBuilder<T, Created> {
//         SessionBuilder {
//             name: self.name,
//             created_before: Created(created_before),
//         }
//     }
// }
//
// impl SessionBuilder<Name, Created> {
//     pub fn build(self) -> ZellijSession {
//         ZellijSession {
//             name: self.name.0,
//             created_before: self.created_before.0,
//         }
//     }
// }
