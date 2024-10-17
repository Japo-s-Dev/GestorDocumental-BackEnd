DROP TABLE IF EXISTS public.separator_privilege;
DROP TABLE IF EXISTS public.structure_privilege;
DROP TABLE IF EXISTS public.log_detail;
DROP TABLE IF EXISTS public.log_session;
DROP TABLE IF EXISTS public.document;
DROP TABLE IF EXISTS public.value;
DROP TABLE IF EXISTS public.separator cascade;
DROP TABLE IF EXISTS public.event;
DROP TABLE IF EXISTS public.comment;
DROP TABLE IF EXISTS public.archive cascade;
DROP TABLE IF EXISTS public."user";
DROP TABLE IF EXISTS public.assosiated_privilege;
DROP TABLE IF EXISTS public.role cascade;
DROP TABLE IF EXISTS public.index cascade;
DROP TABLE IF EXISTS public.privilege;
DROP TABLE IF EXISTS public.structure cascade;
DROP TABLE IF EXISTS public.datatype cascade;

CREATE TABLE IF NOT EXISTS
    public.datatype (
        id BIGSERIAL PRIMARY KEY,
        datatype_name VARCHAR(50) NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now()
    );

CREATE TABLE IF NOT EXISTS
    public.structure (
        id BIGSERIAL PRIMARY KEY,
        project_name VARCHAR(50) NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now()
    );

CREATE TABLE IF NOT EXISTS
    public.privilege (
        id BIGSERIAL PRIMARY KEY,
        privilege_name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50),
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now()
    );

CREATE TABLE IF NOT EXISTS
    public.index (
        id BIGSERIAL PRIMARY KEY,
        project_id BIGINT NOT NULL,
        datatype_id BIGINT NOT NULL,
        required BOOLEAN,
        index_name VARCHAR(50) NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (project_id) REFERENCES structure(id),
        FOREIGN KEY (datatype_id) REFERENCES datatype(id)
    );

CREATE TABLE IF NOT EXISTS
    public.role (
        id BIGSERIAL PRIMARY KEY,
        role_name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50),
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now()
    );

CREATE TABLE IF NOT EXISTS
    public.assosiated_privilege (
        id BIGINT NOT NULL PRIMARY KEY,
        role_name VARCHAR(50) NOT NULL,
        privilege_id BIGINT NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (privilege_id) REFERENCES privilege (id),
        FOREIGN KEY (role_name) REFERENCES role (role_name)
    );

CREATE TABLE IF NOT EXISTS
    public.user (
        id BIGSERIAL PRIMARY KEY,
        username VARCHAR(50) UNIQUE NOT NULL,
        email VARCHAR(50) UNIQUE NOT NULL,
        pwd VARCHAR(256),
        pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
        token_salt uuid NOT NULL DEFAULT gen_random_uuid(),
        assigned_role VARCHAR(50),
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (assigned_role) REFERENCES role(role_name)
    );

CREATE TABLE IF NOT EXISTS
    public.archive (
        id BIGSERIAL PRIMARY KEY,
        project_id BIGINT,
        owner BIGINT NOT NULL,
        last_edit_user BIGINT,
        tag VARCHAR(50),
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (owner) REFERENCES "user" (id),
        FOREIGN KEY (last_edit_user) REFERENCES "user" (id)
    );

CREATE TABLE IF NOT EXISTS
    public.archive_comment (
        id BIGSERIAL PRIMARY KEY,
        archive_id BIGINT NOT NULL,
        text VARCHAR(250) NOT NULL,
        user_id BIGINT NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL default now(),
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (user_id) REFERENCES "user" (id)
    );


CREATE TABLE IF NOT EXISTS
    public.document_comment (
        id BIGSERIAL PRIMARY KEY,
        document_id BIGINT NOT NULL,
        text VARCHAR(250) NOT NULL,
        user_id BIGINT NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL default now(),
        FOREIGN KEY (document_id) REFERENCES document(id),
        FOREIGN KEY (user_id) REFERENCES "user" (id)
    );

CREATE TABLE IF NOT EXISTS
    public.separator (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        parent_id BIGINT,
        archive_id BIGINT NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (parent_id) REFERENCES separator(id)
    );

CREATE TABLE IF NOT EXISTS
    public.value (
        id BIGSERIAL PRIMARY KEY,
        index_id BIGINT NOT NULL,
        project_id BIGINT NOT NULL,
        archive_id BIGINT NOT NULL,
        last_edit_user BIGINT NOT NULL,
        value VARCHAR(128) NOT NULL,
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (index_id) REFERENCES index(id),
        FOREIGN KEY (project_id) REFERENCES structure(id),
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (last_edit_user) REFERENCES "user" (id)
    );

CREATE TABLE IF NOT EXISTS
    public.document (
        id BIGSERIAL PRIMARY KEY,
        archive_id BIGINT NOT NULL,
        separator_id BIGINT NOT NULL,
        name VARCHAR(256) NOT NULL,
        doc_type VARCHAR(50) NOT NULL,
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

CREATE TABLE IF NOT EXISTS
    public.log_session (id BIGSERIAL PRIMARY KEY);

CREATE TABLE IF NOT EXISTS
    public.log_detail (
        id_log BIGINT NOT NULL,
        user_id BIGINT NOT NULL,
        action VARCHAR(50),
        token VARCHAR(50),
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        PRIMARY KEY (id_log, user_id),
        FOREIGN KEY (id_log) REFERENCES log_session (id),
        FOREIGN KEY (user_id) REFERENCES "user" (id)
    );

CREATE TABLE IF NOT EXISTS
    public.structure_privilege (
        project_id BIGINT,
        role_name VARCHAR(50),
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (project_id) REFERENCES structure(id),
        FOREIGN KEY (role_name) REFERENCES role(role_name)
);

CREATE TABLE IF NOT EXISTS
    public.separator_privilege (
        separator_id BIGINT,
        role_name VARCHAR(50),
        cid bigint NOT NULL,
        ctime timestamp with time zone NOT NULL default now(),
        mid bigint NOT NULL,
        mtime timestamp with time zone NOT NULL  default now(),
        FOREIGN KEY (separator_id) REFERENCES separator(id),
        FOREIGN KEY (role_name) REFERENCES role(role_name)
);

CREATE TABLE IF NOT EXISTS public.archive_event (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    action VARCHAR(128) NOT NULL,
    object VARCHAR(128) NOT NULL,
    object_id BIGINT NOT NULL,
    archive_id BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT now(),
    FOREIGN KEY (archive_id) REFERENCES public.archive(id),
    FOREIGN KEY (user_id) REFERENCES public."user"(id)
);

CREATE TABLE IF NOT EXISTS public.document_event (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    action VARCHAR(128) NOT NULL,
    object VARCHAR(128) NOT NULL,
    object_id BIGINT NOT NULL,
    document_id BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT now(),
    FOREIGN KEY (document_id) REFERENCES public.document(id),
    FOREIGN KEY (user_id) REFERENCES public."user"(id)
);


CREATE OR REPLACE FUNCTION log_document_event()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (NEW.owner, 'CREATE', 'DOCUMENT', NEW.id, NEW.archive_id, now());
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (NEW.last_edit_user, 'UPDATE', 'DOCUMENT', NEW.id, NEW.archive_id, now());
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (OLD.owner, 'DELETE', 'DOCUMENT', OLD.id, OLD.archive_id, now());
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER document_event_trigger
AFTER INSERT OR UPDATE OR DELETE ON document
FOR EACH ROW
EXECUTE FUNCTION log_document_event();

CREATE OR REPLACE FUNCTION log_archive_event()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (NEW.owner, 'CREATE', 'ARCHIVE', NEW.id, NEW.id, now());
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (NEW.last_edit_user, 'UPDATE', 'ARCHIVE', NEW.id, NEW.id, now());
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (OLD.owner, 'DELETE', 'ARCHIVE', OLD.id, OLD.id, now());
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER archive_event_trigger
AFTER INSERT OR UPDATE OR DELETE ON archive
FOR EACH ROW
EXECUTE FUNCTION log_archive_event();

CREATE OR REPLACE FUNCTION log_separator_event()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (NEW.cid, 'CREATE', 'SEPARATOR', NEW.id, NEW.archive_id, now());
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO event (user_id, action, object, object_id,aur/notion-app-electron archive_id, timestamp)
        VALUES (NEW.mid, 'UPDATE', 'SEPARATOR', NEW.id, NEW.archive_id, now());
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO event (user_id, action, object, object_id, archive_id, timestamp)
        VALUES (OLD.cid, 'DELETE', 'SEPARATOR', OLD.id, OLD.archive_id, now());
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER separator_event_trigger
AFTER INSERT OR UPDATE OR DELETE ON separator
FOR EACH ROW
EXECUTE FUNCTION log_separator_event();



