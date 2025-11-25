
cli-about = Command-line Git information tool
cli-usage-header = Usage
cli-arguments-header = Arguments
cli-arguments-input = Run as if onefetch was started in <input> instead of the current working directory
cli-options-header = Options
cli-options-help = Print help { $short ->
    [true] (see more with '--help')
    *[false] (see a summary with '-h')
}
cli-options-version = Print version

# Value name
cli-value-input = INPUT
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
cli-info-disabled_fields = Allows you to disable FIELD(s) from appearing in the output
cli-info-no_title = Hides the title
cli-info-number_of_authors-short = Maximum NUM of authors to be shown (default: {$def})
cli-info-number_of_authors-long = 
    Maximum NUM of authors to be shown 
    
    (default: {$def})
cli-info-number_of_languages-short = Maximum NUM of languages to be shown (default: {$def})
cli-info-number_of_languages-long = 
    Maximum NUM of languages to be shown 

    (default: {$def})
cli-info-number_of_file_churns-short = Maximum NUM of file churns to be shown (default: {$def})
cli-info-number_of_file_churns-long = 
    Maximum NUM of file churns to be shown 
    
    (default: {$def})
cli-info-churn_pool_size-short = Minimum NUM of commits from HEAD used to compute the churn summary
cli-info-churn_pool_size-long =
    Minimum NUM of commits from HEAD used to compute the churn summary

    By default, the actual value is non-deterministic due to time-based computation
    and will be displayed under the info title "Churn (NUM)"
cli-info-exclude = Ignore all files & directories matching EXCLUDE
cli-info-no_bots = Exclude [bot] commits. Use <REGEX> to override the default pattern
cli-info-no_merges = Ignores merge commits
cli-info-email = Show the email address of each author
cli-info-http_url = Display repository URL as HTTP
cli-info-hide_token = Hide token in repository URL
cli-info-include_hidden = Count hidden files and directories
cli-info-tipe-short = Filters output by language type (default: ${def}) (possible values: {$pos})
cli-info-tipe-long = 
    Filters output by language type 
    
    (default: ${def}) 
    (possible values: {$pos})

# TEXT FORMATTING
cli-text-heading = TEXT FORMATTING
cli-text-colors = 
    Changes the text colors (X X X...)
    
    Goes in order of title, ~, underline, subtitle, colon, and info
    
    For example:
    
    '--text-colors 9 10 11 12 13 14'
cli-text-iso_time = Use ISO 8601 formatted timestamps
cli-text-number_separator = Which thousands SEPARATOR to use
cli-text-no_bold = Turns off bold formatting

# ASCII
cli-ascii-heading = ASCII
cli-ascii-ascii_input = 
    Takes a non-empty STRING as input to replace the ASCII logo
   
    It is possible to pass a generated STRING by command substitution
   
    For example:
   
    '--ascii-input "$(fortune | cowsay -W 25)"'
cli-ascii-ascii_colors = Colors (X X X...) to print the ascii art
cli-ascii-ascii_language = Which LANGUAGE's ascii art to print
cli-ascii-true_color = 
    Specify when to use true color
   
    If set to auto: true color will be enabled if supported by the terminal

# IMAGE
cli-image-heading = IMAGE
cli-image-image = Path to the IMAGE file
cli-image-image_protocol = Which image PROTOCOL to use
cli-image-color_resolution = VALUE of color resolution to use with SIXEL backend

# VISUALS
cli-visuals-heading = VISUALS
cli-visuals-no_color_palette = Hides the color palette
cli-visuals-no_art = Hides the ascii art or image if provided
cli-visuals-nerd_fonts = 
    Use Nerd Font icons
   
    Replaces language chips with Nerd Font icons


# DEVELOPER
cli-dev-heading = DEVELOPER
cli-dev-output = Outputs Onefetch in a specific format
cli-dev-completion = If provided, outputs the completion file for given SHELL

# OTHER
cli-other-heading = OTHER
cli-other-languages = Prints out supported languages
cli-other-package_managers = Prints out supported package managers
