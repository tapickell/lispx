package main

import (
	"fmt"

	parsec "github.com/prataprc/goparsec"
)

var input = `(+ (* 2 (/ (- 40 20) 5)) 4)`

func main() {
	var array parsec.Parser
	ast := parsec.NewAST("pnote", 100)

	numb := parsec.And(
		func(ns []parsec.ParsecNode) parsec.ParsecNode {
			t := ns[0].(*parsec.Terminal)
			//t.Value = `"` + t.Value + `"`
			return t
		},
		parsec.Token(`\d+`, "NUMBER"),
	)
	operator := parsec.And(
		func(ns []parsec.ParsecNode) parsec.ParsecNode {
			t := ns[0].(*parsec.Terminal)
			//t.Value = `"` + t.Value + `"`
			return t
		},
		parsec.Token(`[+,\-,*,/]`, "OPERATOR"),
	)
	openp := parsec.Atom(`(`, "OPENP")
	closep := parsec.Atom(`)`, "CLOSEP")
	spc := ast.Maybe("space", nil, parsec.Atom(` `, "SPACE"))

	number := ast.OrdChoice("number", nil, numb, &array)
	itemsep := ast.And("itemsep", nil, number, spc)
	items := ast.Kleene("items", nil, itemsep, nil)
	array = ast.And("array", nil, openp, operator, items, closep)

	s := parsec.NewScanner([]byte(input))
	node, _ := ast.Parsewith(array, s)
	fmt.Println(node.GetValue())
	ast.Prettyprint()
}
