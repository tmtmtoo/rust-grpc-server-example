-- Diff code generated with pgModeler (PostgreSQL Database Modeler)
-- pgModeler version: 0.9.2-beta1
-- Diff date: 2020-04-26 22:54:16
-- Source model: dev
-- Database: dev
-- PostgreSQL version: 11.0

-- [ Diff summary ]
-- Dropped objects: 0
-- Created objects: 1
-- Changed objects: 0
-- Truncated tables: 0

SET check_function_bodies = false;
-- ddl-end --

SET search_path=public,pg_catalog;
-- ddl-end --


-- [ Created objects ] --
-- object: public.greetings | type: TABLE --
-- DROP TABLE IF EXISTS public.greetings CASCADE;
CREATE TABLE public.greetings (
	id uuid NOT NULL,
	name varchar(255) NOT NULL,
	created_at timestamp NOT NULL,
	CONSTRAINT pk_id PRIMARY KEY (id)

);
-- ddl-end --
ALTER TABLE public.greetings OWNER TO dev;
-- ddl-end --

