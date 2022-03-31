package goc_core

import (
	"errors"
	"fmt"
	"gochain_poc/goc_crypto"
	"gochain_poc/goc_util"
	"strings"
)

type Blockchain struct {
	Blocks     []*Block
	Length     uint64
	Difficulty string
}

func MakeBlockchain() Blockchain {
	return Blockchain{make([]*Block, 1), 0, "0000"}
}

func (c *Blockchain) InsertAdminWallet(publicKey, privateKey string) {
	adminWallet := NewWalletData{publicKey}

	signature, _ := goc_crypto.Sign(adminWallet.Serialize(), privateKey)
	block := MineBlock(c, "deadbeef", &adminWallet, signature, "deadbeef")
	c.Blocks[0] = &block
}

func (c *Blockchain) AppendBlock(b *Block) error {
	if !strings.HasPrefix(b.Hash(), c.Difficulty) {
		return errors.New("invalid block - difficulty")
	}

	if len(c.Blocks) > 0 {
		prevBlock := c.Blocks[len(c.Blocks)-1]
		if prevBlock.Hash() != b.PreviousBlockHash {
			return errors.New("invalid block - prev. hash")
		}
	}

	wallet := c.Find("NewWalletData", func(block *Block) bool {
		return block.From == b.From
	})()

	if b.Data.TypeId() == "NewWalletData" {
		if wallet != nil {
			return errors.New("invalid block - duplicate wallet")
		}
	} else {
		signatureValid, _ := goc_crypto.CheckSignature(b.Data.Serialize(), b.Signature, wallet.Data.(*NewWalletData).Pubkey)
		if !signatureValid {
			return errors.New("invalid block - Signature")
		}
	}

	c.Blocks = append(c.Blocks, b)
	c.Length++
	return nil
}

func (c *Blockchain) LastHash() string {
	return c.Blocks[len(c.Blocks)-1].Hash()
}

func (c *Blockchain) Find(typeId string, pred func(*Block) bool) func() *Block {
	i := 0

	return func() *Block {
		for i < len(c.Blocks) {
			block := c.Blocks[i]
			i++
			if block.Data.TypeId() == typeId && pred(block) {
				return block
			}
		}

		return nil
	}
}

func (c *Blockchain) From(hash string) func() (int, *Block) {
	i := goc_util.Max(len(c.Blocks)-1024, 0)

	if hash != "" {
		for i = range c.Blocks {
			if c.Blocks[i].Hash() == hash {
				break
			}
		}
	}

	return func() (int, *Block) {
		for i < len(c.Blocks) {
			block := c.Blocks[i]
			i++
			return i - 1, block
		}

		return len(c.Blocks), nil
	}
}

func (c *Blockchain) Dump() string {
	result := strings.Builder{}

	for i := range c.Blocks {
		block := c.Blocks[i]
		description := block.Data.Serialize()
		descriptionShort := description[:80]
		result.WriteString(fmt.Sprintf("%d %s [%s] %s\n", i, block.Hash(), block.From, descriptionShort))
	}

	return result.String()
}
