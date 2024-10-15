package fortune

import (
	"slices"
	"testing"
)

// TestGet checks that Get returns one of the strings from fortunes.
func TestGet(t *testing.T) {
	msg := Get()
	if i := slices.Index(fortunes, msg); i < 0 {
		t.Errorf("Get returned %q, not one the expected messages", msg)
	}
}
