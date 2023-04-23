Onefetch is installed, then what?

### Usage

```sh
> onefetch /path/of/your/repo
 ```
 Or
 
```sh
> cd /path/of/your/repo
> onefetch
```

### Misc

By @spenserblack
```sh
# Runs `onefetch -a Assembly`, `onefetch -a C`, etc.
onefetch -l | tr "[:upper:] " "[:lower:]-" | while read line; do echo "$line"; onefetch -a $line; done;
```
By @quazar-omega

A script to put in your `.bashrc` - or `.zshrc` - to run onefetch whenever you open a shell into a repository or `cd` into a repository, making sure that it's different from the last one you were in:
```sh
# git repository greeter
last_repository=
check_directory_for_new_repository() {
	current_repository=$(git rev-parse --show-toplevel 2> /dev/null)
	
	if [ "$current_repository" ] && \
	   [ "$current_repository" != "$last_repository" ]; then
		onefetch
	fi
	last_repository=$current_repository
}
cd() {
	builtin cd "$@"
	check_directory_for_new_repository
}

# optional, greet also when opening shell directly in repository directory
# adds time to startup
check_directory_for_new_repository
```

By @TheSast

A fish adaptation of the previous script to put in your `$XDG_CONFIG_HOME/fish/functions/cd.fish`:
```fish
function cd -w='cd'
  builtin cd $argv || return
  check_directory_for_new_repository
end

function check_directory_for_new_repository
  set current_repository (git rev-parse --show-toplevel 2> /dev/null)
  if [ "$current_repository" ] && \
    [ "$current_repository" != "$last_repository" ]
    onefetch
  end
  set -x last_repository $current_repository
end
```

By @mbrehin
```sh
# Add Git alias for onefetch.
git config --global alias.project-summary '!which onefetch && onefetch'
```
