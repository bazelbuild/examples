package main

import (
	"fmt"
	"os"

	"example.com/myproject/stage3/lib"
)

func main() {
	who := "world"
	if (len(os.Args) > 1) {
		who = os.Args[1];
	}
	fmt.Println(getGreet(who))
	lib.PrintLocalTime()
}
