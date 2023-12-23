--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

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
-- Name: before_message_delete(); Type: FUNCTION; Schema: public; Owner: chatter
--

CREATE FUNCTION public.before_message_delete() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  -- Update records in "reported" where "message_uuid" matches the UUID of the message being deleted.
  UPDATE report
  SET is_deleted = TRUE
  WHERE message_uuid = OLD.uuid; -- OLD.uuid represents the UUID of the record being deleted in "message".
  RETURN OLD;
END;
$$;


ALTER FUNCTION public.before_message_delete() OWNER TO chatter;

--
-- Name: chatters_delete_trigger_function(); Type: FUNCTION; Schema: public; Owner: chatter
--

CREATE FUNCTION public.chatters_delete_trigger_function() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  -- Insert the deleted record into the DELETED_RECORD table
  INSERT INTO DELETED_RECORD (ORIGIN, RECORD_ID, RECORD_LOGIN, RECORD_CREATED_AT)
  VALUES ('CHATTER', OLD.id, OLD.login, OLD.created_at);

  RETURN OLD;
END;
$$;


ALTER FUNCTION public.chatters_delete_trigger_function() OWNER TO chatter;

--
-- Name: messages_delete_trigger_function(); Type: FUNCTION; Schema: public; Owner: chatter
--

CREATE FUNCTION public.messages_delete_trigger_function() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  -- Insert the deleted record into the DELETED_RECORD table
  INSERT INTO DELETED_RECORD (ORIGIN, RECORD_UUID, RECORD_ROOM, RECORD_CREATED_AT)
  VALUES ('MESSAGE', OLD.uuid, OLD.ROOM, OLD.timestamp);

  RETURN OLD;
END;
$$;


ALTER FUNCTION public.messages_delete_trigger_function() OWNER TO chatter;

--
-- Name: update_lastupdated_timestamp(); Type: FUNCTION; Schema: public; Owner: chatter
--

CREATE FUNCTION public.update_lastupdated_timestamp() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  NEW.last_update = current_timestamp;
  RETURN NEW;
END;
$$;


ALTER FUNCTION public.update_lastupdated_timestamp() OWNER TO chatter;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO chatter;

--
-- Name: chatter; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.chatter (
    id integer NOT NULL,
    login character varying NOT NULL,
    password character varying NOT NULL,
    is_authorized boolean DEFAULT true NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    name character varying NOT NULL,
    pfp character varying,
    locale_id integer DEFAULT 1 NOT NULL,
    profile_id integer DEFAULT 1 NOT NULL,
    last_update timestamp with time zone DEFAULT now(),
    last_logon timestamp with time zone
);


ALTER TABLE public.chatter OWNER TO chatter;

--
-- Name: chatter_id_seq; Type: SEQUENCE; Schema: public; Owner: chatter
--

CREATE SEQUENCE public.chatter_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.chatter_id_seq OWNER TO chatter;

--
-- Name: chatter_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: chatter
--

ALTER SEQUENCE public.chatter_id_seq OWNED BY public.chatter.id;


--
-- Name: deleted_record; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.deleted_record (
    id integer NOT NULL,
    origin text NOT NULL,
    record_id integer,
    record_login text,
    record_created_at timestamp with time zone,
    record_uuid uuid,
    record_room text,
    deleted_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.deleted_record OWNER TO chatter;

--
-- Name: deleted_record_id_seq; Type: SEQUENCE; Schema: public; Owner: chatter
--

CREATE SEQUENCE public.deleted_record_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.deleted_record_id_seq OWNER TO chatter;

--
-- Name: deleted_record_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: chatter
--

ALTER SEQUENCE public.deleted_record_id_seq OWNED BY public.deleted_record.id;


--
-- Name: label; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.label (
    id integer NOT NULL,
    name character varying NOT NULL,
    default_translation character varying NOT NULL
);


ALTER TABLE public.label OWNER TO chatter;

--
-- Name: label_id_seq; Type: SEQUENCE; Schema: public; Owner: chatter
--

CREATE SEQUENCE public.label_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.label_id_seq OWNER TO chatter;

--
-- Name: label_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: chatter
--

ALTER SEQUENCE public.label_id_seq OWNED BY public.label.id;


--
-- Name: message; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.message (
    uuid uuid NOT NULL,
    content character varying NOT NULL,
    room character varying NOT NULL,
    author integer NOT NULL,
    "timestamp" timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    reception_status integer NOT NULL
);


ALTER TABLE public.message OWNER TO chatter;

--
-- Name: process_kind; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.process_kind (
    id integer NOT NULL,
    name text
);


ALTER TABLE public.process_kind OWNER TO chatter;

--
-- Name: process_kind_id_seq; Type: SEQUENCE; Schema: public; Owner: chatter
--

CREATE SEQUENCE public.process_kind_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.process_kind_id_seq OWNER TO chatter;

--
-- Name: process_kind_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: chatter
--

ALTER SEQUENCE public.process_kind_id_seq OWNED BY public.process_kind.id;


--
-- Name: process_report; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.process_report (
    id integer NOT NULL,
    records_processed integer GENERATED ALWAYS AS ((successfull_records + failed_records)) STORED,
    successfull_records integer NOT NULL,
    failed_records integer NOT NULL,
    passed_at timestamp with time zone DEFAULT now(),
    process_id integer NOT NULL
);


ALTER TABLE public.process_report OWNER TO chatter;

--
-- Name: process_report_id_seq; Type: SEQUENCE; Schema: public; Owner: chatter
--

CREATE SEQUENCE public.process_report_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.process_report_id_seq OWNER TO chatter;

--
-- Name: process_report_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: chatter
--

ALTER SEQUENCE public.process_report_id_seq OWNED BY public.process_report.id;


--
-- Name: profile; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.profile (
    id integer NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.profile OWNER TO chatter;

--
-- Name: profile_id_seq; Type: SEQUENCE; Schema: public; Owner: chatter
--

CREATE SEQUENCE public.profile_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.profile_id_seq OWNER TO chatter;

--
-- Name: profile_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: chatter
--

ALTER SEQUENCE public.profile_id_seq OWNED BY public.profile.id;


--
-- Name: report; Type: TABLE; Schema: public; Owner: chatter
--

CREATE TABLE public.report (
    id integer NOT NULL,
    reporter_id integer NOT NULL,
    reported_id integer,
    message_uuid uuid,
    report_kind_id integer NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    reported_name text,
    reported_pfp text,
    message_content text,
    is_deleted boolean DEFAULT false,
    CONSTRAINT something_reported CHECK (((reported_id IS NOT NULL) OR (message_uuid IS NOT NULL) OR (is_deleted IS TRUE)))
);


ALTER TABLE public.report OWNER TO chatter;

--
-- Name: report_id_seq; Type: SEQUENCE; Schema: public; Owner: chatter
--

CREATE SEQUENCE public.report_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.report_id_seq OWNER TO chatter;

--
-- Name: report_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: chatter
--

ALTER SEQUENCE public.report_id_seq OWNED BY public.report.id;


--
-- Name: chatter id; Type: DEFAULT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.chatter ALTER COLUMN id SET DEFAULT nextval('public.chatter_id_seq'::regclass);


--
-- Name: deleted_record id; Type: DEFAULT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.deleted_record ALTER COLUMN id SET DEFAULT nextval('public.deleted_record_id_seq'::regclass);


--
-- Name: label id; Type: DEFAULT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.label ALTER COLUMN id SET DEFAULT nextval('public.label_id_seq'::regclass);


--
-- Name: process_kind id; Type: DEFAULT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.process_kind ALTER COLUMN id SET DEFAULT nextval('public.process_kind_id_seq'::regclass);


--
-- Name: process_report id; Type: DEFAULT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.process_report ALTER COLUMN id SET DEFAULT nextval('public.process_report_id_seq'::regclass);


--
-- Name: profile id; Type: DEFAULT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.profile ALTER COLUMN id SET DEFAULT nextval('public.profile_id_seq'::regclass);


--
-- Name: report id; Type: DEFAULT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.report ALTER COLUMN id SET DEFAULT nextval('public.report_id_seq'::regclass);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: chatter chatter_login_key; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.chatter
    ADD CONSTRAINT chatter_login_key UNIQUE (login);


--
-- Name: chatter chatter_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.chatter
    ADD CONSTRAINT chatter_pkey PRIMARY KEY (id);


--
-- Name: deleted_record deleted_record_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.deleted_record
    ADD CONSTRAINT deleted_record_pkey PRIMARY KEY (id);


--
-- Name: label label_name_key; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.label
    ADD CONSTRAINT label_name_key UNIQUE (name);


--
-- Name: label label_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.label
    ADD CONSTRAINT label_pkey PRIMARY KEY (id);


--
-- Name: message message_uuid_key; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_uuid_key UNIQUE (uuid);


--
-- Name: process_kind process_kind_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.process_kind
    ADD CONSTRAINT process_kind_pkey PRIMARY KEY (id);


--
-- Name: process_report process_report_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.process_report
    ADD CONSTRAINT process_report_pkey PRIMARY KEY (id);


--
-- Name: profile profile_name_key; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.profile
    ADD CONSTRAINT profile_name_key UNIQUE (name);


--
-- Name: profile profile_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.profile
    ADD CONSTRAINT profile_pkey PRIMARY KEY (id);


--
-- Name: report report_pkey; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.report
    ADD CONSTRAINT report_pkey PRIMARY KEY (id);


--
-- Name: report unicity_reported_message; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.report
    ADD CONSTRAINT unicity_reported_message UNIQUE (reporter_id, message_uuid);


--
-- Name: report unicity_reported_user; Type: CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.report
    ADD CONSTRAINT unicity_reported_user UNIQUE (reporter_id, reported_id);


--
-- Name: chatter chatters_delete_trigger; Type: TRIGGER; Schema: public; Owner: chatter
--

CREATE TRIGGER chatters_delete_trigger AFTER DELETE ON public.chatter FOR EACH ROW EXECUTE FUNCTION public.chatters_delete_trigger_function();


--
-- Name: message messages_delete_trigger; Type: TRIGGER; Schema: public; Owner: chatter
--

CREATE TRIGGER messages_delete_trigger AFTER DELETE ON public.message FOR EACH ROW EXECUTE FUNCTION public.messages_delete_trigger_function();


--
-- Name: message trigger_before_message_delete; Type: TRIGGER; Schema: public; Owner: chatter
--

CREATE TRIGGER trigger_before_message_delete BEFORE DELETE ON public.message FOR EACH ROW EXECUTE FUNCTION public.before_message_delete();


--
-- Name: chatter update_lastupdated_timestamp; Type: TRIGGER; Schema: public; Owner: chatter
--

CREATE TRIGGER update_lastupdated_timestamp BEFORE UPDATE ON public.chatter FOR EACH ROW EXECUTE FUNCTION public.update_lastupdated_timestamp();


--
-- Name: message fk_chatter; Type: FK CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT fk_chatter FOREIGN KEY (author) REFERENCES public.chatter(id) ON DELETE CASCADE;


--
-- Name: report fk_message; Type: FK CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.report
    ADD CONSTRAINT fk_message FOREIGN KEY (message_uuid) REFERENCES public.message(uuid) ON DELETE SET DEFAULT;


--
-- Name: process_report fk_process_id_process_kind; Type: FK CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.process_report
    ADD CONSTRAINT fk_process_id_process_kind FOREIGN KEY (process_id) REFERENCES public.process_kind(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: chatter fk_profile; Type: FK CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.chatter
    ADD CONSTRAINT fk_profile FOREIGN KEY (profile_id) REFERENCES public.profile(id) ON DELETE SET DEFAULT;


--
-- Name: report fk_reported; Type: FK CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.report
    ADD CONSTRAINT fk_reported FOREIGN KEY (reported_id) REFERENCES public.chatter(id) ON DELETE CASCADE;


--
-- Name: report fk_reporter; Type: FK CONSTRAINT; Schema: public; Owner: chatter
--

ALTER TABLE ONLY public.report
    ADD CONSTRAINT fk_reporter FOREIGN KEY (reporter_id) REFERENCES public.chatter(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

