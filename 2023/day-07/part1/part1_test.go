package main

import "testing"

func Test(t *testing.T) {
	assertEq(t, getHandType("88AA7"), twoPair)
	assertEq(t, getHandType("8T7TQ"), onePair)
}

func assertEq[T comparable](t *testing.T, a T, b T) {
	if a != b {
		t.Fatalf("%v != %v\n", a, b)
	}
}
