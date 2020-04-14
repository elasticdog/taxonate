package lang

import (
	"encoding/json"
	"tool/cli"
	"tool/exec"
)

command: dump: {
	task: prettier: exec.Run & {
		cmd:    "prettier --parser json"
		stdin:  json.Marshal(output)
		stdout: string
	}

	task: display: cli.Print & {
		text: task.prettier.stdout
	}
}
