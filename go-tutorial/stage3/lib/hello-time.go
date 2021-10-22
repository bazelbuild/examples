package lib

import (
	"fmt"
	"time"
)

func PrintLocalTime() {
	fmt.Println(time.Now().Format(time.ANSIC))
}
