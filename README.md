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

Finally, you'll need a OPENAI API key (you can get one [here](https://platform.openai.com/account/api-keys)), and you'll need to export your API Key as an environment variable:


```
export OPENAI_API_KEY=<your api key>
```


Then you can start a conversation with ChatGPT:

```
chatgpt what is 2 + 2
```

You can also send multiline messages:

```
chatgpt '''
    Make this sentence more sophisticated:

    I like to eat pizza
    '''
```

Your messages in each terminal window is saved `~/.chatgpt/{pid}/chatlog.json`. This means you can ask follow-up questions in a terminal window and start a new conversation by opening a new window.
