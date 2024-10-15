package fortune

import "math/rand"

var fortunes = []string{
	"Your build will complete quickly.",
	"Your dependencies will be free of bugs.",
	"Your tests will pass.",
}

func Get() string {
	return fortunes[rand.Intn(len(fortunes))]
}
