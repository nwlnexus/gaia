create table nodes (
    id text primary key,
    serial text not null,
    account_id text not null references accounts(id),
    location_id text references locations(id),
    created datetime not null default current_timestamp
);
create unique index nodes_serial_idx on nodes (serial);