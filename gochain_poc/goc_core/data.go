package goc_core

import (
	"encoding/json"
	"errors"
)

type BlockData interface {
	TypeId() string
	Serialize() string
}

func ParseData(typeId, jsonString string) (BlockData, error) {
	switch typeId {
	case "TransactionData":
		var data TransactionData
		err := json.Unmarshal([]byte(jsonString), &data)
		return &data, err

	case "NewWalletData":
		var data NewWalletData
		err := json.Unmarshal([]byte(jsonString), &data)
		return &data, err

	default:
		return nil, errors.New("invalid type")
	}
}

type TransactionData struct {
	To     string `json:"to"`
	ItemId string `json:"item_id"`
	Amount uint64 `json:"amount"`
}

func (t *TransactionData) TypeId() string {
	return "TransactionData"
}

func (t *TransactionData) Serialize() string {
	data, _ := json.Marshal(t)
	return string(data)
}

type NewWalletData struct {
	Pubkey string `json:"pubkey"`
}

func (d *NewWalletData) TypeId() string {
	return "NewWalletData"
}

func (d *NewWalletData) Serialize() string {
	data, _ := json.Marshal(d)
	return string(data)
}