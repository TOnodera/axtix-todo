-- Project Name : rust-todo
-- Date/Time    : 2022/11/06 0:40:14
-- Author       : tonod
-- RDBMS Type   : PostgreSQL
-- Application  : A5:SQL Mk-2

-- TODOテーブル
drop table if exists todos cascade;

create table todos (
  id bigserial not null
  , title varchar(255) not null
  , content text
  , done boolean not null
  , created_at time with time zone default CURRENT_TIMESTAMP not null
  , updated_at time with time zone default CURRENT_TIMESTAMP not null
  , constraint todos_PKC primary key (id)
) ;

comment on table todos is 'TODOテーブル';
comment on column todos.id is 'ID';
comment on column todos.title is 'タイトル';
comment on column todos.content is '本文';
comment on column todos.done is '完了';
comment on column todos.created_at is '登録日時';
comment on column todos.updated_at is '更新日時';
