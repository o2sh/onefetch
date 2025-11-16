cli-input = Run as if onefetch was started in <input> instead of the current working directory

# Value name
cli-value-num = NUM
cli-value-field = FIELD
cli-value-regex = REGEX
cli-value-exclude = EXCLUDE

# INFO
cli-info-heading = INFO
cli-info-disabled-fields = Allows you to disable FIELD(s) from appearing in the output
cli-info-no-title = Hides the title
cli-info-number-of-authors = Maximum NUM of authors to be shown
cli-info-number-of-languages = Maximum NUM of languages to be shown
cli-info-number-of-file-churns = Maximum NUM of file churns to be shown
cli-info-churn-pool-size = 
    Minimum NUM of commits from HEAD used to compute the churn summary

    By default, the actual value is non-deterministic due to time-based computation
    and will be displayed under the info title "Churn (NUM)"
cli-info-exclude = Ignore all files & directories matching EXCLUDE
cli-info-no-bots = Exclude [bot] commits. Use <REGEX> to override the default pattern
cli-info-no-merges = Ignores merge commits
cli-info-email = Show the email address of each author
cli-info-http-url = Display repository URL as HTTP
cli-info-hide-token = Hide token in repository URL
cli-info-include-hidden = Count hidden files and directories
cli-info-type = Filters output by language type