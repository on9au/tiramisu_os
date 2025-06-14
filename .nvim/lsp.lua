-- Only override rust-analyzer for this project
return {
	settings = {
		["rust-analyzer"] = {
			cargo = {
				target = "x86_64-none.json", -- for LSP only
			},
		},
	},
}
