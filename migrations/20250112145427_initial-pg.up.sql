-- NOT FINAL VERSION.
-- UNTIL LATE STAGES OF DEVELOPMENT OR RELEASE - MAY BE OVERWRITTEN.
-- AFTER - CREATE NEW MIGRATION SCRIPTS.
-- IF THIS COMMENT REMOOVED - IT MEANS "DO NOT EDIT ANYMORE"


CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE  user_role AS ENUM
    ('admin', 'test_giver', 'solver');

CREATE TABLE IF NOT EXISTS public.answers ( 
    id SERIAL PRIMARY KEY NOT NULL, 
    question_id INTEGER, 
    answer_text TEXT, 
    is_correct BOOLEAN
);

CREATE TABLE IF NOT EXISTS public.tests_open_questions_states 
(
    test_journal_record_id INT NOT NULL,
    test_session_question_id INT NOT NULL,
    -- Need to check?
    is_inserted BOOLEAN NOT NULL,
    -- Checked?
    is_reviewed BOOLEAN NOT NULL DEFAULT FALSE,
    -- Aprooved (e.q count as answered and count as)
    is_aprooved BOOLEAN

);
CREATE TABLE IF NOT EXISTS public.questions ( 
    id SERIAL NOT NULL PRIMARY KEY, 
    question_text TEXT NOT NULL
);
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
    title character varying(128) NOT NULL,
    description text,
    duration integer NOT NULL,
    pass_score smallint NOT NULL,
    created_at timestamp without time zone NOT NULL,
    CONSTRAINT tests_pkey PRIMARY KEY (id)
);

CREATE TABLE tests_questions_pool ( 
    test_id INTEGER NOT NULL, 
    question_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS public.test_session_answered_rels ( 
    journal_record_id INTEGER NOT NULL, 
    question_id INTEGER NOT NULL, 
    selected_answer_id INTEGER, 
    arbitrary_answer TEXT, 
	UNIQUE (journal_record_id, question_id),
    CONSTRAINT chk_selected_xor_arbitrary_answer 
        CHECK (
            (selected_answer_id IS NULL AND arbitrary_answer IS NULL) -- not answered yet.
            OR (arbitrary_answer IS NOT NULL AND selected_answer_id IS NULL )-- arbitrary, e.q open question, text passed.
            OR (selected_answer_id IS NOT NULL AND arbitrary_answer IS NULL)-- selected variant, e.q close question - answer id. 
        )
);

CREATE TABLE IF NOT EXISTS public.tests_journal
(
    id serial NOT NULL,
    test_id integer NOT NULL,
    user_id integer NOT NULL,
    locked BOOLEAN NOT NULL DEFAULT FALSE, 
    open_questions_count INT NOT NULL,
    needs_to_review_open_questions_count INT NOT NULL,
    test_user_session_started timestamp without time zone NOT NULL,
    test_user_session_ended timestamp without time zone NULL, --BULL: May be not yet done.
    duration_taken_secs interval GENERATED ALWAYS AS ((test_user_session_ended - test_user_session_started)) STORED,

    CONSTRAINT tests_journal_pkey PRIMARY KEY (id)
);



CREATE TABLE IF NOT EXISTS public.users
(
    id serial NOT NULL,
    locked BOOLEAN NOT NULL DEFAULT FALSE,
    login cHAR varying(64) NOT NULL,
    password_hash CHAR VARYING(88) NOT NULL, -- sized for SCRYPT N=17,r=8,p=1 string.
    first_name text NOT NULL,
    second_name text,
    last_name text,
    asigned_role user_role NOT NULL,
    active boolean NOT NULL DEFAULT true,
    asigned_groups character varying(32)[],
    CONSTRAINT users_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.user_sessions
(
    sid uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id INTEGER NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    expires_at timestamp without time zone NOT NULL,
    CONSTRAINT user_sessions_pkey PRIMARY KEY (sid)
);

COMMENT ON COLUMN public.user_sessions.sid
    IS 'Consider as auth token itself.';

ALTER TABLE IF EXISTS public.answers 
    ADD CONSTRAINT answers_question_id_fkey FOREIGN KEY (question_id)
    REFERENCES public.questions(id)
    ON UPDATE NO ACTION
    ON DELETE CASCADE;

ALTER TABLE public.tests_open_questions_states   
    ADD CONSTRAINT tests_open_questions_states_tests_journal_fkey 
        FOREIGN KEY (test_journal_record_id, test_session_question_id)
        REFERENCES public.test_session_answered_rels
		(journal_record_id, question_id ) 
        ;

ALTER TABLE IF EXISTS public.test_session_answered_rels 
    ADD CONSTRAINT test_session_answered_rels_tests_journal_fkey FOREIGN KEY (journal_record_id) 
        REFERENCES public.tests_journal(id),
    ADD CONSTRAINT test_session_answered_rels_question_id_fkey FOREIGN KEY (question_id)
        REFERENCES public.questions(id),
    ADD CONSTRAINT test_session_answered_rels_selected_answer_id_fkey FOREIGN KEY (selected_answer_id)
        REFERENCES public.answers(id);


ALTER TABLE IF EXISTS public.user_sessions -- auTH user_session
    ADD CONSTRAINT user_sessions_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.tests_questions_pool
    ADD CONSTRAINT test_questions_pool_test_fkey FOREIGN KEY (test_id)
    REFERENCES public.tests(id)
    ON UPDATE NO ACTION,
    ADD CONSTRAINT test_question_pool_question_fkey FOREIGN KEY (question_id)
    REFERENCES questions(id)
    ON UPDATE NO ACTION;

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