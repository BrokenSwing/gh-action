# gh-action

This repository contains the source code the for the `gh-action` [github CLI](https://cli.github.com/) extension.

## Install the extension

You can install this extension using the following command:
```
gh extension install brokenswing/gh-action
```

If you already installed the extension and want to upgrade it to the latest version, run:
```
gh extension upgrade action
```

## Usage

### Create a new action `gh action new --help`

This extension allows you to create a new github action easily either:
* as a standalone if ran outside of a git repository
* as a [local action](https://docs.github.com/en/actions/learn-github-actions/finding-and-customizing-actions?learn=getting_started&learnProduct=actions#adding-an-action-from-the-same-repository) if ran inside a git repository