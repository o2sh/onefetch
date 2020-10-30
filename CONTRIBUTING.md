Thank you for your interest in contributing to onefetch! Whether it's a bug report, new feature, correction, or additional 
documentation, we greatly value feedback and contributions from our community.

Please read through this document before submitting any issues or pull requests to ensure we have all the necessary 
information to effectively respond to your bug report or contribution.

# Contributing Guidelines

* [Reporting Bugs / Feature Requests](#reporting-bugs--feature-requests)
* [Contributing via Pull Requests](#contributing-via-pull-requests)
    * [Finding contributions to work on](#finding-contributions-to-work-on)
    * [Adding support for a new language](#adding-support-for-a-new-language)
      * [Ascii logo](#ascii-logo)
    * [Project-specific notes](#project-specific-notes)

## Reporting Bugs / Feature Requests

We welcome you to use the GitHub issue tracker to report bugs or suggest features.

When filing an issue, please check [existing open](https://github.com/o2sh/onefetch/issues), or [recently closed](https://github.com/o2sh/onefetch/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aclosed%20), issues to make sure somebody else hasn't already 
reported the issue. Please try to include as much information as you can. Details like these are incredibly useful:

* A reproducible test case or series of steps
* The version of our code being used
* Any modifications you've made relevant to the bug
* Anything unusual about your environment or deployment

## Contributing via Pull Requests

Contributions via pull requests are much appreciated. Before sending us a pull request, please ensure that:

1. You are working against the latest source on the *master* branch.
2. You check existing open, and recently merged, pull requests to make sure someone else hasn't addressed the problem already.
3. You open an issue to discuss any significant work - we would hate for your time to be wasted.

To send us a pull request, please:

1. Fork the repository.
2. Modify the source; please focus on the specific change you are contributing. If you also reformat all the code, it will be hard for us to focus on your change.
3. Ensure local tests pass.
4. Commit to your fork using clear commit messages.
5. Send us a pull request, answering any default questions in the pull request interface.
6. Pay attention to any automated CI failures reported in the pull request, and stay involved in the conversation.

GitHub provides additional document on [forking a repository](https://help.github.com/articles/fork-a-repo/) and 
[creating a pull request](https://help.github.com/articles/creating-a-pull-request/).

### Finding contributions to work on

Looking at the existing issues is a great way to find something to contribute on. As our projects, by default, use the default GitHub issue labels (enhancement/bug/duplicate/help wanted/invalid/question/wontfix), looking at any ['help wanted'](https://github.com/o2sh/onefetch/labels/help%20wanted) issues is a great place to start.

### Adding support for a new language

First, you have to make sure that the language you want to add is supported by tokei ([supported languages](https://github.com/XAMPPRocky/tokei#supported-languages))

Adding support for a new Language requires adding a new entry in the `define_language!` macro in [language.rs](https://raw.githubusercontent.com/o2sh/onefetch/master/src/language.rs).

**Example**:

` { CSharp, "csharp.ascii", "C#", define_colors!( [Color::Blue, Color::Magenta] ), "c#" }, `

The first item `CSharp` corresponds to the name of the language as defined in tokei. The second item `csharp.ascii` is the name of the file containing the ascii logo, this file has to be placed in the _./resources_ folder (more info below). The third item `C#` is the display name. Then we have the colors used to customize the look of the ascii logo when displayed to the screen. The last item `c#` is required only if the Enum name  `CSharp` doesn't match the display name `C#`.

#### Ascii logo

```
{4}   _{1} _  _ 
{4} _|_{1}(_|/ \
{0} o{4}| {1} _|\_/

{0}    /\
{0}   /  \
{0}  |    |
{0}  |{2}NASA{0}|
{0}  |    |
{0}  |    |
{0}  |    |
{0} '      `
{0} |      |
{0} |      |
{0} |______|
{3}  '-`'-`   .
{3}  / . \'\ . .'
{3} ''( .'\.' ' .;'
{3}'.;.;' ;'.;' ..;;'
```

Remarks:
 - Your ascii logo's dimensions must fall below `25*45` (height\*width). The CI will fail otherwise.
 - You can use `{N}` to color the ascii which will utilize the vec! of colors defined in `define_language!`
 - Make sure to trim any unnecessary trailing whitespaces
 - See example here [Convert image to ASCII art](https://github.com/o2sh/onefetch/wiki/image-to-ascii)
 - You must always provide an array of basic colors
 - Optionally, you may provide true colors in a second array of colors separated from the first array by a colon
 - One last approach allows you to define colors using the Colors structure itself
 - Make sure if you use true colors that the number of true colors matches the number of basic colors
 - For example, the following are equivalent:
```
    { CSharp, "csharp.ascii", "C#", define_colors!( [Color::Blue, Color::Magenta] ), "c#" },
    { CSharp, "csharp.ascii", "C#", define_colors!( [Color::Blue, Color::Magenta] : [Color::TrueColor{ r:0, g:255, b:255 }, Color::TrueColor{ r:255, g:0, b:255 } ] ), "c#" },
    { CSharp, "csharp.ascii", "C#", define_colors!( Colors { basic_colors: vec![Color::Blue, Color::Magenta] , true_colors: Some(vec![Color::TrueColor{ r:0, g:255, b:255 }, Color::TrueColor{ r:255, g:0, b:255 } ] ) } ), "c#" },
```

### Project-specific notes

- Please ensure your changes are formatted according to `cargo fmt`.
- Do check for linting errors with `cargo clippy`. If you're having trouble with this, feel free to ask for help.
- Documenting your changes in `CHANGELOG.md` (in the Unreleased section) would be awesome, but is not required.
- If you can, try to write some tests for your change (if it addresses a bug) or feature. Again, feel free to ask for help. Our CI will run these tests to ensure your code never breaks with future changes.

