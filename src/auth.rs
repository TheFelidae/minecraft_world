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

    fn has_privilege(&self, privilege: &str) -> bool {
        self.privileges().iter().any(|p| p == privilege)
    }

    fn check_password(&self, password: &str) -> bool;
}

pub trait AuthBackend<U: User> {
    fn users(&self) -> &Vec<U>;
    fn users_mut(&mut self) -> &mut Vec<U>;
    fn get_user(&self, id: String) -> Option<&U> {
        self.users().iter().find(|user| user.name() == id)
    }
    fn get_user_mut(&mut self, id: String) -> Option<&mut U> {
        self.users_mut().iter_mut().find(|user| user.name() == id)
    }
}
