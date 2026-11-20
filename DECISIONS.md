# Decisions

Choices made so far, with reasons. Each one can be revisited.

## Product

| # | Decision | Reason |
| --- | --- | --- |
| D1 | Co-authors (`Co-authored-by` trailers) are always accounted for, and percentages are in terms of contributions, not of total commits. For example, a commit by Alice co-authored by Bob is one contribution each: 50% Alice, 50% Bob. | Co-authors did part of the work; leaving them out hides contributors, notably in squash-merged and pair-programmed history. Shares of contributions add up to 100%, so the percentages still read as a split of the work. |
| D2 | Onefetch is an offline tool: it never touches the network, so it neither clones remote repositories ([#420](https://github.com/o2sh/onefetch/pull/420)) nor fetches stats such as stars or forks from GitHub or GitLab ([#351](https://github.com/o2sh/onefetch/issues/351)). | Staying offline keeps onefetch fast, usable without a connection, and simple. Host stats would only help repositories on a few hosts, yet every run would pay for the network call. Running onefetch on a remote repository is already a one-line script: shallow-clone it, then run onefetch. |
| D3 | Command-line arguments take precedence over the config file: when an option is set in both, the argument wins. | Users can depart from their saved settings for a single run without editing the config file. |

## Performance

| # | Decision | Reason |
| --- | --- | --- |
| P1 | Churn runs on its own thread alongside the commit walk and covers as many commits as it can diff before the walk finishes. `--churn-pool-size` sets a minimum. | Diffing is far slower than walking, so diffing every commit would make large repositories slow. |
