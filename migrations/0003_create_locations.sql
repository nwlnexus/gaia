create table locations (
    id text primary key,
    code intger not null,
    name text not null,
    account_id text not null references accounts(id),
    created datetime not null default current_timestamp
);
create unique index locations_code_idx on locations (code);