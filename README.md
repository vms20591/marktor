# marktor

A very simple application to bookmark tor hidden services.

**Note**

This is more of a learning exercise for me. So, if you find anything so naive in the code that could be improved, you could raise them as an issue. Don't expect any awesome Rust stand practices here yet! I myself am exploring it. Learning is one thing, but only when implementing with what you've learnt, shows if you've grasped any.

## Setting Up 

1. **Clone**

```
git clone <repo_url>
cd <repo>
```

2. **Build**

```
cargo build --release
``` 

3. **Run**

```
./target/release/marktor
```
or, better symlink the *executable*, like for example,

```
ln -s $PWD/target/release/marktor $HOME/.local/bin
```

## Usage

```
marktor [<location_to_file>] [<subcommand>] [<args>]
```

**Note**

For `create`, `delete` and `update`,

1. If *location_to_file* doesn't exist, it will be created!
2. If *location_to_file* is not given, a file named `marktor.json` would be created in the current directory!

1. **Create a bookmark**

```
marktor $HOME/marktor.json add TorProject http://idnxcnkne4qt76tg.onion
```

2. **Update a bookmark**

```
marktor $HOME/marktor.json update TorProject http://idnxcnkne4qt76tg.onion
```

**Note**

If a bookmark doesn't exist, it will be added!

3. **Get a bookmark**

```
marktor $HOME/marktor.json tor
marktor $HOME/marktor.json torproject
marktor $HOME/marktor.json TorProject
```

4. **List all bookmarks**

```
marktor $HOME/marktor.json list
```

5. **Delete a bookmark**

```
marktor $HOME/marktor.json delete TorProject
