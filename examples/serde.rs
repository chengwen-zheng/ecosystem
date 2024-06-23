use core::fmt;

use chrono::{DateTime, Utc};
use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeStruct as _,
    Deserialize, Serialize,
};

// #[derive(Debug, PartialEq, Deserialize, Serialize)]
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
    dob: DateTime<Utc>,
}

fn main() -> anyhow::Result<()> {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
        dob: Utc::now(),
    };

    let json = serde_json::to_string(&user)?;

    println!("{}", json);

    let user1: User = serde_json::from_str(&json)?;
    println!("{:?}", user1);

    assert_eq!(user, user1);

    Ok(())
}

// Implement Serialize and Deserialize for User. This is the same as #[derive(Serialize, Deserialize)]
impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("skills", &self.skills)?;
        state.serialize_field("dob", &self.dob)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UserVisitor;

        impl<'de> Visitor<'de> for UserVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct User")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut name = None;
                let mut age = None;
                let mut skills = None;
                let mut dob = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "name" => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        "age" => {
                            if age.is_some() {
                                return Err(serde::de::Error::duplicate_field("age"));
                            }
                            age = Some(map.next_value()?);
                        }
                        "skills" => {
                            if skills.is_some() {
                                return Err(serde::de::Error::duplicate_field("skills"));
                            }
                            skills = Some(map.next_value()?);
                        }
                        "dob" => {
                            if dob.is_some() {
                                return Err(serde::de::Error::duplicate_field("dob"));
                            }
                            dob = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let age = age.ok_or_else(|| serde::de::Error::missing_field("age"))?;
                let skills = skills.ok_or_else(|| serde::de::Error::missing_field("skills"))?;
                let dob = dob.ok_or_else(|| serde::de::Error::missing_field("dob"))?;

                Ok(User {
                    name,
                    age,
                    skills,
                    dob,
                })
            }
        }

        deserializer.deserialize_struct("User", &["name", "age", "skills", "dob"], UserVisitor)
    }
}
