-- Esquema base de app

CREATE TABLE
    datatype (
        datatype_id SERIAL PRIMARY KEY,
        name VARCHAR(50) NOT NULL
    );

CREATE TABLE
    project (
        project_id SERIAL PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        template VARCHAR(50) NOT NULL
    );

CREATE TABLE
    archive (
        archive_id SERIAL PRIMARY KEY,
        creation_date DATE NOT NULL,
        modified_date DATE,
        owner INT NOT NULL,
        last_edit_user INT,
        tag VARCHAR(50)
    );

CREATE TABLE 
    separator (
        separator_id Serial PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        parent_id INT NOT NULL,
        archive_id INT NOT NULL,
        FOREIGN KEY (archive_id) REFERENCES archive(archive_id) ON DELETE CASCADE,
        FOREIGN KEY (parent_id) REFERENCES separator(separator_id) ON DELETE CASCADE
    );

CREATE TABLE
    document (
        document_id SERIAL PRIMARY KEY,
        parent_id INT NOT NULL,
        name VARCHAR(50) NOT NULL,
        doctype VARCHAR(50) NOT NULL,
        creation_date DATE NOT NULL,
        modified_date DATE,
        owner INT NOT NULL,
        last_edit_user INT,
        url VARCHAR(50) NOT NULL,
        FOREIGN KEY (parent_id) REFERENCES separator(separator_id) ON DELETE CASCADE
    );

CREATE TABLE
    index (
        correlative SERIAL NOT NULL,
        project_id INT NOT NULL,
        datatype INT NOT NULL,
        required VARCHAR(50),
        PRIMARY KEY (correlative, project_id),
        FOREIGN KEY (project_id) REFERENCES project(project_id) ON DELETE CASCADE,
        FOREIGN KEY (datatype) REFERENCES datatype(datatype_id) ON DELETE CASCADE
    );

CREATE TABLE
    value (
        correlative INT NOT NULL,
        project_id INT NOT NULL,
        archive_id INT NOT NULL,
        creation_date DATE NOT NULL,
        modified_date DATE,
        last_edit_user INT,
        value VARCHAR(50) NOT NULL,
        FOREIGN KEY (correlative, project_id) REFERENCES index(correlative, project_id) ON DELETE CASCADE,
        FOREIGN KEY (project_id) REFERENCES project(project_id) ON DELETE CASCADE,
        FOREIGN KEY (archive_id) REFERENCES archive(archive_id) ON DELETE CASCADE
    );

CREATE TABLE
    "user" (
        id BIGSERIAL PRIMARY KEY,
        username VARCHAR(50) UNIQUE NOT NULL,
        email VARCHAR(50) UNIQUE NOT NULL,        
        pwd VARCHAR(256),
        pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
        token_salt uuid NOT NULL DEFAULT gen_random_uuid()
    );

CREATE TABLE
    role (
        role_id SERIAL PRIMARY KEY,
        name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50)
    );

CREATE TABLE
    privilege (
        privilege_id SERIAL PRIMARY KEY,
        name VARCHAR(50) UNIQUE NOT NULL,
        description VARCHAR(50)
    );

CREATE TABLE
    assigned_role (
        user_id INT NOT NULL,
        role_id INT NOT NULL,
        PRIMARY KEY (user_id, role_id),
        FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE,
        FOREIGN KEY (role_id) REFERENCES role (role_id) ON DELETE CASCADE
    );

CREATE TABLE
    assosiated_privilege (
        privilege_id INT NOT NULL,
        role_id INT NOT NULL,
        PRIMARY KEY (privilege_id, role_id),
        FOREIGN KEY (privilege_id) REFERENCES privilege (privilege_id) ON DELETE CASCADE,
        FOREIGN KEY (role_id) REFERENCES role (role_id) ON DELETE CASCADE
    );

CREATE TABLE
    log_session (id_log SERIAL PRIMARY KEY);

CREATE TABLE
    log_detail (
        id_log INT NOT NULL,
        user_id INT NOT NULL,
        datetime DATETIME NOT NULL,
        action VARCHAR(50),
        token VARCHAR(50),
        PRIMARY KEY (id_log, user_id),
        FOREIGN KEY (id_log) REFERENCES log_session (id_log) ON DELETE CASCADE,
        FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE
    );
