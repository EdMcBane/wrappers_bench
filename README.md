create extension wrappers_bench;
create foreign data wrapper bench_fdw handler bench_fdw_handler validator bench_fdw_validator;
create server my_bench_server foreign data wrapper bench_fdw;
create foreign table bench (id bigint, col text) server my_bench_server;

\timing
select * from bench where id < 0;
