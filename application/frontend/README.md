# loremaster - Frontend

## Setup

The frontend style libraries have been excluded from version control.
You must download them and place them in the styles/library folder.

Current style libraries:

- [Bootstrap 5.1](https://getbootstrap.com/docs/5.1/getting-started/download/)

## Local Debugging

This command will compile the source code and create a ```dist/``` folder with the compiled contents to be served.

```bash
trunk build
```

This command also compiles the source code, but will continue to monitor changes in the code and auto-rebuild when a file is changed.

```bash
trunk watch
```

Trunk can also serve the content with the following command, but I don't see a use for this at the moment besides if the backend isn't avaialble for some reason.

```bash
trunk serve
```
