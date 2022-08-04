--
-- PostgreSQL database dump
--

-- Dumped from database version 13.5 (Debian 13.5-0+deb11u1)
-- Dumped by pg_dump version 13.5 (Debian 13.5-0+deb11u1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: birds; Type: TABLE; Schema: public; Owner: doerig
--

CREATE TABLE public.birds (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    mandants_id uuid NOT NULL,
    name character varying(128) NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.birds OWNER TO doerig;

--
-- Name: breeds; Type: TABLE; Schema: public; Owner: doerig
--

CREATE TABLE public.breeds (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    nestboxes_id uuid NOT NULL,
    users_id uuid NOT NULL,
    birds_id uuid NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.breeds OWNER TO doerig;

--
-- Name: mandants; Type: TABLE; Schema: public; Owner: doerig
--

CREATE TABLE public.mandants (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    association_name character varying(256) NOT NULL,
    website character varying(256) NOT NULL,
    email character varying(128) NOT NULL
);


ALTER TABLE public.mandants OWNER TO doerig;

--
-- Name: nestboxes; Type: TABLE; Schema: public; Owner: doerig
--

CREATE TABLE public.nestboxes (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    mandants_id uuid NOT NULL,
    public boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL
);


ALTER TABLE public.nestboxes OWNER TO doerig;

--
-- Name: nestboxes_geolocations; Type: TABLE; Schema: public; Owner: doerig
--

CREATE TABLE public.nestboxes_geolocations (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    nestboxes_id uuid NOT NULL,
    lat double precision NOT NULL,
    lng double precision NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expired_at timestamp without time zone
);


ALTER TABLE public.nestboxes_geolocations OWNER TO doerig;

--
-- Name: sessions; Type: TABLE; Schema: public; Owner: doerig
--

CREATE TABLE public.sessions (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    users_id uuid NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    exprires_at timestamp without time zone NOT NULL
);


ALTER TABLE public.sessions OWNER TO doerig;

--
-- Name: users; Type: TABLE; Schema: public; Owner: doerig
--

CREATE TABLE public.users (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    mandants_id uuid NOT NULL,
    locked boolean DEFAULT false NOT NULL,
    username character varying(64) NOT NULL,
    lastname character varying(256) NOT NULL,
    email character varying(128) NOT NULL,
    password_hash character(64) NOT NULL,
    firstname character varying(256)
);


ALTER TABLE public.users OWNER TO doerig;

--
-- Name: birds birds_pkey; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.birds
    ADD CONSTRAINT birds_pkey PRIMARY KEY (id);


--
-- Name: breeds breeds_pkey; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.breeds
    ADD CONSTRAINT breeds_pkey PRIMARY KEY (id);


--
-- Name: mandants mandants_email_key; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.mandants
    ADD CONSTRAINT mandants_email_key UNIQUE (email);


--
-- Name: mandants mandants_pkey; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.mandants
    ADD CONSTRAINT mandants_pkey PRIMARY KEY (id);


--
-- Name: nestboxes_geolocations nestboxes_geolocations_pkey; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.nestboxes_geolocations
    ADD CONSTRAINT nestboxes_geolocations_pkey PRIMARY KEY (id);


--
-- Name: nestboxes nestboxes_pkey; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.nestboxes
    ADD CONSTRAINT nestboxes_pkey PRIMARY KEY (id);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: breeds fk_birds_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.breeds
    ADD CONSTRAINT fk_birds_id FOREIGN KEY (birds_id) REFERENCES public.birds(id);


--
-- Name: users fk_mandants_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT fk_mandants_id FOREIGN KEY (mandants_id) REFERENCES public.mandants(id);


--
-- Name: nestboxes fk_mandants_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.nestboxes
    ADD CONSTRAINT fk_mandants_id FOREIGN KEY (mandants_id) REFERENCES public.mandants(id);


--
-- Name: birds fk_mandants_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.birds
    ADD CONSTRAINT fk_mandants_id FOREIGN KEY (mandants_id) REFERENCES public.mandants(id);


--
-- Name: nestboxes_geolocations fk_nestboxes_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.nestboxes_geolocations
    ADD CONSTRAINT fk_nestboxes_id FOREIGN KEY (nestboxes_id) REFERENCES public.nestboxes(id);


--
-- Name: breeds fk_nestboxes_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.breeds
    ADD CONSTRAINT fk_nestboxes_id FOREIGN KEY (nestboxes_id) REFERENCES public.nestboxes(id);


--
-- Name: sessions fk_users_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT fk_users_id FOREIGN KEY (users_id) REFERENCES public.users(id);


--
-- Name: breeds fk_users_id; Type: FK CONSTRAINT; Schema: public; Owner: doerig
--

ALTER TABLE ONLY public.breeds
    ADD CONSTRAINT fk_users_id FOREIGN KEY (users_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--

