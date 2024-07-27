package main

import (
	"fmt"
	"strings"

	"github.com/chzyer/readline"
)

func main() {
	fmt.Println("LispG Version 0.0.0.0.1")
	fmt.Println("Interactive LispG - Press Ctrl+c to exit")
	rl, err := readline.NewEx(&readline.Config{Prompt: "lispg> ", HistoryFile: ".lispg_history", DisableAutoSaveHistory: false})
	if err != nil {
		panic(err)
	}
	defer rl.Close()

	for {
		line, err := rl.Readline()
		if err != nil {
			panic(err)
		}
		trimmed := strings.Trim(line, "\r\n		")
		if trimmed == "(:exit)" {
			break
		}
		if trimmed == "" {
			continue
		}
		parsed_node := parse_form(trimmed)
		result := eval(parsed_node)
		fmt.Printf("#=> %s\n", result)
	}
}
