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

Your messages in each terminal window are saved to `{data_dir}/{OS boot time}/{terminal pid}/chatlog.json`. This means you can ask follow-up questions in a terminal window and start a new conversation by opening a new window.

For example on MacOS the data path is `$HOME/Library/Application Support/`. For other platforms, refer to [the directories documentation](https://github.com/dirs-dev/directories-rs#projectdirs).

## Settings

The CLI is configured either by environment variables or the config file.

To find out where the config file is expected to be, run `chatgpt -p`. Create that file and populate it with the below values to use it.

The CLI will always use CLI flags over environment variables over the config file.

### Specify the API key

You can specify the API key either via an environment variable or in the `config.json` file:

```
export OPENAI_API_KEY=<your api key>
```

`config.json`
```
{
  "openai_api_key": "<your api key>"
}
```

### Use a different model like GPT-4

By default, the CLI uses the `gpt-3.5-turbo` model.

However, you can use a different model by passing the `--model` flag:

```
chatgpt --model=gpt-4 Complete this phrase: "Ravioli ravioli, give me the..."
```

You can also change the default model by setting the `CHATGPT_CLI_MODEL` environment variable:

```
export CHATGPT_CLI_MODEL=gpt-4
```

Or add this to the config file:

`config.json`
```
{
  "model": "<gpt-4>"
}
```

NOTE: The gpt-4 model is not yet available to everyone. You can join the waitlist [here](https://openai.com/waitlist/gpt-4-api).

### Increase the request timeout

By default, the CLI will wait 120 seconds for a response from the API. You can increase this timeout by setting the `CHATGPT_CLI_REQUEST_TIMEOUT_SECS` environment variable:

```
export CHATGPT_CLI_REQUEST_TIMEOUT_SECS=600
```
