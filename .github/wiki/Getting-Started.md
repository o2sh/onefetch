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
By @Quazar_omega

A little function to put in `.bashrc` to run onefetch whenever you cd into a repo, making sure that it's different from the last one you were in:
```sh
LAST_REPO=""
cd() { 
    builtin cd "$@";
    git rev-parse 2>/dev/null;

    if [ $? -eq 0 ]; then
        if [ "$LAST_REPO" != $(basename $(git rev-parse --show-toplevel)) ]; then
        onefetch
        LAST_REPO=$(basename $(git rev-parse --show-toplevel))
        fi
    fi
}
```

By @mbrehin
```sh
# Add Git alias for onefetch.
git config --global alias.project-summary '!which onefetch && onefetch'
```
