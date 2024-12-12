use std::{cell::Ref, collections::HashMap, rc::Weak, sync::Arc};

use rusqlite::Connection;

pub trait User {
    fn name(&self) -> String;
    fn password(&self) -> String;
    fn last_login(&self) -> i32;
    fn privileges(&self) -> Vec<String>;
    
    fn set_id(&mut self, id: String);
    fn set_name(&mut self, name: String);
    fn set_password(&mut self, password: String);
    fn set_last_login(&mut self, last_login: i32);
    fn set_privileges(&mut self, privileges: Vec<String>);
}

pub trait Auth<'a, U: User> {
    fn users(&'a self) -> &'a Vec<U>;
    fn get_user(&'a self, id: String) -> Option<&'a U>;
    fn add_user(&'a mut self, id: String) -> Option<&'a U>;
}

struct AuthTxtBackend {
    users: Vec<AuthTxtBackendUser>
}

struct AuthTxtBackendUser {
    name: String,
    password: String,
    privileges: Vec<String>,
    last_login: i32
}

impl User for AuthTxtBackendUser {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn password(&self) -> String {
        self.password.clone()
    }

    fn last_login(&self) -> i32 {
        self.last_login
    }

    fn privileges(&self) -> Vec<String> {
        self.privileges.clone()
    }

    fn set_id(&mut self, id: String) {
        unimplemented!()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn set_last_login(&mut self, last_login: i32) {
        self.last_login = last_login;
    }

    fn set_privileges(&mut self, privileges: Vec<String>) {
        self.privileges = privileges;
    }
}

impl AuthTxtBackend {
    fn from(serialized: &str) -> AuthTxtBackend {
        let mut data = Vec::new();
        for line in serialized.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap().trim();
            let password = parts.next().unwrap_or("").trim();
            let mut parts = password.splitn(2, ':');
            let password = parts.next().unwrap().trim();
            let privileges = parts.next().unwrap_or("").trim().split(',').map(|x| x.to_string()).collect();
            data.push(AuthTxtBackendUser {
                name: name.to_string(),
                password: password.to_string(),
                privileges,
                last_login: 0
            });
        }

        AuthTxtBackend {
            users: data
        }
    }
}

impl<'a> Auth<'a, AuthTxtBackendUser> for AuthTxtBackend {
    fn users(&'a self) -> &'a Vec<AuthTxtBackendUser> {
        &self.users
    }

    fn get_user(&'a self, id: String) -> Option<&'a AuthTxtBackendUser> {
        self.users.iter().find(|user| user.name == id)
    }
    
    fn add_user(&'a mut self, id: String) -> Option<&'a AuthTxtBackendUser> {
        self.users.push(AuthTxtBackendUser {
            name: id,
            password: String::new(),
            privileges: Vec::new(),
            last_login: 0
        });
        self.users.last()
    }
}

#[cfg(test)]
mod auth_txt_backend_tests {
    use super::*;

    #[test]
    fn from() {
        let backend = AuthTxtBackend::from("celeron55::interact,shout");
        assert_eq!(backend.users.len(), 1);
        assert_eq!(backend.users[0].name(), "celeron55");
        assert_eq!(backend.users[0].privileges(), vec!["interact", "shout"]);
    }
}


/* Schema for SQLite3
CREATE TABLE `auth` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `name` VARCHAR(32) UNIQUE,
    `password` VARCHAR(512),
    `last_login` INTEGER
);
CREATE TABLE `user_privileges` (
    `id` INTEGER,
    `privilege` VARCHAR(32),
    PRIMARY KEY (id, privilege),
    CONSTRAINT fk_id FOREIGN KEY (id) REFERENCES auth (id) ON DELETE CASCADE
);
*/

struct AuthSqlBackendUser {
    id: i32,
    name: String,
    password: String,
    last_login: i32,
    privileges: Vec<String>
}

struct AuthSqlBackend {
    conn: Connection,
    users: Vec<AuthSqlBackendUser>
}

impl AuthSqlBackend {
    fn open_file(file: &str) -> AuthSqlBackend {
        let conn = Connection::open(file).unwrap();
        let mut users = Vec::new();
        {
            let mut stmt = conn.prepare("SELECT id, name, password, last_login FROM auth").unwrap();

            for row in stmt.query_map([], |row| {
                Ok(AuthSqlBackendUser {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    password: row.get(2)?,
                    last_login: row.get(3)?,
                    privileges: Vec::new()
                })
            }).unwrap() {
                users.push(row.unwrap());
            }
        }
        AuthSqlBackend {
            conn,
            users
        }
    }

    fn save() {
        // apply changes to SQLite3
        
    }
}
