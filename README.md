Talk to ChatGPT from your terminal.


## Quickstart

First you'll need to install the CLI:


```
cargo install chatgpt-cli
```

Then, you'll need to make sure your cargo bin directory is in your path. You can do this by adding the following to your `~/.bashrc` or `~/.zshrc`:

```
export PATH="$PATH:$HOME/.cargo/bin"
```

Then you can start a conversation with ChatGPT:

```
chatgpt What is 2 + 2
```

You can also send multiline messages:

```
chatgpt-cli '''
    Make this sentence more sophisticated:

    I like to eat pizza
```

Your messages in each terminal window is saved `~/.chatgpt/{pid}/chatlog.json`. This means you can ask follow-up questions in a terminal window and start a new conversation by opening a new window.
