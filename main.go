package main

import (
	"fmt"
	"net/http"
)

func main() {

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		var header = fmt.Sprint(r.Header)
		println(header)
		w.Write([]byte("ok"))
	})

	http.ListenAndServe(":8000", nil)

}
