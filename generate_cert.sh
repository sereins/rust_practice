#!/bin/bash
set -e

# 机构组织信息
PARAMS="/C=CN/ST=GD/L=SZ/O=COM/OU=NSP/CN=myCA/emailAddress=email@qq.com"
# 根证书
ROOT_KEY=root-key
ROOT_CERT=root.crt
openssl genrsa -out $ROOT_KEY 2048
openssl req -new -x509 -key $ROOT_KEY -out $ROOT_CERT -subj $PARAMS

# 扩张配置文件
cat <<EOF > extfile.cnf
subjectAltName = @alt_names
[alt_names]
IP.1 = 127.0.0.1
DNS.1 = example.com
EOF

# 服务端证书
SERVER_KEY=server.key
SERVER_CSR=server.csr
SERVER_CERT=server.crt

echo "------------Generate Server Cert-----------------"
openssl genrsa -out $SERVER_KEY 2048
openssl req -new -key $SERVER_KEY -out $SERVER_CSR -subj $PARAMS
openssl x509 -req -in $SERVER_CSR -CA $ROOT_CERT -CAkey $ROOT_KEY -CAcreateserial -out $SERVER_CERT -days 365 -extfile extfile.cnf

# 客服端证书
CLIENT_KEY=client.key
CLIENT_CSR=client.csr
CLIENT_CERT=client.crt

echo "------------Generate Client Cert-----------------"
openssl genrsa -out $CLIENT_KEY 2048
openssl req -new -key $CLIENT_KEY -out $CLIENT_CSR -subj $PARAMS
openssl x509 -req -in $CLIENT_CSR -CA $ROOT_CERT -CAkey $ROOT_KEY -CAcreateserial -out $CLIENT_CERT -days 365 -extfile extfile.cnf

rm root.srl extfile.cnf $SERVER_CSR $CLIENT_CSR $ROOT_KEY