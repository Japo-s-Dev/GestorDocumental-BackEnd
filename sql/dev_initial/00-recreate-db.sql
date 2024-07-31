-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'app_user' OR datname = 'gestor_documental';
DROP DATABASE IF EXISTS gestor_documental;
DROP USER IF EXISTS app_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER app_user PASSWORD 'dev_password';
CREATE DATABASE gestor_documental OWNER app_user ENCODING = 'UTF-8';
