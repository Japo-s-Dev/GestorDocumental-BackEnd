SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'app_user' OR datname = 'gestor_documental';

REASSIGN OWNED BY app_user TO postgres;
DROP OWNED BY app_user;

DROP DATABASE IF EXISTS gestor_documental;
DROP USER IF EXISTS app_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER app_user PASSWORD 'dev_password';
CREATE DATABASE gestor_documental owner app_user ENCODING = 'UTF-8';

