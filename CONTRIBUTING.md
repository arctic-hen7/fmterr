# Contributing

First off, thanks so much for taking the time to contribute to fmterr, it's greatly appreciated!

## I just want to propose something

If you just want to let us know about a bug, or propose a new feature, please [open an issue](https://github.com/arctic-hen7/fmterr/issues/new/choose) on this repository. We'll take a look as soon as possible!

## I want to help

That's fantastic! You can check out the [roadmap](./README.md#Roadmap) or the [issues](https://github.com/arctic-hen7/fmterr/issues) to see what's currently needing to be done. If you want to work on something that's not on there, please [file an issue](https://github.com/arctic-hen7/fmterr/issues/new/choose) and we'll take a look it as soon as possible!

## How do I contribute?

Contributing to a project on Github is pretty straight forward. If this is you're first time contributing to a project, all you need to do is fork this repository to your own GitHub account, add then change the code you want to (usually on your local machine, you'd pull your fork down). Commit your changes as necessary, and when you're done, submit a pull request on this repository and we'll review it as soon as possible!

Make sure your code doesn't break anything existing, that all tests pass, and, if necessary, add tests so your code can be confirmed to work automatically.

After you've submitted a pull request, a maintainer will review your changes. Unfortunately, not every pull request will be merged, but we'll try to request changes so that your pull request can best be integrated into the project.

## Building and Testing

Fmterr uses [Bonnie](https://github.com/arctic-hen7/bonnie) for command aliasing (you can install it with `cargo install bonnie`). You can run `bonnie help` to see all available commands, but here are the ones you'll use the most:

-   `bonnie test` -- runs tests, watching for changes
-   `bonnie dev example <example>` -- runs the given example, watching for changes

Note that you'll also need `cargo watch` installed (`cargo install cargo-watch`) to support these commands.

## Documentation

If the code you write needs to be documented in, the README, the book, or elsewhere, please do so! Also, **please ensure your code is commented**, it makes everything so much easier.

## Committing

We use the Conventional Commits system, but you can commit however you want. Your pull request will be squashed and merged into a single compliant commit, so don't worry about this!
