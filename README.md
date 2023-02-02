CONTRATO INTELIGENTE PARA MINAR NFT’s en NEAR

Versión de node: 12.22.5
Versión de NEAR CLI: 3.4.2

El siguiente contrato inteligente es un ejemplo del minado de NFT´s en NEAR Protocol, siga los siguientes pasos para compilar y desplegar el contrato.

Compilar y desplegar contrato Contrato:

    ./build.sh

CONTRACT=dev-1675279884701-60062749811064

USER_ACCOUNT=fakeyairnava.testnet

Inicializar contrato:

    near call $CONTRACT new_default_meta '{"owner_id": "'$CONTRACT'"}' --accountId $CONTRACT

Mostrar metadata del contrato

    near view $CONTRACT nft_metadata

Minar con metadata

    near call $CONTRACT nft_mint '{"receiver_id": "'$USER_ACCOUNT'", "metadata": {"title": "Mi NFT", "description": "Este NFT tiene metadata personalizada", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}}' --accountId $USER_ACCOUNT --amount 0.1 --gas=300000000000000


Minar sin metadata

    near call $CONTRACT mint '{ "receiver_id": "'$USER_ACCOUNT'" }' --accountId $USER_ACCOUNT --deposit 0.01 --gas=300000000000000

Consultar NFT por su ID

    near view $CONTRACT nft_token '{"token_id": "0"}'

Consultar NFT's por segmento

    near view $CONTRACT nft_tokens '{"from_index": "0", "limit": 50}'

Consultar NFT's por segmento de un usuario

    near view $CONTRACT nft_tokens_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}' 