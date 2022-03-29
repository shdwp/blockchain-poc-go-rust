package main

import (
	"flag"
	"gochain_poc/goc_client"
	"gochain_poc/goc_core"
	"gochain_poc/goc_crypto"
	"gochain_poc/goc_server"
	"log"
	"strings"
)

func main() {

	bindPtr := flag.String("b", ":8080", "")
	peersPtr := flag.String("peers", "127.0.0.1:8080", "")
	flag.Parse()

	if len(flag.Args()) == 0 {
		flag.Usage()
		return
	}

	switch flag.Args()[0] {
	case "server":
		chain := goc_core.MakeBlockchain()
		chain.InsertAdminWallet(goc_crypto.GenerateKey())
		goc_server.Serve(chain, *bindPtr)

	case "client":
		chain := goc_core.MakeBlockchain()
		client := goc_client.NewClient(chain, strings.Split(*peersPtr, ","))
		client.Catchup("")

	case "new_wallet":
		pubKey, privKey := goc_crypto.GenerateKey()
		log.Println("PUBKEY:")
		log.Println(pubKey)

		log.Println()
		log.Println("PRIVATE KEY:")
		log.Println(privKey)
	}
}