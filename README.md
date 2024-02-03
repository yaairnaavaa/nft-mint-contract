CONTRATO INTELIGENTE PARA MINAR NFT’s en NEAR

Versión de node: 12.22.5

Versión de NEAR CLI: 3.4.2

El siguiente contrato inteligente es un ejemplo del minado de NFT´s en NEAR Protocol, siga los siguientes pasos para compilar y desplegar el contrato.

Compilar y desplegar contrato Contrato:

    ./build.sh

Una vez compilado y desplegado el contrato deberá cambiar el CONTRACT y USER_ACCOUNT por las correspondientes.

CONTRACT=dev-1706919298520-70169149370455

USER_ACCOUNT=yairnava.testnet

Inicializar contrato:

    near call $CONTRACT new_default_meta '{"owner_id": "'$CONTRACT'"}' --accountId $CONTRACT

Mostrar metadata del contrato

    near view $CONTRACT nft_metadata

Minar con metadata

    near call $CONTRACT nft_mint '{"receiver_id": "'$USER_ACCOUNT'", "metadata": {"title": "Mi NFT", "description": "Este NFT tiene metadata personalizada", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}}' --accountId $USER_ACCOUNT --amount 0.1 


Minar sin metadata

    near call $CONTRACT mint '{ "receiver_id": "'$USER_ACCOUNT'" }' --accountId $USER_ACCOUNT --deposit 0.01

Consultar número de tokens minados

    near view $CONTRACT nft_total_supply

Consultar NFT por su ID

    near view $CONTRACT nft_token '{"token_id": "0"}'

Consultar NFT's por segmento

    near view $CONTRACT nft_tokens '{"from_index": "0", "limit": 50}'

Consultar NFT's por segmento de un usuario

    near view $CONTRACT nft_tokens_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}' 

Actualizar NFT

    near call $CONTRACT nft_update '{"nft_id": "'2'", "new_metadata": {"title": "Mi NFT Actualizado", "description": "Este NFT tiene metadata personalizada", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}}' --accountId $USER_ACCOUNT
