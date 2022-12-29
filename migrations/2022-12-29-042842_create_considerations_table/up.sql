create type token_type as enum('ERC721', 'ERC20', 'ERC1155');

CREATE TABLE considerations
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id      UUID    NOT NULL  REFERENCES orders (id),
    recipient     VARCHAR    NOT NULL,
    type          token_type NOT NULL,
    /* common for all type*/
    token_address varchar DEFAULT NULL,
    amount        varchar default null,
    end_amount     varchar default null,
    /* in case ERC20 None */
    /* in case ERC721 & ERC1155 */
    identifier    varchar default null
);