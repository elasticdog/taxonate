package lang

import (
	"encoding/json"
	"tool/cli"
	"tool/exec"
	"tool/file"
)

command: dump: {
	task: write: file.Create & {
		filename: "languages.json"
		contents: json.Marshal(output) + "\n"
	}

	task: prettier: exec.Run & {
		cmd:    "prettier --print-width 40 --parser json"
		stdin:  task.write.contents
		stdout: string
	}

	task: display: cli.Print & {
		text: task.prettier.stdout
	}
}
