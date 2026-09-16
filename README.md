# sesh

quickly launch predefined window layouts and programs by project type.  
fuzzy find a project and attach to or create the tmux session with your window layout running defined commands.

## install

```sh
cargo install sesh-tmux
```

the crate is published as `sesh-tmux` (the name `sesh` was taken), but it
installs a binary called `sesh`.

requires `tmux` on your `PATH`. unix only (linux, macos).

## usage

```sh
sesh
```

or bind it to ctrl+f in your shell rc file.

zsh:

```sh
bindkey -s ^f "sesh\n"
```

bash:

```sh
bind '"\C-f": "sesh\n"'
```

then:

1. pick a project from the fuzzy finder
2. if it's the first time opening the project, you will be prompted to pick one of the layouts defined in `~/.config/sesh/config.toml` (the choice is remembered in `~/.config/sesh/projects.toml`)
3. you will be attached to the session

already inside tmux? `sesh` switches the session instead of nesting.

## configuration

`sesh` reads `~/.config/sesh/config.toml`, creating an empty one on first run.

_refactor in progress_

## license

[MIT](LICENSE)
