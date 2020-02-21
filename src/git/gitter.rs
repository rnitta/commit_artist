use regex::Regex;

#[derive(Clone)]
pub struct Gitter {
    pub name: String,
    pub email_user: String,
    pub email_domain: String,
    pub time: String,
}

impl std::fmt::Display for Gitter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} <{}@{}> {}",
            self.name, self.email_user, self.email_domain, self.time
        )
    }
}

impl Gitter {
    pub fn parse(line: &str) -> Self {
        let regx = Regex::new(r"^(.+?) <(\S+?)@(\S+?)> (.+)$").unwrap();
        let captures = regx.captures(line).expect("Error: unparsable");

        let name: String = captures
            .get(1)
            .expect("Error in Getting Name")
            .as_str()
            .to_owned();
        let email_user: String = captures
            .get(2)
            .expect("Error in Getting EmailUser")
            .as_str()
            .to_owned();
        let email_domain: String = captures
            .get(3)
            .expect("Error in Getting EmailDomain")
            .as_str()
            .to_owned();
        let time: String = captures
            .get(4)
            .expect("Error in Getting Time")
            .as_str()
            .to_owned();

        Self {
            name,
            email_user,
            email_domain,
            time,
        }
    }

    // rnitta <attinyes@gmail.com> 1571472461 +0900
    pub fn bytes(&self) -> usize {
        self.name.len()
            + 2 // " <"
            + self.email_user.len()
            + 1 // "@"
            + self.email_domain.len()
            + 2 // "> "
            + self.time.len()
    }
}
