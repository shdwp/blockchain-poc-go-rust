package goc_client

import (
	"encoding/json"
	"fmt"
	"gochain_poc/goc_core"
	"io"
	"log"
	"net/http"
)

type GocClient struct {
	chain goc_core.Blockchain
	peers []string
}

func (c *GocClient) Catchup(lastHash string) {
	for i := range c.peers {
		resp, err := http.Get(fmt.Sprintf("http://%s/catchup?lastIdx=%d", c.peers[i], lastHash))
		if err != nil {
			log.Println("request error: ", err)
			continue
		}

		body, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Println("body read error: ", err)
			continue
		}

		var array []goc_core.Block
		if err = json.Unmarshal(body, &array); err != nil {
			log.Println("response json unmarshal: ", err)
			continue
		}

		var dataParsingError error
		for i := range array {
			block := &array[i]
			if data, err := goc_core.ParseData(block.DataTypeId, block.DataJson); err == nil {
				block.Data = data
			} else {
				dataParsingError = err
				break
			}
		}

		if dataParsingError != nil {
			log.Println("data parsing error: ", dataParsingError)
			continue
		}

		for i := range array {
			if err := c.chain.AppendBlock(&array[i]); err != nil {
				log.Println("append block error: ", err)
				break
			}
		}
	}
}

func NewClient(chain goc_core.Blockchain, peers []string) *GocClient {
	client := GocClient{peers: peers, chain: chain}
	return &client
}