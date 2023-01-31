create type TOKEN_TYPE as enum('erc721', 'erc20', 'erc1155');

CREATE TABLE considerations
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id      UUID    NOT NULL  REFERENCES orders (id),
    recipient     VARCHAR    NOT NULL,
    token_type          TOKEN_TYPE NOT NULL,
    /* common for all type*/
    token_address varchar DEFAULT NULL,
    amount        varchar default null,
    end_amount     varchar default null,
    /* in case ERC20 None */
    /* in case ERC721 & ERC1155 */
    identifier    varchar default null
);