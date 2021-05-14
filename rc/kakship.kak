declare-option -docstring "modelinefmt backup value." str kakship_modelinefmt_bak %opt{modelinefmt}

define-command -docstring "kakship-enable: require kakship module and enable kakship for all normal buffers." \
kakship-enable %{
	remove-hooks global kakship(-.*)?
	hook -group kakship global WinCreate ^[^*]+$ %{
		require-module kakship
		hook -group kakship window NormalIdle .* starship-modeline
	}
}

provide-module kakship %{

define-command -hidden -docstring "set modeline using kakship" starship-modeline %{
	evaluate-commands %sh{
		# this comment trigger var export kak_session, kak_client, kak_config
		prompt=$(cd $(dirname $kak_buffile) && kakship prompt)
		printf 'set-option window modelinefmt %%{%s}' "${prompt}"
	}
}

define-command -docstring "disable starship modeline" kakship-disable %{
	remove-hooks global kakship(-.*)?
	remove-hooks window kakship(-.*)?
	set-option window modelinefmt %opt{kakship_modelinefmt_bak}
}

}
