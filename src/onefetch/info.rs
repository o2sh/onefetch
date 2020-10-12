use {
    crate::onefetch::{
        ascii_art::AsciiArt,
        cli::Options,
        commit_info::CommitInfo,
        error::*,
        language::Language,
        license::{Detector, LICENSE_FILES},
    },
    colored::{Color, ColoredString, Colorize},
    git2::Repository,
    regex::Regex,
    std::{ffi::OsStr, fmt::Write, fs},
    tokio::process::Command,
};

pub struct Info {
    git_version: String,
    git_username: String,
    project_name: String,
    current_commit: CommitInfo,
    version: String,
    creation_date: String,
    dominant_language: Language,
    languages: Vec<(Language, f64)>,
    authors: Vec<(String, usize, usize)>,
    last_change: String,
    repo_url: String,
    commits: String,
    pending: String,
    repo_size: String,
    number_of_lines: usize,
    number_of_tags: usize,
    number_of_branches: usize,
    license: String,
    config: Options,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = String::new();
        let color = match self.colors().get(0) {
            Some(&c) => c,
            None => Color::White,
        };
        if !self.config.disabled_fields.git_info {
            let git_info_length;
            if self.git_username != "" {
                git_info_length = self.git_username.len() + self.git_version.len() + 3;
                write!(
                    &mut buf,
                    "{} ~ ",
                    &self.get_formatted_info_label(&self.git_username, color)
                )?;
            } else {
                git_info_length = self.git_version.len();
            }
            write_buf(
                &mut buf,
                &self.get_formatted_info_label(&self.git_version, color),
                "",
            )?;
            let separator = "-".repeat(git_info_length);
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("", color),
                &separator,
            )?;
        }
        if !self.config.disabled_fields.project {
            let branches_str = match self.number_of_branches {
                0 => String::new(),
                1 => String::from("1 branch"),
                _ => format!("{} branches", self.number_of_branches),
            };

            let tags_str = match self.number_of_tags {
                0 => String::new(),
                1 => String::from("1 tag"),
                _ => format!("{} tags", self.number_of_tags),
            };

            let branches_tags_str = if tags_str.is_empty() && branches_str.is_empty() {
                String::new()
            } else if branches_str.is_empty() || tags_str.is_empty() {
                format!("({})", format!("{}{}", tags_str, branches_str))
            } else {
                format!("({}, {})", branches_str, tags_str)
            };

            let project_str = &self.get_formatted_info_label("Project: ", color);

            writeln!(
                buf,
                "{}{} {}",
                project_str, self.project_name, branches_tags_str
            )?;
        }

        if !self.config.disabled_fields.head {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("HEAD: ", color),
                &self.current_commit,
            )?;
        }

        if !self.config.disabled_fields.pending && self.pending != "" {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Pending: ", color),
                &self.pending,
            )?;
        }

        if !self.config.disabled_fields.version {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Version: ", color),
                &self.version,
            )?;
        }

        if !self.config.disabled_fields.created {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Created: ", color),
                &self.creation_date,
            )?;
        }

        if !self.config.disabled_fields.languages && !self.languages.is_empty() {
            if self.languages.len() > 1 {
                let title = "Languages: ";
                let pad = " ".repeat(title.len());
                let mut s = String::from("");
                let languages: Vec<(String, f64)> = {
                    let mut iter = self.languages.iter().map(|x| (format!("{}", x.0), x.1));
                    if self.languages.len() > 6 {
                        let mut languages = iter.by_ref().take(6).collect::<Vec<_>>();
                        let other_sum = iter.fold(0.0, |acc, x| acc + x.1);
                        languages.push(("Other".to_owned(), other_sum));
                        languages
                    } else {
                        iter.collect()
                    }
                };

                for (cnt, language) in languages.iter().enumerate() {
                    let formatted_number = format!("{:.*}", 1, language.1);
                    if cnt != 0 && cnt % 2 == 0 {
                        s = s + &format!("\n{}{} ({} %) ", pad, language.0, formatted_number);
                    } else {
                        s = s + &format!("{} ({} %) ", language.0, formatted_number);
                    }
                }
                writeln!(buf, "{}{}", &self.get_formatted_info_label(title, color), s)?;
            } else {
                write_buf(
                    &mut buf,
                    &self.get_formatted_info_label("Language: ", color),
                    &self.dominant_language,
                )?;
            };
        }

        if !self.config.disabled_fields.authors && !self.authors.is_empty() {
            let title = if self.authors.len() > 1 {
                "Authors: "
            } else {
                "Author: "
            };

            writeln!(
                buf,
                "{}{}% {} {}",
                &self.get_formatted_info_label(title, color),
                self.authors[0].2,
                self.authors[0].0,
                self.authors[0].1
            )?;

            let title = " ".repeat(title.len());

            for author in self.authors.iter().skip(1) {
                writeln!(
                    buf,
                    "{}{}% {} {}",
                    &self.get_formatted_info_label(&title, color),
                    author.2,
                    author.0,
                    author.1
                )?;
            }
        }

        if !self.config.disabled_fields.last_change {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Last change: ", color),
                &self.last_change,
            )?;
        }

        if !self.config.disabled_fields.repo {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Repo: ", color),
                &self.repo_url,
            )?;
        }

        if !self.config.disabled_fields.commits {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Commits: ", color),
                &self.commits,
            )?;
        }

        if !self.config.disabled_fields.lines_of_code {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Lines of code: ", color),
                &self.number_of_lines,
            )?;
        }

        if !self.config.disabled_fields.size {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Size: ", color),
                &self.repo_size,
            )?;
        }

        if !self.config.disabled_fields.license {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("License: ", color),
                &self.license,
            )?;
        }

        if !self.config.no_color_blocks {
            writeln!(
                buf,
                "\n{0}{1}{2}{3}{4}{5}{6}{7}",
                "   ".on_black(),
                "   ".on_red(),
                "   ".on_green(),
                "   ".on_yellow(),
                "   ".on_blue(),
                "   ".on_magenta(),
                "   ".on_cyan(),
                "   ".on_white()
            )?;
        }

        let center_pad = "   ";
        let mut info_lines = buf.lines();

        if let Some(custom_image) = &self.config.image {
            if let Some(image_backend) = &self.config.image_backend {
                writeln!(
                    f,
                    "{}",
                    image_backend.add_image(
                        info_lines.map(|s| format!("{}{}", center_pad, s)).collect(),
                        custom_image
                    )
                )?;
            } else {
                panic!("No image backend found")
            }
        } else {
            let mut logo_lines =
                AsciiArt::new(self.get_ascii(), self.colors(), !self.config.no_bold);
            loop {
                match (logo_lines.next(), info_lines.next()) {
                    (Some(logo_line), Some(info_line)) => {
                        writeln!(f, "{}{}{:^}", logo_line, center_pad, info_line)?
                    }
                    (Some(logo_line), None) => writeln!(f, "{}", logo_line)?,
                    (None, Some(info_line)) => writeln!(
                        f,
                        "{:<width$}{}{:^}",
                        "",
                        center_pad,
                        info_line,
                        width = logo_lines.width()
                    )?,
                    (None, None) => {
                        writeln!(f, "\n")?;
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Info {
    #[tokio::main]
    pub async fn new(config: Options) -> Result<Info> {
        let repo = Repository::discover(&config.path)
            .chain_err(|| "Could not find a valid git repo on the current path")?;
        let workdir = repo
            .workdir()
            .chain_err(|| "Unable to run onefetch on bare git repo")?;
        let workdir_str = workdir.to_str().unwrap();
        let (languages_stats, number_of_lines) =
            Language::get_language_stats(workdir_str, &config.excluded)?;

        let (
            (repository_name, repository_url),
            git_history,
            (number_of_tags, number_of_branches),
            current_commit_info,
            (git_v, git_user),
            version,
            pending,
            repo_size,
            project_license,
            dominant_language,
        ) = futures::join!(
            Info::get_repo_name_and_url(&repo),
            Info::get_git_history(workdir_str, config.no_merges),
            Info::get_number_of_tags_branches(workdir_str),
            Info::get_current_commit_info(&repo),
            Info::get_git_version_and_username(workdir_str),
            Info::get_version(workdir_str),
            Info::get_pending_changes(workdir_str),
            Info::get_packed_size(workdir_str),
            Info::get_project_license(workdir_str),
            Language::get_dominant_language(&languages_stats)
        );

        let creation_date = Info::get_creation_date(&git_history);
        let number_of_commits = Info::get_number_of_commits(&git_history);
        let authors = Info::get_authors(&git_history, config.number_of_authors);
        let last_change = Info::get_date_of_last_commit(&git_history);

        Ok(Info {
            git_version: git_v,
            git_username: git_user,
            project_name: repository_name,
            current_commit: current_commit_info?,
            version: version?,
            creation_date: creation_date?,
            dominant_language,
            languages: languages_stats,
            authors,
            last_change: last_change?,
            repo_url: repository_url,
            commits: number_of_commits,
            pending: pending?,
            repo_size: repo_size?,
            number_of_lines,
            number_of_tags,
            number_of_branches,
            license: project_license?,
            config,
        })
    }

    async fn get_git_history(dir: &str, no_merges: bool) -> Vec<String> {
        let mut args = vec!["-C", dir, "log"];
        if no_merges {
            args.push("--no-merges");
        }

        args.push("--pretty=%cr\t%ae\t%an");

        let output = Command::new("git")
            .args(args)
            .output()
            .await
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);
        output.lines().map(|x| x.to_string()).collect::<Vec<_>>()
    }

    async fn get_number_of_tags_branches(dir: &str) -> (usize, usize) {
        let tags = async {
            let output = Command::new("git")
                .args(vec!["-C", dir, "tag"])
                .output()
                .await
                .expect("Failed to execute git.");

            let tags = String::from_utf8_lossy(&output.stdout);

            tags.lines().count()
        };

        let branches = async {
            let output = Command::new("git")
                .args(vec!["-C", dir, "branch", "-r"])
                .output()
                .await
                .expect("Failed to execute git.");

            let branches = String::from_utf8_lossy(&output.stdout);

            if branches.lines().count() > 0 {
                branches.lines().count() - 1 //Exclude origin/HEAD -> origin/master
            } else {
                0
            }
        };

        futures::join!(tags, branches)
    }

    async fn get_repo_name_and_url(repo: &Repository) -> (String, String) {
        let config = repo
            .config()
            .chain_err(|| "Could not retrieve git configuration data");
        let mut remote_url = String::new();
        let mut repository_name = String::new();

        let remote_regex = Regex::new(r"remote\.[a-zA-Z0-9]+\.url").unwrap();

        for entry in &config.unwrap().entries(None).unwrap() {
            let entry = entry.unwrap();
            if remote_regex.is_match(entry.name().unwrap()) {
                remote_url = entry.value().unwrap().to_string()
            };
        }

        let name_parts: Vec<&str> = remote_url.split('/').collect();

        if !name_parts.is_empty() {
            let mut i = 1;
            while repository_name.is_empty() && i <= name_parts.len() {
                repository_name = name_parts[name_parts.len() - i].to_string();
                i += 1;
            }
        }

        if repository_name.contains(".git") {
            let repo_name = repository_name.clone();
            let parts: Vec<&str> = repo_name.split(".git").collect();
            repository_name = parts[0].to_string();
        }

        (repository_name, remote_url)
    }

    async fn get_current_commit_info(repo: &Repository) -> Result<CommitInfo> {
        let head = repo
            .head()
            .chain_err(|| "Error while retrieving reference information")?;
        let head_oid = head
            .target()
            .ok_or("Error while retrieving reference information")?;
        let refs = repo
            .references()
            .chain_err(|| "Error while retrieving reference information")?;
        let refs_info = refs
            .filter_map(|reference| match reference {
                Ok(reference) => match (reference.target(), reference.shorthand()) {
                    (Some(oid), Some(shorthand)) if oid == head_oid => {
                        Some(if reference.is_tag() {
                            String::from("tags/") + shorthand
                        } else {
                            String::from(shorthand)
                        })
                    }
                    _ => None,
                },
                Err(_) => None,
            })
            .collect::<Vec<String>>();
        Ok(CommitInfo::new(head_oid, refs_info))
    }

    fn get_authors(git_history: &[String], n: usize) -> Vec<(String, usize, usize)> {
        let mut authors = std::collections::HashMap::new();
        let mut author_name_by_email = std::collections::HashMap::new();
        let mut total_commits = 0;
        for line in git_history {
            let author_email = line.split('\t').collect::<Vec<_>>()[1].to_string();
            let author_name = line.split('\t').collect::<Vec<_>>()[2].to_string();
            let commit_count = authors.entry(author_email.to_string()).or_insert(0);
            author_name_by_email
                .entry(author_email.to_string())
                .or_insert(author_name);
            *commit_count += 1;
            total_commits += 1;
        }

        let mut authors: Vec<(String, usize)> = authors.into_iter().collect();
        authors.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

        authors.truncate(n);

        let authors: Vec<(String, usize, usize)> = authors
            .into_iter()
            .map(|(author, count)| {
                (
                    author_name_by_email
                        .get(&author)
                        .unwrap()
                        .trim_matches('\'')
                        .to_string(),
                    count,
                    count * 100 / total_commits,
                )
            })
            .collect();

        authors
    }

    async fn get_git_version_and_username(dir: &str) -> (String, String) {
        let version = Command::new("git")
            .arg("--version")
            .output()
            .await
            .expect("Failed to execute git.");
        let version = String::from_utf8_lossy(&version.stdout).replace('\n', "");

        let username = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("config")
            .arg("--get")
            .arg("user.name")
            .output()
            .await
            .expect("Failed to execute git.");
        let username = String::from_utf8_lossy(&username.stdout).replace('\n', "");
        (version, username)
    }

    async fn get_version(dir: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("describe")
            .arg("--abbrev=0")
            .arg("--tags")
            .output()
            .await
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);

        if output == "" {
            Ok("??".into())
        } else {
            Ok(output.to_string().replace('\n', ""))
        }
    }

    fn get_number_of_commits(git_history: &[String]) -> String {
        let number_of_commits = git_history.len();
        number_of_commits.to_string()
    }

    async fn get_pending_changes(dir: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("status")
            .arg("--porcelain")
            .output()
            .await
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);

        if output == "" {
            Ok("".into())
        } else {
            let lines = output.lines();

            let mut deleted = 0;
            let mut added = 0;
            let mut modified = 0;

            for line in lines {
                let prefix = &line[..2];

                match prefix.trim() {
                    "D" => deleted += 1,
                    "A" | "AM" | "??" => added += 1,
                    "M" | "MM" | "R" => modified += 1,
                    _ => {}
                }
            }

            let mut result = String::from("");
            if modified > 0 {
                result = format!("{}+-", modified)
            }

            if added > 0 {
                result = format!("{} {}+", result, added);
            }

            if deleted > 0 {
                result = format!("{} {}-", result, deleted);
            }

            Ok(result.trim().into())
        }
    }

    async fn get_packed_size(dir: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("count-objects")
            .arg("-vH")
            .output()
            .await
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);
        let lines = output.to_string();
        let size_line = lines
            .split('\n')
            .find(|line| line.starts_with("size-pack:"));

        let repo_size = match size_line {
            None => "??",
            Some(size_str) => &(size_str[11..]),
        };

        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("ls-files")
            .output()
            .await
            .expect("Failed to execute git.");
        // To check if command executed successfully or not
        let error = &output.stderr;

        if error.is_empty() {
            let output = String::from_utf8_lossy(&output.stdout);

            let lines = output.to_string();
            let files_list = lines.split('\n');
            let mut files_count: u128 = 0;
            for _file in files_list {
                files_count += 1;
            }
            files_count -= 1; // As splitting giving one line extra(blank).
            let res = repo_size.to_owned() + (" (") + &(files_count.to_string()) + (" files)");
            Ok(res)
        } else {
            let res = repo_size;
            Ok(res.into())
        }
    }

    fn get_date_of_last_commit(git_history: &[String]) -> Result<String> {
        let last_commit = git_history.first();

        let output = match last_commit {
            Some(date) => date.split('\t').collect::<Vec<_>>()[0].to_string(),
            None => "??".into(),
        };

        Ok(output)
    }

    fn get_creation_date(git_history: &[String]) -> Result<String> {
        let first_commit = git_history.last();

        let output = match first_commit {
            Some(creation_time) => creation_time.split('\t').collect::<Vec<_>>()[0].to_string(),
            None => "??".into(),
        };

        Ok(output)
    }

    async fn get_project_license(dir: &str) -> Result<String> {
        fn is_license_file<S: AsRef<str>>(file_name: S) -> bool {
            LICENSE_FILES
                .iter()
                .any(|&name| file_name.as_ref().starts_with(name))
        }

        let detector = Detector::new()?;

        let mut output = fs::read_dir(dir)
            .chain_err(|| "Could not read directory")?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|entry| {
                entry.is_file()
                    && entry
                        .file_name()
                        .map(OsStr::to_string_lossy)
                        .map(is_license_file)
                        .unwrap_or_default()
            })
            .filter_map(|entry| {
                let contents = fs::read_to_string(entry).unwrap_or_default();
                detector.analyze(&contents)
            })
            .collect::<Vec<_>>();

        output.sort();
        output.dedup();
        let output = output.join(", ");

        if output == "" {
            Ok("??".into())
        } else {
            Ok(output)
        }
    }

    fn get_ascii(&self) -> &str {
        let language = if let Language::Unknown = self.config.ascii_language {
            &self.dominant_language
        } else {
            &self.config.ascii_language
        };

        language.get_ascii_art()
    }

    fn colors(&self) -> Vec<Color> {
        let language = if let Language::Unknown = self.config.ascii_language {
            &self.dominant_language
        } else {
            &self.config.ascii_language
        };

        let colors = language.get_colors();

        let colors: Vec<Color> = colors
            .iter()
            .enumerate()
            .map(|(index, default_color)| {
                if let Some(color_num) = self.config.ascii_colors.get(index) {
                    if let Some(color) = Info::num_to_color(color_num) {
                        return color;
                    }
                }
                *default_color
            })
            .collect();
        colors
    }

    fn num_to_color(num: &str) -> Option<Color> {
        let color = match num {
            "0" => Color::Black,
            "1" => Color::Red,
            "2" => Color::Green,
            "3" => Color::Yellow,
            "4" => Color::Blue,
            "5" => Color::Magenta,
            "6" => Color::Cyan,
            "7" => Color::White,
            "8" => Color::BrightBlack,
            "9" => Color::BrightRed,
            "10" => Color::BrightGreen,
            "11" => Color::BrightYellow,
            "12" => Color::BrightBlue,
            "13" => Color::BrightMagenta,
            "14" => Color::BrightCyan,
            "15" => Color::BrightWhite,
            _ => return None,
        };
        Some(color)
    }

    /// Returns a formatted info label with the desired color and boldness
    fn get_formatted_info_label(&self, label: &str, color: Color) -> ColoredString {
        let formatted_label = label.color(color);
        if self.config.no_bold {
            formatted_label
        } else {
            formatted_label.bold()
        }
    }
}

fn write_buf<T: std::fmt::Display>(
    buffer: &mut String,
    title: &ColoredString,
    content: T,
) -> std::fmt::Result {
    writeln!(buffer, "{}{}", title, content)
}
