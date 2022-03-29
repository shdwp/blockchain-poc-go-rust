package goc_server

import (
	"fmt"
	"net/http"
)

func write_err(writer http.ResponseWriter, err error) {
	writer.WriteHeader(500)
	writer.Write([]byte(fmt.Sprint("\"", err.Error(), "\"")))
}