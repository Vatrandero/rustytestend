-- NOT FINAL VERSION.
-- UNTIL LATE STAGES OF DEVELOPMENT OR RELEASE - MAY BE OVERWRITTEN.
-- AFTER - CREATE NEW MIGRATION SCRIPTS.
-- IF THIS COMMENT REMOOVED - IT MEANS "DO NOT EDIT ANYMORE"

DROP TABLE public.tests_questions_pool CASCADE;
DROP TABLE public.answers CASCADE;
DROP TABLE public.questions CASCADE;  
DROP TABLE public.tests_open_questions_states CASCADE;
DROP TABLE public.user_sessions CASCADE;
DROP TABLE public.test_asignments CASCADE;
DROP TABLE public.tests_journal CASCADE;
DROP TABLE public.test_session_answered_rels CASCADE;
DROP TABLE public.tests CASCADE;
DROP TABLE public.users CASCADE;
DROP TYPE IF EXISTS user_role;