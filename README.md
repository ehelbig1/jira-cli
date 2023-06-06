## Git Configuration

This repository uses it's own git configuration to define a standard git commit message template and pre-commit hook. The template can be found in the .gitmessage file and all git hooks are defined in the hooks/ directory. To enable this configuration run:

```bash
git config --local include.path ../.gitconfig
```

## Github Action

A release action is included in this template. It is a manual-dispatch job, so you'll have to manually start the action. This action creates a new Release, compiles the code for MacOS, Linux, and Windows, and publishes the binaries to the release.

Before this action will work, be sure to update Workflow permissions to Read and write permissions. Settings -> Actions -> General -> Workflow permissions