-- DEV ONLY force drop db
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = 'gestor_documental' or usename = 'app_user';

DROP DATABASE IF EXISTS gestor_documental;
DROP USER IF EXISTS app_user;

CREATE USER app_user WITH PASSWORD 'dev_password';
CREATE DATABASE gestor_documental owner app_user encoding = 'UTF8';




