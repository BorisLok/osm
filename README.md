<h3>Requirement</h3>
* Rust 1.56.0

<h3>Install</h3>
* Install Rust https://www.rust-lang.org/tools/install

<h3>Setting</h3>
* Create .env file under the root of project.

```
# 設定 localhost database.
POSTGRES_DATABASE=
POSTGRES_USER=
POSTGRES_PASSWORD=
POSTGRES_PORT=
POSTGRES_HOST=
# Google API KEY.
API_KEY=
```

* database schema

```
create table city_information
(
    osm_id       integer not null primary key,
    osm_type     varchar(128),
    place_id     integer,
    place_rank   smallint,
    place_type   varchar(128),
    importance   numeric,
    display_name text,
    category     varchar(32),
    country      varchar(128),
    city         varchar(128)
);

create table city_geom
(
    osm_id integer not null primary key references city_information (osm_id),
    geom   geometry(geometry, 4326)
);
```

* Change cities.yaml to config which cities you wanted to crawl.

<h3>Execute</h3>

```
cargo run
```
