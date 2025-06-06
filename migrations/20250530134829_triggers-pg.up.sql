-- NOT FINAL VERSION.
-- UNTIL LATE STAGES OF DEVELOPMENT OR RELEASE - MAY BE OVERWRITTEN.
-- AFTER - CREATE NEW MIGRATION SCRIPTS.
-- IF THIS COMMENT REMOOVED - IT MEANS "DO NOT EDIT ANYMORE"

CREATE OR REPLACE FUNCTiON chk_record_expired_or_locked() 
RETURNS TRIGGER AS $$ 
DECLARE 
now_t TIMESTAMP without time zone;
started_t TIMESTAMP without time zone;
ended_t TIMESTAMP without time zone;
test_dur INTEGER;
max_end_expected_t TIMESTAMP without time zone;
BEGIN
now_t := now();
IF EXISTS (
	SELECT 1 FROM tests_journal
	WHERE id = NEW.journal_record_id AND locked
    ) THEN
        RAISE EXCEPTION 'Locked';
END IF;

SELECT tj.test_user_session_started,
tj.test_user_session_ended, t.duration
	INTO started_t, ended_t, test_dur 
	FROM tests_journal tj
	JOIN test_session_answered_rels rels 
	ON rels.journal_record_id = tj.id
	JOIN tests t ON tj.test_id = t.id
	WHERE tj.id = NEW.journal_record_id;
max_end_expected_t := started_t + (test_dur
* INTERVAL '1 second'  );
IF now_t > max_end_expected_t THEN
RAISE EXCEPTION 'Expired';
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_check_expired_or_blocked
BEFORE INSERT OR UPDATE ON test_session_answered_rels
FOR EACH ROW
EXECUTE FUNCTION chk_record_expired_or_locked();


CREATE OR REPLACE FUNCTION set_test_journal_end_time_on_lock()
RETURNS TRIGGER AS $$ 
DECLARE
now_t TIMESTAMP without time zone;
BEGIN 

now_t := now();

IF EXISTS (
	SELECT 1 FROM tests_journal
	WHERE id = NEW.id AND locked
    ) THEN
	RAISE EXCEPTION 'AlreadyBlocked';
END IF;
NEW.test_user_session_ended := NOW();

END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_set_test_journal_end_time_on_lock
BEFORE UPDATE ON tests_journal 
FOR EACH ROW 
EXECUTE FUNTCION set_test_journal_end_time_on_lock();