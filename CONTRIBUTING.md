Contributing
===

Thanks for considering contibuting to Ruperf! Be sure to read the `whitepaper.tex` document before to get an even better idea of the project.

Before sending a Pull Request
---

1. All source code files must be well documented. This includes brief descriptions of the following:
- Overview of the file itself, located at the top. Use `//!`.
- All data structures, methods, types, etc. Use `///`.
- Referenced code must be cited; with a brief description and the source in a comment.

2. Configure git hooks for formatting and other pre-commit checks using clippy and `cargo format`.

in the root directory, after pulling from upstream do:

```
git config core.hooksPath "./git_hooks"
```

3. Make sure your pull request is directed to the `dev` branch

Code Review
---

| Changes | Minumum to Merge |
|:------------- | :-------------: |
|Small  | Single review  |
|Major  |Full team      |

- Review must be from another person
- Submitter must rebase and merge once reviewed.

More ways to contribute
---

- Composing tutorials
- Writing blog posts
- Submitting bug reports
- Sharing and improving ideas

References
---

[Using git hooks][1]

[1]: https://git-scm.com/docs/githooks
