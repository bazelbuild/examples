package main

import (
	"fmt"
	"os"
	"time"
)

func getGreet(who string) string {
	return "Hello " + who;
}

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
