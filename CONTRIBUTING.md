# Contributing Guidelines

Thank you for your interest in contributing to onefetch! Whether it's a bug report, new feature, correction, or additional 
documentation, we greatly value feedback and contributions from our community.

Please read through this document before submitting any issues or pull requests to ensure we have all the necessary 
information to effectively respond to your bug report or contribution.

## Reporting Bugs/Feature Requests

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

### Project-specific notes

- Please ensure your changes are formatted according to `cargo fmt`.
- Do check for linting errors with `cargo clippy`. If you're having trouble with this, feel free to ask for help.
- Documenting your changes in `CHANGELOG.md` (in the Unreleased section) would be awesome, but is not required.
- If you can, try to write some tests for your change (if it addresses a bug) or feature. Again, feel free to ask for help. Our CI will run these tests to ensure your code never breaks with future changes.

## Finding contributions to work on

Looking at the existing issues is a great way to find something to contribute on. As our projects, by default, use the default GitHub issue labels (enhancement/bug/duplicate/help wanted/invalid/question/wontfix), looking at any ['help wanted'](https://github.com/o2sh/onefetch/labels/help%20wanted) issues is a great place to start. 

## Code of Conduct

This project has adopted the Contributor Covenant Code of Conduct, version 2.0. It can be found in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Contributors and community members are expected to adhere to this code of conduct to create a welcoming environment.

## Licensing

See the [LICENSE](https://github.com/o2sh/onefetch/blob/master/LICENSE) file for our project's licensing. We will ask you confirm the licensing of your contribution.

