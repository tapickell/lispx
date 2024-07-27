package main

import (
	"fmt"
	parsec "github.com/prataprc/goparsec"
)

//var input = `(+ (* 2 (/ (- 40 20) 5)) 4)`

func parse_form(input string) parsec.Queryable {
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
	ast.Prettyprint()
	return node
}

// parsed_node
//array @ 0
//  *OPENP: "("
//  *OPERATOR: "+"
//  items @ 3
//    itemsep @ 3
//      *NUMBER: "40"
//      *missing: ""
//    itemsep @ 6
//      *NUMBER: "2"
//      *missing: ""
//  *CLOSEP: ")"
func eval(parsed_node parsec.Queryable) string {
	// Walk the AST recursively and eval each statement
	fmt.Printf("Children Length %d\n", len(parsed_node.GetChildren()))
//0 &{OPENP ( 0 map[class:[term]]}
//1 &{OPERATOR + 1 map[class:[term]]}
//2 &{items [0x14000122bd0 0x14000122cf0] map[class:[nonterm]]}
//3 &{CLOSEP ) 7 map[class:[term]]}
	for _, child := range parsed_node.GetChildren() {
		eval_child(child)
	}
	return parsed_node.GetValue()
}

func eval_child(child_node parsec.Queryable) {
	fmt.Println("Child Node: ", child_node.GetValue())
}
