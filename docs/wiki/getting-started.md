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

By [**@spenserblack**](https://github.com/spenserblack)

```sh
# Runs `onefetch -a Assembly`, `onefetch -a C`, etc.
onefetch -l | tr "[:upper:] " "[:lower:]-" | while read line; do echo "$line"; onefetch -a $line; done;
```

### Automatic repo detection and running

If you want to automate the detection and running of `onefetch` every time you `cd` into a repository you can leverage one of the methods below:

#### 1. Bash / Zsh

By [**@quazar-omega**](https://github.com/quazar-omega)

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

#### 2. Fish

By [**@TheSast**](https://github.com/TheSast)

A fish adaptation of the previous script, run it once in your shell to save it:

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
  set -gx last_repository $current_repository
end

funcsave cd
funcsave check_directory_for_new_repository
```

#### 3. CMD

By [**@mataha**](https://github.com/mataha)

An adaptation of the above snippet suited for Windows's `cmd.exe`,
specifically for inclusion in AutoRun scripts or DOSKEY macrofiles:

```bat
@set LAST_REPOSITORY=

@doskey cd = ( ^
    for %%^^^^ in ("") do @for /f "delims=" %%i in (^^""$*%%~^^"^") do @( ^
        if "%%~i"=="" ( ^
            if defined USERPROFILE ( ^
                if /i not "%%CD%%"=="%%USERPROFILE%%" ( ^
                    chdir /d "%%USERPROFILE%%" ^&^& set "OLDPWD=%%CD%%" ^
                ) ^
            ) else (call) ^
        ) else if "%%~i"=="-" ( ^
            if defined OLDPWD ( ^
                if /i not "%%CD%%"=="%%OLDPWD%%" ( ^
                    chdir /d "%%OLDPWD%%"      ^&^& set "OLDPWD=%%CD%%" ^
                ) ^
            ) else (call) ^
        ) else ( ^
            if defined CD ( ^
                if /i not "%%CD%%"=="%%~fi" ( ^
                    chdir /d %%~fi             ^&^& set "OLDPWD=%%CD%%" ^
                ) ^
            ) else (call) ^
        ) ^
    ) ^
) ^&^& for /f "delims=" %%r in ('git rev-parse --show-toplevel 2^^^>nul') do @( ^
    if not "%%~r"=="%%LAST_REPOSITORY%%" ( ^
        onefetch ^
    ) ^& set "LAST_REPOSITORY=%%~r" ^
)
```

#### 4. Powershell

By [**@kiapanahi**](https://github.com/kiapanahi)

An adaptation of the above snippet suited for `Powershell`. Put this script in the `$PROFILE`.

```pwsh
# git repository greeter

# Set the console output encoding to UTF-8, so that special characters are displayed correctly when piping to Write-Host
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$global:lastRepository = $null

function Check-DirectoryForNewRepository {
    $currentRepository = git rev-parse --show-toplevel 2>$null
    if ($currentRepository -and ($currentRepository -ne $global:lastRepository)) {
        onefetch | Write-Host
    }
    $global:lastRepository = $currentRepository
}

function Set-Location {
    Microsoft.PowerShell.Management\Set-Location @args  
    Check-DirectoryForNewRepository
}

# Optional: Check the repository also when opening a shell directly in a repository directory
# Uncomment the following line if desired
#Check-DirectoryForNewRepository
```

### Git alias

By [**@mbrehin**](https://github.com/mbrehin)

You can also add git alias to run onefetch during your git workflows

```sh
# Add Git alias for onefetch.
git config --global alias.project-summary '!which onefetch && onefetch'
```
