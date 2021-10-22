package main

import (
	"fmt"
	"os"
	"time"
)

func printLocalTime() {
	fmt.Println(time.Now().Format(time.ANSIC))
}

func main() {
	who := "world"
	if (len(os.Args) > 1) {
		who = os.Args[1];
	}
	fmt.Println(getGreet(who))
	printLocalTime()
}
