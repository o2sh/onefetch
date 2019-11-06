use std::ffi::OsStr;
use std::fmt::Write;
use std::fs;
use std::process::Command;

use colored::{Color, ColoredString, Colorize};
use git2::Repository;
use image::DynamicImage;
use license::Detector;

use crate::image_backends;
use crate::language::Language;
use crate::{AsciiArt, CommitInfo, Configuration, Error, InfoFieldOn};

type Result<T> = std::result::Result<T, crate::Error>;

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
    repo: String,
    commits: String,
    repo_size: String,
    number_of_lines: usize,
    license: String,
    custom_logo: Language,
    custom_colors: Vec<String>,
    disable_fields: InfoFieldOn,
    bold_enabled: bool,
    custom_image: Option<DynamicImage>,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = String::new();
        let color = match self.colors().get(0) {
            Some(&c) => c,
            None => Color::White,
        };
        if !self.disable_fields.git_info {
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
        if !self.disable_fields.project {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Project: ", color),
                &self.project_name,
            )?;
        }

        if !self.disable_fields.head {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("HEAD: ", color),
                &self.current_commit,
            )?;
        }

        if !self.disable_fields.version {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Version: ", color),
                &self.version,
            )?;
        }

        if !self.disable_fields.created {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Created: ", color),
                &self.creation_date,
            )?;
        }

        if !self.disable_fields.languages && !self.languages.is_empty() {
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
                    if cnt != 0 && cnt % 3 == 0 {
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

        if !self.disable_fields.authors && !self.authors.is_empty() {
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

        if !self.disable_fields.last_change {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Last change: ", color),
                &self.last_change,
            )?;
        }

        if !self.disable_fields.repo {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Repo: ", color),
                &self.repo,
            )?;
        }

        if !self.disable_fields.commits {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Commits: ", color),
                &self.commits,
            )?;
        }

        if !self.disable_fields.lines_of_code {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Lines of code: ", color),
                &self.number_of_lines,
            )?;
        }

        if !self.disable_fields.size {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("Size: ", color),
                &self.repo_size,
            )?;
        }

        if !self.disable_fields.license {
            write_buf(
                &mut buf,
                &self.get_formatted_info_label("License: ", color),
                &self.license,
            )?;
        }

        writeln!(
            buf,
            "\n{0}{1}{2}{3}{4}{5}{6}{7}\n{8}{9}{10}{11}{12}{13}{14}{15}",
            "   ".on_black(),
            "   ".on_red(),
            "   ".on_green(),
            "   ".on_yellow(),
            "   ".on_blue(),
            "   ".on_magenta(),
            "   ".on_cyan(),
            "   ".on_white(),
            "   ".on_bright_black(),
            "   ".on_bright_red(),
            "   ".on_bright_green(),
            "   ".on_bright_yellow(),
            "   ".on_bright_blue(),
            "   ".on_bright_magenta(),
            "   ".on_bright_cyan(),
            "   ".on_bright_white(),
        )?;

        let center_pad = "   ";
        let mut info_lines = buf.lines();

        if let Some(custom_image) = &self.custom_image {
            if let Some(backend) = image_backends::get_best_backend() {
                writeln!(
                    f,
                    "{}",
                    backend.add_image(
                        info_lines.map(|s| format!("{}{}", center_pad, s)).collect(),
                        custom_image
                    )
                )?;
            } else {
                panic!("No image backend found")
            }
        } else {
            let mut logo_lines = AsciiArt::new(self.get_ascii(), self.colors(), self.bold_enabled);
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
    pub fn new(
        dir: &str,
        logo: Language,
        colors: Vec<String>,
        disabled: InfoFieldOn,
        bold_flag: bool,
        custom_image: Option<DynamicImage>,
        no_merges: bool,
    ) -> Result<Info> {
        let repo = Repository::discover(&dir).map_err(|_| Error::NotGitRepo)?;
        let workdir = repo.workdir().ok_or(Error::BareGitRepo)?;
        let workdir_str = workdir.to_str().unwrap();

        let config = Info::get_configuration(&repo)?;
        let current_commit_info = Info::get_current_commit_info(&repo)?;
        let authors = Info::get_authors(workdir_str, no_merges, 3);
        let (git_v, git_user) = Info::get_git_info(workdir_str);
        let version = Info::get_version(workdir_str)?;
        let commits = Info::get_commits(workdir_str, no_merges)?;
        let repo_size = Info::get_packed_size(workdir_str)?;
        let last_change = Info::get_last_change(workdir_str)?;
        let creation_date = Info::get_creation_time(workdir_str)?;
        let project_license = Info::get_project_license(workdir_str)?;
        let (languages_stats, number_of_lines) = Language::get_language_stats(workdir_str)?;
        let dominant_language = Language::get_dominant_language(languages_stats.clone());

        Ok(Info {
            git_version: git_v,
            git_username: git_user,
            project_name: config.repository_name,
            current_commit: current_commit_info,
            version,
            creation_date,
            dominant_language,
            languages: languages_stats,
            authors,
            last_change,
            repo: config.repository_url,
            commits,
            repo_size,
            number_of_lines,
            license: project_license,
            custom_logo: logo,
            custom_colors: colors,
            disable_fields: disabled,
            bold_enabled: bold_flag,
            custom_image,
        })
    }

    fn get_configuration(repo: &Repository) -> Result<Configuration> {
        let config = repo.config().map_err(|_| Error::NoGitData)?;
        let mut remote_url = String::new();
        let mut repository_name = String::new();
        let mut remote_upstream: Option<String> = None;

        for entry in &config.entries(None).unwrap() {
            let entry = entry.unwrap();
            match entry.name().unwrap() {
                "remote.origin.url" => remote_url = entry.value().unwrap().to_string(),
                "remote.upstream.url" => remote_upstream = Some(entry.value().unwrap().to_string()),
                _ => (),
            }
        }

        if let Some(url) = remote_upstream {
            remote_url = url.clone();
        }

        let url = remote_url.clone();
        let name_parts: Vec<&str> = url.split('/').collect();

        if !name_parts.is_empty() {
            repository_name = name_parts[name_parts.len() - 1].to_string();
        }

        if repository_name.contains(".git") {
            let repo_name = repository_name.clone();
            let parts: Vec<&str> = repo_name.split(".git").collect();
            repository_name = parts[0].to_string();
        }

        Ok(Configuration {
            repository_name: repository_name.clone(),
            repository_url: name_parts.join("/"),
        })
    }

    fn get_current_commit_info(repo: &Repository) -> Result<CommitInfo> {
        let head = repo.head().map_err(|_| Error::ReferenceInfoError)?;
        let head_oid = head.target().ok_or(Error::ReferenceInfoError)?;
        let refs = repo.references().map_err(|_| Error::ReferenceInfoError)?;
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

    // Return first n most active commiters as authors within this project.
    fn get_authors(dir: &str, no_merges: bool, n: usize) -> Vec<(String, usize, usize)> {
        let mut args = vec!["-C", dir, "log", "--format='%aN'"];
        if no_merges {
            args.push("--no-merges");
        }

        let output = Command::new("git")
            .args(args)
            .output()
            .expect("Failed to execute git.");

        // create map for storing author name as a key and their commit count as value
        let mut authors = std::collections::HashMap::new();
        let mut total_commits = 0;
        let output = String::from_utf8_lossy(&output.stdout);
        for line in output.lines() {
            let commit_count = authors.entry(line.to_string()).or_insert(0);
            *commit_count += 1;
            total_commits += 1;
        }

        // sort authors by commit count where the one with most commit count is first
        let mut authors: Vec<(String, usize)> = authors.into_iter().collect();
        authors.sort_by_key(|(_, c)| *c);
        authors.reverse();

        // truncate the vector so we only get the count of authors we specified as 'n'
        authors.truncate(n);

        // get only authors without their commit count
        // and string "'" prefix and suffix
        let authors: Vec<(String, usize, usize)> = authors
            .into_iter()
            .map(|(author, count)| {
                (
                    author.trim_matches('\'').to_string(),
                    count,
                    count * 100 / total_commits,
                )
            })
            .collect();

        authors
    }

    fn get_git_info(dir: &str) -> (String, String) {
        let version = Command::new("git")
            .arg("--version")
            .output()
            .expect("Failed to execute git.");
        let version = String::from_utf8_lossy(&version.stdout).replace('\n', "");

        let username = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("config")
            .arg("--get")
            .arg("user.name")
            .output()
            .expect("Failed to execute git.");
        let username = String::from_utf8_lossy(&username.stdout).replace('\n', "");
        (version, username)
    }

    fn get_version(dir: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("describe")
            .arg("--abbrev=0")
            .arg("--tags")
            .output()
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);

        if output == "" {
            Ok("??".into())
        } else {
            Ok(output.to_string().replace('\n', ""))
        }
    }

    fn get_commits(dir: &str, no_merges: bool) -> Result<String> {
        let mut args = vec!["-C", dir, "rev-list", "--count"];
        if no_merges {
            args.push("--no-merges");
        }
        args.push("HEAD");

        let output = Command::new("git")
            .args(args)
            .output()
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);

        if output == "" {
            Ok("0".into())
        } else {
            Ok(output.to_string().replace('\n', ""))
        }
    }

    fn get_packed_size(dir: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("count-objects")
            .arg("-vH")
            .output()
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

    fn get_last_change(dir: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("log")
            .arg("-1")
            .arg("--format=%cr")
            .output()
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);

        if output == "" {
            Ok("??".into())
        } else {
            Ok(output.to_string().replace('\n', ""))
        }
    }

    fn get_creation_time(dir: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("log")
            .arg("--reverse")
            .arg("--pretty=oneline")
            .arg("--format=\"%ar\"")
            .output()
            .expect("Failed to execute git.");

        let output = String::from_utf8_lossy(&output.stdout);

        let output = match output.lines().next() {
            Some(creation_time) => creation_time.replace('"', ""),
            None => "??".into(),
        };
        Ok(output)
    }

    fn get_project_license(dir: &str) -> Result<String> {
        let detector = Detector::new()?;

        let mut output = fs::read_dir(dir)
            .map_err(|_| Error::ReadDirectory)?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(
                |entry| {
                    entry.is_file()
                        && entry
                            .file_name()
                            .map(OsStr::to_string_lossy)
                            .iter()
                            .any(|x| x.starts_with("LICENSE") || x.starts_with("COPYING"))
                }, // TODO: multiple prefixes, like COPYING?
            )
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
        let language = if let Language::Unknown = self.custom_logo {
            &self.dominant_language
        } else {
            &self.custom_logo
        };

        language.get_ascii_art()
    }

    fn colors(&self) -> Vec<Color> {
        let language = if let Language::Unknown = self.custom_logo {
            &self.dominant_language
        } else {
            &self.custom_logo
        };

        let colors = language.get_colors();

        let colors: Vec<Color> = colors
            .iter()
            .enumerate()
            .map(|(index, default_color)| {
                if let Some(color_num) = self.custom_colors.get(index) {
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
        let mut formatted_label = label.color(color);
        if self.bold_enabled {
            formatted_label = formatted_label.bold();
        }
        formatted_label
    }
}

fn write_buf<T: std::fmt::Display>(
    buffer: &mut String,
    title: &ColoredString,
    content: T,
) -> std::fmt::Result {
    writeln!(buffer, "{}{}", title, content)
}
