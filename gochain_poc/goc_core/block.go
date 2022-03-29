package goc_core

import (
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"strings"
)

type Block struct {
	From              string                 `json:"from"`
	Data              interface{ BlockData } `json:"-"`
	DataTypeId        string                 `json:"data_type_id"`
	DataJson          string                 `json:"data_string"`
	Signature         string                 `json:"signature"`
	Nonce             uint64                 `json:"nonce"`
	PreviousBlockHash string                 `json:"previous_block_hash"`
}

func MakeBlock(from string, data BlockData, sign string, nonce uint64, prevHash string) Block {
	return Block{from, data, data.TypeId(), data.Serialize(), sign, nonce, prevHash}
}

func MineBlock(chain *Blockchain, from string, data BlockData, sign string, prevHash string) Block {
	const maxAttempts = 1024

	block := MakeBlock(from, data, sign, 0, prevHash)
	for attempt := 0; attempt < maxAttempts && !strings.HasPrefix(block.Hash(), chain.Difficulty); attempt++ {
		block = MakeBlock(from, data, sign, block.Nonce+1, prevHash)
	}

	return block
}

func (b *Block) Hash() string {
	buf := make([]byte, 0)
	buf = append(buf, b.Data.TypeId()...)
	buf = append(buf, b.Data.Serialize()...)
	buf = append(buf, b.PreviousBlockHash...)
	binary.BigEndian.PutUint64(buf, b.Nonce)

	hasher := sha256.New()
	hasher.Write(buf)

	return hex.EncodeToString(hasher.Sum(nil))
}

func (b *Block) Serialize() string {
	arr, err := json.Marshal(b)
	if err != nil {
		return ""
	}

	return string(arr)
}

func ParseBlock(jsonString string) (Block, error) {
	var block Block
	err := json.Unmarshal([]byte(jsonString), &block)
	return block, err
}