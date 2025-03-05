use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub const ADMIN_ROLE: &str = "admin";

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Role(String);

impl Borrow<str> for Role {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl Borrow<String> for Role {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl Debug for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Role {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct Roles {
    roles: HashSet<Role>,
}

impl Debug for Roles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.roles.fmt(f)
    }
}

impl Roles {
    pub fn new() -> Self {
        Self {
            roles: HashSet::new(),
        }
    }

    pub fn from_strs(roles: &[&str]) -> Self {
        let mut result = Roles::new();

        for role in roles {
            result.insert(Role::new(role.to_string()));
        }

        result
    }

    pub fn insert(&mut self, role: Role) -> bool {
        self.roles.insert(role)
    }

    pub fn remove<Q>(&mut self, role: &Q) -> bool
    where
        Role: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.roles.remove(role)
    }

    pub fn contains<Q>(&self, role: &Q) -> bool
    where
        Role: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.roles.contains(role)
    }
}
