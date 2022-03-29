package goc_server

import (
	"fmt"
	"gochain_poc/goc_core"
	"log"
	"net/http"
)

func mine(chain goc_core.Blockchain) func(writer http.ResponseWriter, request *http.Request) {
	return func(writer http.ResponseWriter, request *http.Request) {
		from := request.FormValue("from")
		typeId := request.FormValue("typeId")
		jsonString := request.FormValue("data")
		signature := request.FormValue("signature")

		data, err := goc_core.ParseData(typeId, jsonString)
		if err != nil {
			writer.Write([]byte(fmt.Sprint("error parsing data - ", err)))
			return
		}

		block := goc_core.MineBlock(&chain, from, data, signature, chain.LastHash())
		chain.AppendBlock(&block)
	}
}

func catchup(chain goc_core.Blockchain) func(writer http.ResponseWriter, request *http.Request) {
	return func(writer http.ResponseWriter, request *http.Request) {
		lastHash := request.FormValue("lastIdx")

		writer.WriteHeader(200)
		writer.Write([]byte("["))

		iter := chain.From(lastHash)
		for i, block := iter(); block != nil; i, block = iter() {
			writer.Write([]byte(block.Serialize()))

			if i < len(chain.Blocks)-1 {
				writer.Write([]byte(",\n"))
			}
		}

		writer.Write([]byte("]"))
	}
}

func Serve(chain goc_core.Blockchain, bind string) {
	log.Println("Starting server at ", bind)
	http.HandleFunc("/mine", mine(chain))
	http.HandleFunc("/catchup", catchup(chain))
	log.Panic(http.ListenAndServe(bind, nil))
}