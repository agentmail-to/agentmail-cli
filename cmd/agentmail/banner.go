package main

import (
	"fmt"
	"os"
)

const banner = `
   ___                    __  __       _ __
  / _ |___ ____ ___  ____/ /_/  |/  __(_) /
 / __ / _ ` + "`" + `/ -_) _ \/ __/ __/ /|_/ / _ / /
/_/ |_\_, /\__/_//_/\__/\__/_/  /_/\__/_/
     /___/
`

func init() {
	if len(os.Args) == 1 {
		fmt.Print(banner)
	}
}
