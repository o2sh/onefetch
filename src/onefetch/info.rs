use {
    crate::onefetch::{
        cli::Cli, cli_utils, commit_info::CommitInfo, deps, error::*, git_utils,
        language::Language, license::Detector, repo::Repo, text_color::TextColor,
    },
    colored::{Color, ColoredString, Colorize},
};

pub struct Info {
    git_username: String,
    git_version: String,
    repo_name: String,
    number_of_tags: usize,
    number_of_branches: usize,
    head_refs: CommitInfo,
    pending_changes: String,
    version: String,
    creation_date: String,
    languages: Vec<(Language, f64)>,
    dependencies: String,
    authors: Vec<(String, usize, usize)>,
    last_change: String,
    repo_url: String,
    number_of_commits: String,
    lines_of_code: usize,
    repo_size: String,
    license: String,
    pub dominant_language: Language,
    pub ascii_colors: Vec<Color>,
    pub text_colors: TextColor,
    pub config: Cli,
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.config.disabled_fields.git_info {
            let git_info_length = self.git_username.len() + self.git_version.len() + 3;

            writeln!(
                f,
                "{} {} {}",
                &self.bold(&self.git_username).color(self.text_colors.title),
                &self.bold("~").color(self.text_colors.tilde),
                &self.bold(&self.git_version).color(self.text_colors.title)
            )?;

            let separator = "-".repeat(git_info_length);

            writeln!(f, "{}", separator.color(self.text_colors.underline))?;
        }

        if !self.config.disabled_fields.project {
            let branches_tags_str = self.get_branches_and_tags_field();

            let project_str = &self.get_formatted_subtitle_label("Project");

            writeln!(
                f,
                "{}{} {}",
                project_str,
                self.repo_name.color(self.text_colors.info),
                branches_tags_str.color(self.text_colors.info)
            )?;
        }

        if !self.config.disabled_fields.head {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("HEAD"),
                &self.head_refs.to_string().color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.pending && !self.pending_changes.is_empty() {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Pending"),
                &self.pending_changes.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.version && !self.version.is_empty() {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Version"),
                &self.version.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.created && !self.creation_date.is_empty() {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Created"),
                &self.creation_date.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.languages && !self.languages.is_empty() {
            let title = if self.languages.len() > 1 { "Languages" } else { "Language" };

            let languages_str = self.get_language_field(title);

            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label(title),
                languages_str.color(self.text_colors.info)
            )?;
        }

        if !self.config.disabled_fields.dependencies && !self.dependencies.is_empty() {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Dependencies"),
                &self.dependencies.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.authors && !self.authors.is_empty() {
            let title = if self.authors.len() > 1 { "Authors" } else { "Author" };

            let author_str = self.get_author_field(title);

            write!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label(title),
                author_str.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.last_change && !self.last_change.is_empty() {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Last change"),
                &self.last_change.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.repo {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Repo"),
                &self.repo_url.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.commits {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Commits"),
                &self.number_of_commits.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.lines_of_code {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Lines of code"),
                &self.lines_of_code.to_string().color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.size && !self.repo_size.is_empty() {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("Size"),
                &self.repo_size.color(self.text_colors.info),
            )?;
        }

        if !self.config.disabled_fields.license && !self.license.is_empty() {
            writeln!(
                f,
                "{}{}",
                &self.get_formatted_subtitle_label("License"),
                &self.license.color(self.text_colors.info),
            )?;
        }

        if !self.config.no_color_palette {
            writeln!(
                f,
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

        Ok(())
    }
}

impl Info {
    pub fn new(config: Cli) -> Result<Info> {
        let repo = Repo::new(&config.repo_path)?;
        let workdir = repo.get_work_dir()?;
        let (repo_name, repo_url) = repo.get_name_and_url()?;
        let head_refs = repo.get_head_refs()?;
        let pending_changes = repo.get_pending_changes()?;
        let version = repo.get_version()?;
        let git_username = repo.get_git_username()?;
        let number_of_tags = repo.get_number_of_tags()?;
        let number_of_branches = repo.get_number_of_branches()?;
        let git_history = git_utils::get_git_history(&workdir, config.no_merges)?;
        let creation_date = git_utils::get_creation_date(&git_history)?;
        let number_of_commits = git_utils::get_number_of_commits(&git_history);
        let authors = git_utils::get_authors(&git_history, config.number_of_authors);
        let last_change = git_utils::get_date_of_last_commit(&git_history)?;
        let git_version = cli_utils::get_git_version()?;
        let repo_size = git_utils::get_packed_size(&workdir)?;
        let license = Detector::new()?.get_license(&workdir)?;
        let dependencies = deps::DependencyDetector::new().get_dependencies(&workdir)?;
        let (languages, lines_of_code) =
            Language::get_language_statistics(&workdir, &config.excluded)?;
        let dominant_language = Language::get_dominant_language(&languages);
        let ascii_colors = Language::get_ascii_colors(
            &config.ascii_language,
            &dominant_language,
            &config.ascii_colors,
            config.true_color,
        );
        let text_colors = TextColor::get_text_colors(&config.text_colors, &ascii_colors);

        Ok(Info {
            git_username,
            git_version,
            repo_name,
            number_of_tags,
            number_of_branches,
            head_refs,
            pending_changes,
            version,
            creation_date,
            languages,
            dependencies,
            authors,
            last_change,
            repo_url,
            number_of_commits,
            lines_of_code,
            repo_size,
            license,
            dominant_language,
            ascii_colors,
            text_colors,
            config,
        })
    }

    fn get_formatted_subtitle_label(&self, label: &str) -> ColoredString {
        let formatted_label = format!(
            "{}{} ",
            label.color(self.text_colors.subtitle),
            ":".color(self.text_colors.colon)
        );
        self.bold(&formatted_label)
    }

    fn bold(&self, label: &str) -> ColoredString {
        if self.config.no_bold {
            label.normal()
        } else {
            label.bold()
        }
    }

    fn get_author_field(&self, title: &str) -> String {
        let mut author_field = String::from("");

        let pad = title.len() + 2;

        for (i, (author_name, author_nbr_commits, autor_contribution)) in
            self.authors.iter().enumerate()
        {
            if i == 0 {
                author_field.push_str(&format!(
                    "{}{} {} {}\n",
                    autor_contribution.to_string().color(self.text_colors.info),
                    "%".color(self.text_colors.info),
                    author_name.to_string().color(self.text_colors.info),
                    author_nbr_commits.to_string().color(self.text_colors.info),
                ));
            } else {
                author_field.push_str(&format!(
                    "{:<width$}{}{} {} {}\n",
                    "",
                    autor_contribution.to_string().color(self.text_colors.info),
                    "%".color(self.text_colors.info),
                    author_name.to_string().color(self.text_colors.info),
                    author_nbr_commits.to_string().color(self.text_colors.info),
                    width = pad
                ));
            }
        }

        author_field
    }

    fn get_language_field(&self, title: &str) -> String {
        let mut language_field = String::from("");

        let pad = title.len() + 2;

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
            let formatted_number = format!("{:.*}", 1, language.1).color(self.text_colors.info);
            if cnt != 0 && cnt % 2 == 0 {
                language_field.push_str(&format!(
                    "\n{:<width$}{} ({} %) ",
                    "",
                    language.0.color(self.text_colors.info),
                    formatted_number,
                    width = pad
                ));
            } else {
                language_field.push_str(&format!(
                    "{} ({} %) ",
                    language.0.color(self.text_colors.info),
                    formatted_number
                ));
            }
        }
        language_field
    }

    fn get_branches_and_tags_field(&self) -> String {
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

        if tags_str.is_empty() && branches_str.is_empty() {
            String::new()
        } else if branches_str.is_empty() || tags_str.is_empty() {
            format!("({}{})", tags_str, branches_str)
        } else {
            format!("({}, {})", branches_str, tags_str)
        }
    }
}
