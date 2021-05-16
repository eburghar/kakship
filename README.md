# kakship and kakship.kak

`kakship` is just a thin wrapper around [starship](https://starship.rs) to format the status line of
[kakoune](https://kakoune.org/) and is meant to be used with the included kakoune script `kakship.kak`.

![kakship prompt](kakship.png?raw=true "Kakship prompt")

## Operating mode

`kakship`

- overrides override the default config file path with `$kak_config/starship.toml`,
- defines the shell to none to disable escaping,
- forward the given arguments to `starship`,
- transforms ansi-codes to kakoune face definitions when called with `prompt` argument.

It uses an included [yew-ansi](https://github.com/siku2/yew-ansi) crate for parsing the ansi-codes to which I just
added support for `reversed` and `dimmed` ansi-codes that can be used in `starship` styles definitions.

The kakoune script call `kakship` when buffer is idle for all normal windows. As `starship` is really fast and format
a prompt in ms, the script doesn't need to be clever about when refreshing the status bar.

## Installation

### Manual

1. Compile `kakship` with cargo and install it somewhere in your $PATH (for example `~/.local/bin`)

```sh
cargo install --force --path . --root ~/.local
```

2. Copy/modify the provided [starship.toml](starship.toml) to your `$kak_config` directory (usually `~/.config/kak/`)


3. Put the `kakship.kak` script in your autoload path and add something like this to your kakrc

```
hook global ModuleLoaded kakship .* %{
	kakship-enable
}
```

### With a plugin manager

with [plug.kak](https://github.com/andreyorst/plug.kak)

```
plug "eburghar/kakship" do %{
	cargo install --force --path . --root ~/.local
	[ ! -e $kak_config/starship.toml ] && cp starship.toml $kak_config/
} config %{
	kakship-enable
}
```

## Writing custom segments

To write new segment, you can use the [custom-commands](https://starship.rs/config/#custom-commands) module of starship.
Define a new section with a dot notation, and insert a variable with the same name in the topmost format definition.

In case you just need string substitutions (like custom.kakmode bellow), you can avoid calling a shell to evaluate the
`when` condition by setting the `shell` variable to `['true']` and the `when` variable to `''`. In case no `$output`
variable appears in the format, `command` variable is not used and no shell is called. The segment will be faster to evaluate.

In case you really need to call an external command, you have 2 choices:

1. setup `shell`, `command` and `when` and let starship do the evaluation
2. use `%sh{}` block or other expansion blocks inside the format and let kakoune do the evaluation. Note than only
curly brace is supported as the quoting char.

The difference is that with `%sh{}`, kakoune will rebuild the modeline every second or so when in normal mode. This
lead for example to a custom time segment definition (`custom.kaktime` below ) which will show seconds even if the
editor is idle, contrary to the starship time module which change only during pause.

## Kakoune segments

Here is a some common segments for kakoune. I'll be happy to maintain a catalog if you send me a MR.

```toml
# TODO: use a kakoune option set in appropriate hooks and use %opt{basename} instead of calling a shell
[custom.kakfile]
description = 'The current Kakoune buffername'
format = '[/$output ]($style)[]($style inverted) '
style = 'bold bg:blue fg:black'
command = 'basename $kak_buffile'
when = 'true'
shell = ['sh']
disabled = false
```

```toml
[custom.kaksession]
description = 'The current Kakoune session'
format = '[]($style)[  %val{client}:%val{session} ]($style)[]($style inverted)'
style = 'bg:yellow fg:black'
when = ''
shell = ['true']
disabled = false
```

```toml
[custom.kakcursor]
description = 'The current Kakoune cursor position'
format = '[%val{cursor_line}:%val{cursor_char_column}]($style)'
style = 'fg:white'
when = ''
shell = ['true']
disabled = false
```

```toml
[custom.kakmode]
description = 'The current Kakoune mode'
format = ' {{mode_info}}'
when = ''
shell = ['true']
disabled = false
```

```toml
[custom.kakcontext]
description = 'The current Kakoune context'
format = ' {{context_info}}'
when = ''
shell = ['true']
disabled = false
```

```toml
[custom.kakfiletype]
description = 'The current buffer filetype'
format = '\[%opt{filetype}\] '
when = ''
shell = ['true']
disabled = false
```

```toml
[custom.kakposition]
description = 'Relative position of the cursor inside the buffer'
format = '[  $output]($style)'
style = 'bright-white'
command = 'echo -n $(($kak_cursor_line * 100 / $kak_buf_line_count))%'
when = '[ -n "$kak_cursor_line" ]'
shell = ['sh']
disabled = false
```

```toml
[custom.kaktime]
description = "Alternate time segment using kakoune evaluation"
format = "[]($style)[  %sh{date +%T} ]($style)"
style = "fg:black bg:bright-green"
when = ''
shell = ['true']
disabled = false
```

## Tips

To check if your modeline is not overloaded.

```sh
kak_config="~/.config/kak" kakship timings
```

To check the settings with all modules default values

```sh
kak_config="~/.config/kak" kakship print-config
```

To debug the prompt as set under kakoune

```sh
kak_config="~/.config/kak" kakship prompt
```
