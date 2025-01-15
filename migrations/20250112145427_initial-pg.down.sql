ALTER TABLE IF EXISTS public.sessions
DROP CONSTRAINT IF EXISTS sessions_user_id_fkey;

ALTER TABLE IF EXISTS public.test_asignments
DROP CONSTRAINT IF EXISTS test_asignments_test_id_fkey;

ALTER TABLE IF EXISTS public.test_asignments
DROP CONSTRAINT IF EXISTS test_asignments_user_id_fkey;

ALTER TABLE IF EXISTS public.tests_journal
DROP CONSTRAINT IF EXISTS tests_journal_test_id_fkey;

ALTER TABLE IF EXISTS public.tests_journal
DROP CONSTRAINT IF EXISTS tests_journal_user_id_fkey;
DROP TABLE sessions;
DROP TABLE test_asignments;
DROP TABLE tests_journal;
DROP TABLE tests;
DROP TABLE users;
DROP TYPE IF EXISTS user_role;