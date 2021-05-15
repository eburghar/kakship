# kakship and kaksip.kak

`kakship` is just a thin wrapper around [starship](https://starship.rs) to format the status line of
[kakoune](https://kakoune.org/) and is meant to be used with the included kakoune script `kakship.kak`.

![kakship prompt](kakship.png?raw=true "Kakship prompt")

# Mode of Operation

`kakship`

- override default config file path with `$kak_config/starship.toml`
- define the shell to none to disable escaping
- call `starship` with the given arguments, so you can use the same arguments than `starship` (show computed config,
  activate modules, measure modules timings, ...),
- transform ansi-codes to `kakoune` face definitions so it can be rendered correctly with all styles

It use an included [yew-ansi](https://github.com/siku2/yew-ansi) crate for parsing the ansi-codes to which I just
added support for `reversed` and `dimmed` ansi-codes that can be used in `starship` styles definitions.

The kakoune script call `kakship` when buffer is idle for all normal buffers As `starship` is really fast and format
a prompt in ms, the script doesn't need to be clever about when refreshing the status bar.

# Installation

1. Compile `kakship` with cargo and install it somewhere in your $PATH (for example `~/.local/bin`)

```sh
cargo install --path . --root ~/.local
```

2. Copy/modify the provided `starship.toml` to your `$kak_config` directory (usually `~/.config/kak/`)

Here is a minimal one with git information

```toml
add_newline = false
format = """\
${custom.kakcursor}\
${custom.kakmode}\
${custom.kakcontext}\
$directory\
${custom.kakfile}\
$git_branch\
$git_commit\
$git_state\
$git_status\
${custom.kaksession}"""

[git_branch]
format = '[ $branch]($style)'
style = 'fg:bright-red'
truncation_length = 9223372036854775807
truncation_symbol = '…'
only_attached = false
always_show_remote = true
disabled = false

[git_commit]
commit_hash_length = 7
format = '[ \($hash$tag\)]($style)'
style = 'fg:bright-yellow'
only_detached = true
disabled = false
tag_symbol = ' '
tag_disabled = false

[git_state]
rebase = 'REBASING'
merge = 'MERGING'
revert = 'REVERTING'
cherry_pick = 'CHERRY-PICKING'
bisect = 'BISECTING'
am = 'AM'
am_or_rebase = 'AM/REBASE'
style = 'fg:bright-yellow'
format = '\( [$state($progress_current/$progress_total)]($style)\)'
disabled = false

[git_status]
format = '( [\[$all_status$ahead_behind\]]($style) )'
style = 'fg:bright-yellow'
stashed = '\$'
ahead = '⇡${count}'
behind = '⇣${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
conflicted = '='
deleted = '✘'
renamed = '»'
modified = '!'
staged = '+'
untracked = '?'
disabled = false

[directory]
truncation_length = 3
truncate_to_repo = false
fish_style_pwd_dir_length = 0
use_logical_path = true
format = '[]($style)[ $read_only]($read_only_style)[$path]($style)'
style = 'bg:blue fg:black'
read_only_style = 'bg:blue fg:200'
disabled = false
read_only = '[]'
truncation_symbol = '…'

[directory.substitutions]
"~/.config" = " "

[custom.kakfile]
description = 'The current Kakoune buffername'
format = '[/$output ]($style)[]($style inverted) '
command = 'basename $kak_buffile'
when = 'true'
shell = ['sh']
style = 'bold bg:blue fg:black'
disabled = false

[custom.kaksession]
description = 'The current Kakoune session'
format = '[]($style)[  %val{client}:%val{session} ]($style)[]($style inverted)'
when = ''
shell = ['true']
style = 'bg:yellow fg:black'
disabled = false

[custom.kakcursor]
description = 'The current Kakoune cursor position'
format = '[%val{cursor_line}:%val{cursor_char_column}]($style)'
when = ''
shell = ['true']
style = 'fg:white'
disabled = false

[custom.kakmode]
description = 'The current Kakoune mode'
format = ' {{mode_info}}'
when = ''
shell = ['true']
disabled = false

[custom.kakcontext]
description = 'The current Kakoune context'
format = ' {{context_info}}'
when = ''
shell = ['true']
disabled = false
```

3. Put the `kakship.kak` script in your autoload path and add something like this to your kakrc

```
hook global ModuleLoaded kakship .* %{
	kakship-enable
}
```

You can use any plugin manager.

# Tips

You can use

```sh
kak_config="~/.config/kak" kakship timings
```

To check if your modeline is not overloaded.
