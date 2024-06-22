use chrono::{DateTime, Datelike as _, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Builder, Debug)]
#[builder(build_fn(name = "_priv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
    #[builder(setter(skip))]
    age: u32,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,

    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}
fn main() -> anyhow::Result<()> {
    let user = User::build()
        .name("John Doe")
        .email("leon.zheng@awesom.com")
        .dob("1990-01-01T00:00:00Z")
        .skill("Rust")
        .skill("Python")
        .build()?;

    println!("{:?}", user);

    Ok(())
}

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn build(&self) -> anyhow::Result<User> {
        let mut user = self._priv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as _;

        Ok(user)
    }
    pub fn dob(&mut self, value: &str) -> &mut Self {
        self.dob = DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();
        self
    }
}
