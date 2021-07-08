Contributing
===

Thanks for considering contibuting to Ruperf! Be sure to read the `whitepaper.tex` document before to get an even better idea of the project.

Before sending a Pull Request
---

1. Make sure all sourced files must be well documented. brief descriptions include:
- description at the top of file
- Data structures and methods

2. Configure git hooks for formatting and other pre-commit checks using clippy and `cargo format`.

in the root directory, after pulling from upstream do
```
git config core.hooksPath "./git_hooks"
```


3. Make sure your pull request is directed to the `dev` branch

For C code
---

- File comments must include `@author` and `@date.` use `//!`.

- Function comments must be included in .h and .c files; require a description of arguments and purpose. use `/*`

- All other comments use `//`

- The amount of characters in a single-line should not exceed 60-70 characters in length.

resources

- [Information on configuring `clang-format`][1]
- [Doxygen C standards][2]
- [General C guidelines][3]
- [Using git hooks][4]

Code Review
---

- Minumum one persone other than yourselve must review code, For major changes, whole team must review.

- Must rebase and merge once reviewed.

More ways to contribute
---

- Composing tutorials
- Writing blog posts
- Submitting bug reports
- Sharing and improving ideas


[1]: https://www.kernel.org/doc/html/latest/process/clang-format.html

[2]: https://www.doxygen.nl/manual/docblocks.html

[3]: https://www.cs.swarthmore.edu/~newhall/unixhelp/c_codestyle.html

[4]: https://git-scm.com/docs/githooks