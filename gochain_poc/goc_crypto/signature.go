package goc_crypto

import (
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/pem"
)

func loadPubkey(publicKey string) (*rsa.PublicKey, error) {
	block, _ := pem.Decode([]byte(publicKey))
	return x509.ParsePKCS1PublicKey(block.Bytes)
}

func loadPrivkey(privateKey string) (*rsa.PrivateKey, error) {
	block, _ := pem.Decode([]byte(privateKey))
	return x509.ParsePKCS1PrivateKey(block.Bytes)
}

func GenerateKey() (publicKey string, privateKey string) {
	key, _ := rsa.GenerateKey(rand.Reader, 2048)

	privateKeyData := x509.MarshalPKCS1PrivateKey(key)
	publicKeyData := x509.MarshalPKCS1PublicKey(&key.PublicKey)

	privateKeyBlock := pem.Block{"RSA PRIVATE KEY", nil, privateKeyData}
	publicKeyBlock := pem.Block{"RSA PUBLIC KEY", nil, publicKeyData}

	privateKey = string(pem.EncodeToMemory(&privateKeyBlock))
	publicKey = string(pem.EncodeToMemory(&publicKeyBlock))
	return
}

func CheckSignature(data string, sign string, publicKey string) (bool, error) {
	key, err := loadPubkey(publicKey)

	dataHash := sha256.Sum256([]byte(data))
	signatureData, err := base64.StdEncoding.DecodeString(sign)

	err = rsa.VerifyPKCS1v15(key, crypto.SHA256, dataHash[:], signatureData)
	return err == nil, err
}

func Sign(data string, privateKey string) (string, error) {
	dataHash := sha256.Sum256([]byte(data))
	privKey, err := loadPrivkey(privateKey)
	signData, err := rsa.SignPKCS1v15(rand.Reader, privKey, crypto.SHA256, dataHash[:])

	if err != nil {
		return "", err
	}

	return base64.StdEncoding.EncodeToString(signData), err
}