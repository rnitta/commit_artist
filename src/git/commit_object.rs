use super::Gitter;
use crypto::{digest::Digest, sha1::Sha1};
use regex::Regex;

#[derive(Clone)]
pub struct CommitObject {
    pub tree: String,
    pub parent: Option<String>,
    pub author: Gitter,
    pub committer: Gitter,
    pub message: String,
}

impl std::fmt::Display for CommitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.parent {
            Some(_) => write!(
                f,
                "tree {}\nparent {}\nauthor {}\ncommitter {}\n\n{}",
                self.tree,
                self.parent.clone().unwrap(),
                self.author,
                self.committer,
                self.message
            ),
            None => write!(
                f,
                "tree {}\nauthor {}\ncommitter {}\n\n{}",
                self.tree, self.author, self.committer, self.message
            ),
        }
    }
}

impl CommitObject {
    pub fn parse_cat_file(s: &str) -> Self {
        let regx = Regex::new(
            r"^tree ([0-9a-f]{40})\r?\n(?:parent ([0-9a-f]{40})\r?\n)?author (.+?)\r?\ncommitter (.+?)\r?\n\r?\n([\s\S]+)",
        )
            .unwrap();
        let captures = regx
            .captures(s)
            .expect("Error: Commit Object Cannot Be Interpreted");
        let tree: String = captures
            .get(1)
            .expect("Error in Getting Tree Hash.")
            .as_str()
            .to_owned();
        let parent: Option<String> = captures.get(2).map(|v| v.as_str().to_owned());
        let author: Gitter = {
            let author: &str = captures.get(3).expect("Error in Getting Author").as_str();
            Gitter::parse(author)
        };
        let committer: Gitter = {
            let committer: &str = captures
                .get(4)
                .expect("Error in Getting Committer")
                .as_str();
            Gitter::parse(committer)
        };
        let message: String = captures
            .get(5)
            .expect("Error in Getting Message.")
            .as_str()
            .to_owned();

        Self {
            tree,
            parent,
            author,
            committer,
            message,
        }
    }

    // the size of commit object; not structure object's size
    pub fn bytes(&self) -> usize {
        let mut byte_count: usize = 5 // "tree "
            + self.tree.len()
            + 1 // "\n"
            + 7 // "author "
            + self.author.bytes()
            + 1 // "\n"
            + 10 // "committer "
            + self.committer.bytes()
            + 1 // "\n"
            + 1 // "\n"
            + self.message.len();

        if self.parent.is_some() {
            byte_count = byte_count + 7 // "parent "
                + self.parent.as_ref().map(|p| p.len()).unwrap() + 1; // "\n"
        }
        byte_count
    }

    pub fn to_sha1(&self, hasher: &mut Sha1) -> String {
        hasher.input_str(&format!("commit {}\0{}", self.bytes(), self));
        let r = hasher.result_str();
        hasher.reset();
        r
    }
}
