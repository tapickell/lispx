package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Println("LispG Version 0.0.0.0.1")
	fmt.Println("Press Ctrl+c to Exit")
	fmt.Print("lispg> ")

	for scanner.Scan() {
		fmt.Printf("#=> %s\n", scanner.Text())
		fmt.Print("lispg> ")
	}

}
