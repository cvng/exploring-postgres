-- Roles

create role viewer noinherit;
create role bidder in role viewer;
create role seller in role bidder;
create role admin in role seller;
