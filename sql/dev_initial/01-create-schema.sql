-- Esquema base de app
DROP TABLE IF EXISTS public.separator_privilege;
DROP TABLE IF EXISTS public.project_privilege;
DROP TABLE IF EXISTS public.log_detail;
DROP TABLE IF EXISTS public.log_session;
DROP TABLE IF EXISTS public.document;
DROP TABLE IF EXISTS public.value;
DROP TABLE IF EXISTS public.separator;
DROP TABLE IF EXISTS public.archive;
DROP TABLE IF EXISTS public."user";
DROP TABLE IF EXISTS public.assosiated_privilege;
DROP TABLE IF EXISTS public.role;
DROP TABLE IF EXISTS public.index;
DROP TABLE IF EXISTS public.privilege;
DROP TABLE IF EXISTS public.project;
DROP TABLE IF EXISTS public.datatype;

CREATE TABLE IF NOT EXISTS
    public.datatype (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR(50) NOT NULL
    );


CREATE TABLE IF NOT EXISTS
    public.project (
        id BIGSERIAL PRIMARY KEY,
        project_name VARCHAR(50) NOT NULL,
    );

CREATE TABLE IF NOT EXISTS
    public.privilege (
        id BIGSERIAL PRIMARY KEY,
        privilege_name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50)
    );

CREATE TABLE IF NOT EXISTS
    public.index (
        id BIGSERIAL PRIMARY KEY,
        project_id BIGINT NOT NULL,
        datatype BIGINT NOT NULL,
        required VARCHAR(50),
        FOREIGN KEY (project_id) REFERENCES project(id),
        FOREIGN KEY (datatype) REFERENCES datatype(id)
    );


CREATE TABLE IF NOT EXISTS
    public.role (
        id BIGSERIAL PRIMARY KEY
        role_name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50)
    );

CREATE TABLE IF NOT EXISTS
    public.assosiated_privilege (
        privilege_id BIGINT NOT NULL,
        role_name VARCHAR(50) NOT NULL,
        PRIMARY KEY (privilege_id, role_name),
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
        FOREIGN KEY (assigned_role) REFERENCES role(role_name)
    );

CREATE TABLE IF NOT EXISTS
    public.archive (
        id BIGSERIAL PRIMARY KEY,
        creation_date DATE NOT NULL,
        modified_date DATE,
        owner BIGINT NOT NULL,
        last_edit_user BIGINT,
        tag VARCHAR(50),
        FOREIGN KEY (owner) REFERENCES "user" (id),
        FOREIGN KEY (last_edit_user) REFERENCES "user" (id)
    );

CREATE TABLE IF NOT EXISTS
    public.separator (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        parent_id BIGINT NOT NULL,
        archive_id BIGINT NOT NULL,
        FOREIGN KEY (archive_id) REFERENCES archive(id),
        FOREIGN KEY (parent_id) REFERENCES separator(id)
    );


CREATE TABLE IF NOT EXISTS
    public.value (
        id BIGSERIAL PRIMARY KEY,
        index_id BIGINT NOT NULL,
        project_id BIGINT NOT NULL,
        archive_id BIGINT NOT NULL,
        creation_date DATE NOT NULL,
        modified_date DATE,
        last_edit_user BIGINT,
        value VARCHAR(50) NOT NULL,
        FOREIGN KEY (index_id) REFERENCES index(id),
        FOREIGN KEY (project_id) REFERENCES project(id),
        FOREIGN KEY (archive_id) REFERENCES archive(id)
    );


CREATE TABLE IF NOT EXISTS
    public.document (
        id BIGSERIAL PRIMARY KEY,
        parent_id BIGINT NOT NULL,
        name VARCHAR(50) NOT NULL,
        doctype VARCHAR(50) NOT NULL,
        creation_date DATE NOT NULL,
        modified_date DATE,
        owner BIGINT NOT NULL,
        last_edit_user BIGINT,
        url VARCHAR(50) NOT NULL,
        FOREIGN KEY (parent_id) REFERENCES separator(id)
    );

CREATE TABLE IF NOT EXISTS
    public.log_session (id BIGSERIAL PRIMARY KEY);

CREATE TABLE IF NOT EXISTS
    public.log_detail (
        id_log BIGINT NOT NULL,
        user_id BIGINT NOT NULL,
        datetime TIMESTAMP NOT NULL,
        action VARCHAR(50),
        token VARCHAR(50),
        PRIMARY KEY (id_log, user_id),
        FOREIGN KEY (id_log) REFERENCES log_session (id),
        FOREIGN KEY (user_id) REFERENCES "user" (id)
    );

CREATE TABLE IF NOT EXISTS
    public.project_privilege (
        project_id BIGINT,
        role_name VARCHAR(50),
        FOREIGN KEY (project_id) REFERENCES project(id),
        FOREIGN KEY (role_name) REFERENCES role(role_name)
);

CREATE TABLE IF NOT EXISTS
    public.separator_privilege (
        separator_id BIGINT,
        role_name VARCHAR(50),
        FOREIGN KEY (separator_id) REFERENCES separator(id),
        FOREIGN KEY (role_name) REFERENCES role(role_name)
);

