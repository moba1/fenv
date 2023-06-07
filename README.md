# fenv

new `env` command

## Getting Started

```bash
$ cat << EOF > /tmp/.env.sample
ENV_A=true
EOF
$ fenv -f /tmp/.env.sample ENV_B=true sh -c 'echo ENV_A=$ENV_A; echo ENV_B=$ENV_B'
ENV_A=true
ENV_B=true
```

## Usage

```
env command with dotenv

Usage: fenv [OPTIONS] [ARGUMENTS]...

Arguments:
  [ARGUMENTS]...  format: `[NAME=VALUE]... [COMMAND [ARG]...]`
                  environment set and comand arguments

Options:
  -f <DOTENV_FILES>         dotenv file path. If you want to use multiple files, specify `-f file1 -f file2 ...`
      --color <COLOR_MODE>  color mode [default: auto] [possible values: never, auto, always]
  -h, --help                Print help
  -V, --version             Print version
```

Specifying a `.env` file with `-f` will use [dotenvy](https://github.com/allan2/dotenvy) to read the `.env` file with the specified path.
Suppose you want to load `.env` with following contents:

```env
ENV_A=1
ENV_B=2
```

Assuming you have this file stored in `~/.env` and want to load it.
To achieve this, execute following command:

```bash
$ fenv -f ~/.env sh -c 'echo ENV_A=$ENV_A; echo ENV_B=$ENV_B'
ENV_A=1
ENV_B=2
```

`-f` may be specified multiple times.
In that case, files, specified by `-f` are read in the specified order.

```bash
# この場合、 ~/.env.1 -> ~/.env2 の順番に読み込まれる
fenv -f ~/.env.1 -f ~/.env.2
```

Alternatively, environment variables can be specified in same way as in conventional `env`.

```bash
$ fenv ENV=true sh -c 'echo ENV=$ENV'
ENV=true
```

It can also be combined.

```bash
$ cat << EOF > ~/.env
ENV_A=true
EOF
$ fenv -f ~/.env ENV_B=true sh -c 'echo ENV_A=$ENV_A; echo ENV_B=$ENV_B'
ENV_A=true
ENV_B=true
```

If specified no program, all currentyly set environment variables are displayed.

```bash
$ fenv
PWD=/home/example
PATH=...
...
```

## Documentation
### Symbol restrict

Environment variable name can use any character except `=`.
On the other hand, environment variable value can use any character.

```bash
$ fenv 🚀=✨
...
🚀=✨
...
```

### Colorized output
If you want to output environment variable, you set color mode by using `--color` option.

- `auto` mode: default mode. If you want to specify this mode, set `--color=auto`. If stdout is tty, environment variables output is colorized, and is not colorized otherwise.
- `never` mode: set `--color=never`. environment variables output is not colorized.
- `always` mode: set `--color=always`. environment variables output is always colorized.

```bash
# colorized outputs
fenv --color=always
```
