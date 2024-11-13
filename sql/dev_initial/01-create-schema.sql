DROP SCHEMA IF EXISTS consts;
CREATE SCHEMA IF NOT EXISTS consts;

DROP TABLE IF EXISTS consts.datatype cascade;
CREATE TABLE IF NOT EXISTS
    consts.datatype (
        id BIGSERIAL PRIMARY KEY,
        datatype_name VARCHAR(50) NOT NULL
    );

DROP TABLE IF EXISTS consts.privilege cascade;
CREATE TABLE IF NOT EXISTS
    consts.privilege (
        id BIGSERIAL PRIMARY KEY,
        privilege_name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50)
    );

DROP TABLE IF EXISTS public.structure cascade;
CREATE TABLE IF NOT EXISTS
    public.structure (
        id BIGSERIAL PRIMARY KEY,
        project_name VARCHAR(50) NOT NULL,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now()
    );

DROP TABLE IF EXISTS public.index cascade;
CREATE TABLE IF NOT EXISTS
    public.index (
        id BIGSERIAL PRIMARY KEY,
        project_id BIGINT NOT NULL,
        datatype_id BIGINT NOT NULL,
        required BOOLEAN,
        index_name VARCHAR(50) NOT NULL,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (project_id) REFERENCES structure(id),
        FOREIGN KEY (datatype_id) REFERENCES consts.datatype(id)
    );

DROP TABLE IF EXISTS public.role cascade;
CREATE TABLE IF NOT EXISTS
    public.role (
        id BIGSERIAL PRIMARY KEY,
        role_name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50),
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now()
    );

DROP TABLE IF EXISTS public.assosiated_privilege cascade;
CREATE TABLE IF NOT EXISTS
    public.assosiated_privilege (
        id BIGSERIAL NOT NULL PRIMARY KEY,
        role_name VARCHAR(50) NOT NULL,
        privilege_id BIGINT NOT NULL,
        is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (privilege_id) REFERENCES consts.privilege (id),
        FOREIGN KEY (role_name) REFERENCES role (role_name)
    );

DROP TABLE IF EXISTS public."user" cascade;
CREATE TABLE IF NOT EXISTS
    public.user (
        id BIGSERIAL PRIMARY KEY,
        username VARCHAR(50) UNIQUE NOT NULL,
        email VARCHAR(50) UNIQUE NOT NULL,
        pwd VARCHAR(256),
        pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
        token_salt uuid NOT NULL DEFAULT gen_random_uuid(),
        assigned_role VARCHAR(50),
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (assigned_role) REFERENCES role(role_name)
    );

DROP TABLE IF EXISTS public.archive cascade;
CREATE TABLE IF NOT EXISTS
    public.archive (
        id BIGSERIAL PRIMARY KEY,
        project_id BIGINT,
        owner BIGINT NOT NULL,
        last_edit_user BIGINT,
        tag VARCHAR(50),
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (owner) REFERENCES "user" (id),
        FOREIGN KEY (last_edit_user) REFERENCES "user" (id)
    );

DROP TABLE IF EXISTS public.archive_comment cascade;
CREATE TABLE IF NOT EXISTS
    public.archive_comment (
        id BIGSERIAL PRIMARY KEY,
        archive_id BIGINT NOT NULL,
        text VARCHAR(250) NOT NULL,
        user_id BIGINT NOT NULL,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL default now(),
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (user_id) REFERENCES "user" (id)
    );

DROP TABLE IF EXISTS public.document_comment cascade;
CREATE TABLE IF NOT EXISTS
    public.document_comment (
        id BIGSERIAL PRIMARY KEY,
        document_id BIGINT NOT NULL,
        text VARCHAR(250) NOT NULL,
        user_id BIGINT NOT NULL,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL default now(),
        FOREIGN KEY (document_id) REFERENCES document(id),
        FOREIGN KEY (user_id) REFERENCES "user" (id)
    );

DROP TABLE IF EXISTS public.separator cascade;
CREATE TABLE IF NOT EXISTS
    public.separator (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        parent_id BIGINT,
        archive_id BIGINT NOT NULL,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (parent_id) REFERENCES separator(id)
    );

DROP TABLE IF EXISTS public.value cascade;
CREATE TABLE IF NOT EXISTS
    public.value (
        id BIGSERIAL PRIMARY KEY,
        index_id BIGINT NOT NULL,
        project_id BIGINT NOT NULL,
        archive_id BIGINT NOT NULL,
        last_edit_user BIGINT NOT NULL,
        value VARCHAR(128) NOT NULL,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (index_id) REFERENCES index(id),
        FOREIGN KEY (project_id) REFERENCES structure(id),
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (last_edit_user) REFERENCES "user" (id)
    );

DROP TABLE IF EXISTS public.document cascade;
CREATE TABLE IF NOT EXISTS
    public.document (
        id BIGSERIAL PRIMARY KEY,
        archive_id BIGINT NOT NULL,
        separator_id BIGINT NOT NULL,
        name VARCHAR(256) NOT NULL,
        doc_type VARCHAR(50) NOT NULL,
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        owner BIGINT NOT NULL,
        last_edit_user BIGINT,
        "key" VARCHAR(256) NOT NULL,
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (separator_id) REFERENCES separator(id),
        FOREIGN KEY (owner) REFERENCES "user" (id),
        FOREIGN KEY (last_edit_user) REFERENCES "user" (id)
    );

DROP TABLE IF EXISTS public.structure_privilege cascade;
CREATE TABLE IF NOT EXISTS
    public.structure_privilege (
        project_id BIGINT,
        role_name VARCHAR(50),
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (project_id) REFERENCES structure(id),
        FOREIGN KEY (role_name) REFERENCES role(role_name)
);

DROP TABLE IF EXISTS public.separator_privilege cascade;
CREATE TABLE IF NOT EXISTS
    public.separator_privilege (
        separator_id BIGINT,
        role_name VARCHAR(50),
        is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (separator_id) REFERENCES separator(id),
        FOREIGN KEY (role_name) REFERENCES role(role_name)
);

DROP TABLE IF EXISTS public.event cascade;
CREATE TABLE IF NOT EXISTS public.event (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    action VARCHAR(128) NOT NULL,
    object VARCHAR(128) NOT NULL,
    object_id BIGINT,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT now(),
    old_data JSONB,
    new_data JSONB,
    additional_info JSONB,
    FOREIGN KEY (user_id) REFERENCES public."user"(id)
);

CREATE OR REPLACE FUNCTION log_event()
RETURNS TRIGGER AS $$
DECLARE
    action_text VARCHAR(20);
    user_id BIGINT;
BEGIN
    IF TG_OP = 'INSERT' THEN
        action_text := 'INSERT';
        user_id := NEW.cid;  -- Adjust if needed
        INSERT INTO public.event (user_id, action, object, object_id, timestamp, new_data)
        VALUES (user_id, action_text, TG_TABLE_NAME, NEW.id, now(), to_jsonb(NEW));
        RETURN NEW;

    ELSIF TG_OP = 'UPDATE' THEN
        user_id := NEW.mid;  -- Adjust if needed

        -- Check for soft delete
        IF (NEW.is_deleted = TRUE) AND (OLD.is_deleted = FALSE) THEN
            action_text := 'DELETE';
            INSERT INTO public.event (user_id, action, object, object_id, timestamp, old_data, new_data)
            VALUES (user_id, action_text, TG_TABLE_NAME, NEW.id, now(), to_jsonb(OLD), to_jsonb(NEW));

        -- Check for restore
        ELSIF (NEW.is_deleted = FALSE) AND (OLD.is_deleted = TRUE) THEN
            action_text := 'RESTORE';
            INSERT INTO public.event (user_id, action, object, object_id, timestamp, old_data, new_data)
            VALUES (user_id, action_text, TG_TABLE_NAME, NEW.id, now(), to_jsonb(OLD), to_jsonb(NEW));

        -- Regular update
        ELSE
            action_text := 'UPDATE';
            INSERT INTO public.event (user_id, action, object, object_id, timestamp, old_data, new_data)
            VALUES (user_id, action_text, TG_TABLE_NAME, NEW.id, now(), to_jsonb(OLD), to_jsonb(NEW));
        END IF;

        RETURN NEW;

    -- Optional: Handle physical deletes if they occur
    ELSIF TG_OP = 'DELETE' THEN
        action_text := 'PHYSICAL DELETE';
        user_id := OLD.cid;  -- Adjust if needed
        INSERT INTO public.event (user_id, action, object, object_id, timestamp, old_data)
        VALUES (user_id, action_text, TG_TABLE_NAME, OLD.id, now(), to_jsonb(OLD));
        RETURN OLD;
    END IF;
END;
$$ LANGUAGE plpgsql;


-- Drop existing triggers
DO $$
DECLARE
    tbl RECORD;
    trigger_name TEXT;
BEGIN
    FOR tbl IN SELECT table_name FROM information_schema.tables
                WHERE table_schema = 'public' AND table_type = 'BASE TABLE'
    LOOP
        trigger_name := tbl.table_name || '_audit_trigger';
        EXECUTE format('DROP TRIGGER IF EXISTS %I ON %I;', trigger_name, tbl.table_name);
    END LOOP;
END;
$$;


-- Recreate triggers
DO $$
DECLARE
    tbl RECORD;
    trigger_name TEXT;
BEGIN
    FOR tbl IN SELECT table_name FROM information_schema.tables
                WHERE table_schema = 'public' AND table_type = 'BASE TABLE'
    LOOP
        -- Exclude the 'event' table and any other tables you don't want to audit
        IF tbl.table_name NOT IN ('event') THEN
            trigger_name := tbl.table_name || '_audit_trigger';
            EXECUTE format('
                CREATE TRIGGER %I
                AFTER INSERT OR UPDATE OR DELETE ON %I
                FOR EACH ROW
                EXECUTE FUNCTION log_event();',
                trigger_name, tbl.table_name);
        END IF;
    END LOOP;
END;
$$;

CREATE OR REPLACE FUNCTION associate_role_with_privileges()
RETURNS TRIGGER AS $$
BEGIN
    -- Insert an entry in associated_privilege for each existing privilege only if it does not already exist
    INSERT INTO public.assosiated_privilege (role_name, privilege_id, is_deleted, cid, ctime, mid, mtime)
    SELECT NEW.role_name, p.id, FALSE, NEW.cid, now(), NEW.cid, now()
    FROM consts.privilege p
    WHERE NOT EXISTS (
        SELECT 1 FROM public.assosiated_privilege ap
        WHERE ap.role_name = NEW.role_name AND ap.privilege_id = p.id
    );

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER role_privilege_association
AFTER INSERT ON public.role
FOR EACH ROW
EXECUTE FUNCTION associate_role_with_privileges();

CREATE OR REPLACE FUNCTION associate_privilege_with_roles()
RETURNS TRIGGER AS $$
DECLARE
    creator_id BIGINT := 1;  -- Replace with a default user ID if no `cid` is available
BEGIN
    -- Insert an entry in associated_privilege for each existing role only if it does not already exist
    INSERT INTO public.assosiated_privilege (role_name, privilege_id, is_deleted, cid, ctime, mid, mtime)
    SELECT r.role_name, NEW.id, FALSE, creator_id, now(), creator_id, now()
    FROM public.role r
    WHERE NOT EXISTS (
        SELECT 1 FROM public.assosiated_privilege ap
        WHERE ap.role_name = r.role_name AND ap.privilege_id = NEW.id
    );

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER privilege_role_association
AFTER INSERT ON consts.privilege
FOR EACH ROW
EXECUTE FUNCTION associate_privilege_with_roles();


CREATE OR REPLACE FUNCTION associate_user_with_structures()
RETURNS TRIGGER AS $$
BEGIN
    -- Insert an entry in structure_privilege for each existing structure when a new user is created,
    -- only if the association does not already exist
    INSERT INTO public.structure_privilege (project_id, user_id, is_enabled, is_deleted, cid, ctime, mid, mtime)
    SELECT s.id, NEW.id, TRUE, FALSE, NEW.id, now(), NEW.id, now()
    FROM public.structure s
    WHERE NOT EXISTS (
        SELECT 1 FROM public.structure_privilege sp
        WHERE sp.project_id = s.id AND sp.user_id = NEW.id
    );

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER user_structure_association
AFTER INSERT ON public."user"
FOR EACH ROW
EXECUTE FUNCTION associate_user_with_structures();

CREATE OR REPLACE FUNCTION associate_structure_with_users()
RETURNS TRIGGER AS $$
BEGIN
    -- Insert an entry in structure_privilege for each existing user when a new structure is created,
    -- only if the association does not already exist
    INSERT INTO public.structure_privilege (project_id, user_id, is_enabled, is_deleted,  cid, ctime, mid, mtime)
    SELECT NEW.id, u.id, TRUE, FALSE, NEW.cid, now(), NEW.cid, now()
    FROM public."user" u
    WHERE NOT EXISTS (
        SELECT 1 FROM public.structure_privilege sp
        WHERE sp.project_id = NEW.id AND sp.user_id = u.id
    );

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER structure_user_association
AFTER INSERT ON public.structure
FOR EACH ROW
EXECUTE FUNCTION associate_structure_with_users();

CREATE OR REPLACE FUNCTION enforce_admin_privilege()
RETURNS TRIGGER AS $$
BEGIN
    -- Force `is_deleted` to TRUE if `role_name` is 'ADMIN'
    IF NEW.role_name = 'ADMIN' THEN
        NEW.is_deleted := TRUE;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION enforce_admin_structure_privilege()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if the user being inserted/updated has the "ADMIN" role
    IF (SELECT assigned_role FROM public."user" WHERE id = NEW.user_id) = 'ADMIN' THEN
        NEW.is_enabled := TRUE;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply the trigger before insert or update
CREATE TRIGGER enforce_admin_structure_privilege_trigger
BEFORE INSERT OR UPDATE ON public.structure_privilege
FOR EACH ROW
EXECUTE FUNCTION enforce_admin_structure_privilege();

