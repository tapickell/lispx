package main

import "testing"

func TestEvalParsed(t *testing.T) {
	parsed := parse_form("(+ 40 2)")
	result := eval(parsed)
	if result != "42" {
		t.Errorf("parse |> eval %s, != %d", result, 42)
	}

}
