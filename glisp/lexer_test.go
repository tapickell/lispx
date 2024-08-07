package main

import (
	"testing"

	require "github.com/alecthomas/assert/v2"
	"github.com/alecthomas/repr"
)

func TestEvalParsed(t *testing.t) {
	expr, err := parser.ParseString("", `1 + 2 / 3 * (1 + 2)`)
	repr.Println(expr)
	require.NoError(t, err)
}
