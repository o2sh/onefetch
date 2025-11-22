cli-input = Run as if onefetch was started in <input> instead of the current working directory

# Value name
cli-value-num = NUM
cli-value-field = FIELD
cli-value-regex = REGEX
cli-value-exclude = EXCLUDE
cli-value-type = TYPE
cli-value-separator = SEPARATOR
cli-value-string = STRING
cli-value-language = LANGUAGE
cli-value-when = WHEN
cli-value-image = IMAGE
cli-value-protocol = PROTOCOL
cli-value-value = VALUE
cli-value-format = FORMAT
cli-value-shell = SHELL

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

# TEXT FORMATTING
cli-text-heading = TEXT FORMATTING
cli-text-colors = 
    Changes the text colors (X X X...)
    
    Goes in order of title, ~, underline, subtitle, colon, and info
    
    For example:
    
    '--text-colors 9 10 11 12 13 14'
cli-text-iso-time = Use ISO 8601 formatted timestamps
cli-text-number-separator = Which thousands SEPARATOR to use
cli-text-no-bold = Turns off bold formatting

# ASCII
cli-ascii-heading = ASCII
cli-ascii-ascii-input = 
    Takes a non-empty STRING as input to replace the ASCII logo
   
    It is possible to pass a generated STRING by command substitution
   
    For example:
   
    '--ascii-input "$(fortune | cowsay -W 25)"'
cli-ascii-ascii-colors = Colors (X X X...) to print the ascii art
cli-ascii-ascii-language = Which LANGUAGE's ascii art to print
cli-ascii-true-color = 
    Specify when to use true color
   
    If set to auto: true color will be enabled if supported by the terminal

# VISUALS
cli-visuals-heading = VISUALS
cli-visuals-no-color-palette = Hides the color palette
cli-visuals-no-art = Hides the ascii art or image if provided
cli-visuals-nerd-fonts = 
    Use Nerd Font icons
   
    Replaces language chips with Nerd Font icons


# DEVELOPER
cli-dev-heading = DEVELOPER
cli-dev-output = Outputs Onefetch in a specific format
cli-dev-completion = If provided, outputs the completion file for given SHELL

# OTHER
cli-other-heading = OTHER
cli-other-languages = Prints out supported languages
cli-other-package-managers = Prints out supported package managers
