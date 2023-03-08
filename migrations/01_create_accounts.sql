create table accounts (
    id text primary key,
    name text not null,
    created datetime not null default current_timestamp
);
create unique index accounts_name_idx on accounts (name);
