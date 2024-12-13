use std::{cell::Ref, collections::HashMap, rc::Weak, sync::Arc};

use rusqlite::{params, Connection};

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

pub trait AuthBackend<U: User> {
    fn users(&self) -> &Vec<U>;
    fn users_mut(&mut self) -> &mut Vec<U>;
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


/* Schema for SQLite3:
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

#[derive(Clone, Debug)]
struct AuthSqlBackendUser {
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
    fn open_memory() -> AuthSqlBackend {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE auth (id INTEGER PRIMARY KEY AUTOINCREMENT, name VARCHAR(32) UNIQUE, password VARCHAR(512), last_login INTEGER)", []).unwrap();
        conn.execute("CREATE TABLE user_privileges (id INTEGER, privilege VARCHAR(32), PRIMARY KEY (id, privilege), CONSTRAINT fk_id FOREIGN KEY (id) REFERENCES auth (id) ON DELETE CASCADE)", []).unwrap();
        AuthSqlBackend {
            conn,
            users: Vec::new()
        }
    }

    fn open_file(file: &str) -> AuthSqlBackend {
        let conn = Connection::open(file).unwrap();
        
        let mut backend = AuthSqlBackend {
            conn,
            users: Vec::new()
        };

        backend.reload();

        backend
    }

    fn reload(&mut self) {
        self.users.clear();

        let mut users = Vec::new();
        {
            let mut stmt = self.conn.prepare("SELECT name, password, last_login FROM auth").unwrap();

            for row in stmt.query_map([], |row| {
                Ok(AuthSqlBackendUser {
                    name: row.get(0)?,
                    password: row.get(1)?,
                    last_login: row.get(2)?,
                    privileges: Vec::new()
                })
            }).unwrap() {
                users.push(row.unwrap());
            }
        }
        {
            // Get the privileges for each user
            let mut stmt = self.conn.prepare("SELECT name, privilege FROM auth JOIN user_privileges ON auth.id = user_privileges.id").unwrap();
            
            struct PrivDataPoint {
                name: String,
                privilege: String
            }

            for row in stmt.query_map([], |row| {
                Ok(PrivDataPoint {
                    name: row.get(0)?,
                    privilege: row.get(1)?
                })
            }).unwrap() {
                let row = row.unwrap();
                let user = users.iter_mut().find(|user| user.name == row.name).unwrap();
                user.privileges.push(row.privilege);
            }
        }

        self.users = users;
    }

    fn save(&mut self) {
        // Begin transaction
        self.conn.execute("BEGIN", []).unwrap();

        // apply changes to SQLite3 connection
        let mut id_table: HashMap<String, i32> = HashMap::new();
        let mut name_table: HashMap<i32, String> = HashMap::new();
        // Save users
        {
            // Identify existing user ids in the database
            let mut stmt = self.conn.prepare("SELECT id, name FROM auth").unwrap();
            for row in stmt.query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?))
            }).unwrap() {
                let (id, name): (i32, String) = row.unwrap();
                id_table.insert(name, id);
            }

            // Populate name_table
            for (name, id) in &id_table {
                name_table.insert(*id, name.clone());
            }

            // Iterate over users, identify existing, update existing
            let mut stmt = self.conn.prepare("UPDATE auth SET name = ?, password = ?, last_login = ? WHERE id = ?").unwrap();
            for user in &self.users {
                if let Some(id) = id_table.get(&user.name) {
                    stmt.execute(params![user.name, user.password, user.last_login, id]).unwrap();
                }
            }

            // Insert new users with unique ids
            let mut stmt = self.conn.prepare("INSERT INTO auth (name, password, last_login) VALUES (?, ?, ?)").unwrap();
            for user in &self.users {
                if !id_table.contains_key(&user.name) {
                    stmt.execute(params![user.name, user.password, user.last_login]).unwrap();
                    id_table.insert(user.name.clone(), self.conn.last_insert_rowid() as i32);
                }
            }
        }
        // Save privileges
        {
            // Search for existing privileges and associate them with the in-memory users
            let temp_users = self.users.clone();
            let mut stmt = self.conn.prepare("SELECT id, privilege FROM user_privileges").unwrap();
            
            struct PrivDataPoint {
                id: i32,
                privilege: String
            }
            
            // Remove any privileges for in-memory users that no longer have them (make sure to handle Err values)
            // if user_priviliges.id == (any in-memory user).id && !(any in-memory user).privileges.contains(user_priviliges.privilege)
            let mut to_remove = Vec::new();
            for row in stmt.query_map([], |row| {
                Ok(PrivDataPoint {
                    id: row.get(0)?,
                    privilege: row.get(1)?
                })
            }).unwrap() {
                let row = row.unwrap();

                if let Some(name) = name_table.get(&row.id) {
                    if let Some(user) = temp_users.iter().find(|user| user.name == *name) {
                        if !user.privileges.contains(&row.privilege) {
                            to_remove.push((row.id, row.privilege));
                        }
                    }
                }
            }

            let mut stmt = self.conn.prepare("DELETE FROM user_privileges WHERE id = ? AND privilege = ?").unwrap();

            for (id, privilege) in to_remove {
                stmt.execute(params![id, privilege]).unwrap();
            }

            // Insert any new privileges which don't already exist
            let existing_privileges: Vec<String> = {
                let mut stmt = self.conn.prepare("SELECT DISTINCT privilege FROM user_privileges").unwrap();
                let mut existing_privileges = Vec::new();
                for row in stmt.query_map([], |row| {
                    Ok(row.get(0)?)
                }).unwrap() {
                    existing_privileges.push(row.unwrap());
                }
                existing_privileges
            };

            let mut stmt = self.conn.prepare("INSERT INTO user_privileges (id, privilege) VALUES (?, ?)").unwrap();
            for user in &self.users {
                if let Some(id) = id_table.get(&user.name) {
                    for privilege in &user.privileges {
                        if !existing_privileges.contains(privilege) {
                            stmt.execute(params![id, privilege]).unwrap();
                        }
                    }
                }
            }
            
        }
        // Clean up database
        {
            // Find and remove any users that are no longer in memory
            struct DbUser {
                id: i32,
                name: String
            }

            let existing_users: Vec<DbUser> = {
                let mut stmt = self.conn.prepare("SELECT id, name FROM auth").unwrap();
                let mut existing_users = Vec::new();
                for row in stmt.query_map([], |row| {
                    Ok(DbUser {
                        id: row.get(0)?,
                        name: row.get(1)?
                    })
                }).unwrap() {
                    existing_users.push(row.unwrap());
                }
                existing_users
            };

            let mut to_remove = Vec::new();
            for user in existing_users {
                if !self.users.iter().any(|u| u.name == user.name) {
                    to_remove.push(user.id);
                }
            }
            
            let mut stmt = self.conn.prepare("DELETE FROM auth WHERE id = ?").unwrap();
            
            for id in to_remove {
                stmt.execute(params![id]).unwrap();
            }

        }
        // Apply changes
        self.conn.execute("COMMIT", []).unwrap();
    }
}

#[cfg(test)]
mod auth_sql_backend_tests {
    use super::*;

    #[test]
    fn open_memory() {
        let backend = AuthSqlBackend::open_memory();
        assert_eq!(backend.users.len(), 0);
    }

    #[test]
    fn open_file() {
        let backend = AuthSqlBackend::open_file("assets/world_luanti_5.10/auth.sqlite");
        assert_eq!(backend.users.len(), 1);
        assert_eq!(backend.users[0].name, "singleplayer");
        assert!(backend.users[0].privileges.iter().find(|p| p.to_string() == "shout").is_some());
        assert!(backend.users[0].privileges.iter().find(|p| p.to_string() == "interact").is_some());
    }

    #[test]
    fn save() {
        // Populate the database with some users
        let mut backend = AuthSqlBackend::open_memory();
        for i in 0..10 {
            backend.users.push(AuthSqlBackendUser {
                name: format!("user{}", i),
                password: String::new(),
                last_login: 0,
                privileges: vec!["interact".to_string(), "shout".to_string()]
            });
        }
        backend.save();

        backend.reload();
        assert_eq!(backend.users.len(), 10);
        for i in 0..10 {
            assert!(backend.users.iter().find(|user| user.name == format!("user{}", i)).is_some());
        }


        // Test erasing a user
        backend.users.remove(6);

        backend.save();

        backend.reload();

        assert_eq!(backend.users.len(), 9);

        for i in 0..10 {
            if i == 6 {
                // assert that user 6 is not present
                assert!(backend.users.iter().find(|user| user.name == "user6").is_none());
                continue;
            }
            assert!(backend.users.iter().find(|user| user.name == format!("user{}", i)).is_some());
            assert!(backend.users.iter().find(|user| user.name == format!("user{}", i)).unwrap().privileges.iter().find(|p| p.to_string() == "shout").is_some());
        }
    }           
}
