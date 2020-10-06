# How to Contribute

We welcome any kind of contributions. Whether you are reporting a bug, coding a feature or correcting a typo. Every effort counts; and all contributions are greatly appreaciated.

Another great way to contribute is to actually use rust-starter for your Open Source or Commercial projects. If you do, make sure to open an issue with a link to your project.

### Testing Code

To run all tests, run the following command.

```sh
cargo test --all

```

Alternatively, if you have [`just`](https://github.com/casey/just) installed you can run the prebuilt recipes. *Not* using `just` is perfectly fine as well, it simply bundles commands automatically.

For example, to test the code, as above simply run:

```sh
$ just run-tests
```

### Linting Code

### Debugging Code

### Commit Messages

Use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) changelog format so the changelog can be updated automatically using [clog](https://github.com/clog-tool/clog-cli)

 * Please format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature of an existing API
    - `imp`  - An improvement to an existing feature/API
    - `perf` - A performance improvement
    - `docs` - Changes to documentation only
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
    - `examples` - Changes to existing example, or a new example
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Parenthesis can be omitted if you are opting not to use the `COMPONENT`.

### Tests and Documentation

1. Create tests for your changes
2. **Ensure the tests are passing.** Run the tests (`cargo test --all`), alternatively `just run-tests` if you have `just` installed.
3. **Optional** Run the lints.
4. Ensure your changes contain documentation if adding new APIs or features.

### Preparing the PR


1. `git rebase` into concise commits and remove `--fixup`s or `wip` commits (`git rebase -i HEAD~NUM` where `NUM` is number of commits back to start the rebase)
2. Push your changes back to your fork (`git push origin $your-branch`)
3. Create a pull request against `master`! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)

### Goals

* Remain backwards compatible when possible.
    Backward compatibility is not critical since projects do not depend on Rust Starter. But it's a good idea to have stable features and usage.
* Reduce dependencies.
* Follow best practices.
* Ease of use and customization.
