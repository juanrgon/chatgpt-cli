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

Your messages in each terminal window are saved to `~/.chatgpt/{OS boot time}/{terminal pid}/chatlog.json`. This means you can ask follow-up questions in a terminal window and start a new conversation by opening a new window.

## Settings

### Use a different model like GPT-4

By default, the CLI uses the `gpt-3.5-turbo` model.

However, you can use a different model by setting the `CHATGPT_CLI_MODEL` environment variable:

```
export CHATGPT_CLI_MODEL=gpt-4
```

NOTE: The gpt-4 model is not yet available to everyone. You can join the wailist [here](https://openai.com/waitlist/gpt-4-api).

### Increase the request timeout

By default, the CLI will wait 10 seconds for a response from the API. You can increase this timeout by setting the `CHATGPT_CLI_TIMEOUT` environment variable:

```
export CHATGPT_CLI_REQUEST_TIMEOUT_SECS=600
```
