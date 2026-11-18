# Decisions

Choices made so far, with reasons. Each one can be revisited.

## Product

| # | Decision | Reason |
| --- | --- | --- |
| D1 | Co-authors (`Co-authored-by` trailers) are always accounted for, and percentages are in terms of contributions, not of total commits. For example, a commit by Alice co-authored by Bob is one contribution each: 50% Alice, 50% Bob. | Co-authors did part of the work; leaving them out hides contributors, notably in squash-merged and pair-programmed history. Shares of contributions add up to 100%, so the percentages still read as a split of the work. |
