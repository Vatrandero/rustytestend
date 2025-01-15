CREATE TYPE  user_role AS ENUM
    ('responder', 'test_giver', 'admin');


CREATE TABLE IF NOT EXISTS public.sessions
(
    sid uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    expires_at timestamp without time zone NOT NULL,
    CONSTRAINT sessions_pkey PRIMARY KEY (sid)
);

COMMENT ON COLUMN public.sessions.sid
    IS 'Consider as auth token itself.';

CREATE TABLE IF NOT EXISTS public.test_asignments
(
    test_id integer NOT NULL,
    user_id integer NOT NULL,
    tries integer NOT NULL DEFAULT 0,
    open_from timestamp without time zone NOT NULL DEFAULT now(),
    close_after timestamp without time zone NOT NULL,
    CONSTRAINT one_user_one_test UNIQUE (test_id, user_id)
);

CREATE TABLE IF NOT EXISTS public.tests
(
    id serial NOT NULL,
    tille character varying(128) NOT NULL,
    descripion text,
    duration integer NOT NULL,
    questions json[] NOT NULL,
    question_count integer GENERATED ALWAYS AS (array_length(questions, 1)) STORED,
    pass_score integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    CONSTRAINT tests_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.tests_journal
(
    id serial NOT NULL,
    test_id integer NOT NULL,
    user_id integer NOT NULL,
    locked BOOLEAN NOT NULL DEFAULT FALSE,
    test_session_started timestamp without time zone NOT NULL,
    test_session_ended timestamp without time zone NOT NULL,
    duration_taken_secs interval GENERATED ALWAYS AS ((test_session_ended - test_session_started)) STORED,
    questions_given json[] NOT NULL,
    CONSTRAINT tests_journal_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.users
(
    id serial NOT NULL,
    login character varying(64) COLLATE pg_catalog."default" NOT NULL,
    password_hash character varying(256) COLLATE pg_catalog."default" NOT NULL,
    first_name text COLLATE pg_catalog."default" NOT NULL,
    second_nane text,
    last_name text,
    asigned_role user_role NOT NULL,
    active boolean NOT NULL DEFAULT true,
    asigned_groups character varying(32)[],
    CONSTRAINT users_pkey PRIMARY KEY (id)
);

ALTER TABLE IF  EXISTS public.sessions -- auTH SESSION
    ADD CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF  EXISTS public.test_asignments
    ADD CONSTRAINT test_asignments_test_id_fkey FOREIGN KEY (test_id)
    REFERENCES public.tests (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF  EXISTS public.test_asignments
    ADD CONSTRAINT test_asignments_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.tests_journal
    ADD CONSTRAINT tests_journal_test_id_fkey FOREIGN KEY (test_id)
    REFERENCES public.tests (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;
CREATE INDEX IF NOT EXISTS journal_by_test
    ON public.tests_journal(test_id);


ALTER TABLE IF  EXISTS public.tests_journal
    ADD CONSTRAINT tests_journal_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;