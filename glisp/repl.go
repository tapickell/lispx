package main

import (
	"fmt"
	"strings"

	"github.com/chzyer/readline"
)

func main() {
	fmt.Println("LispG Version 0.0.0.0.1")
	fmt.Println("Interactive LispG - Press Ctrl+c to exit")
	rl, err := readline.New("lispg> ")
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
		fmt.Printf("#=> %s\n", trimmed)
	}

}
