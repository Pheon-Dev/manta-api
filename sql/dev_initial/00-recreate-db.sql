-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'manta_app_user' OR datname = 'manta_app_db';
DROP DATABASE IF EXISTS manta_app_db;
DROP USER IF EXISTS manta_app_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER manta_app_user PASSWORD 'dev_only_password';
CREATE DATABASE manta_app_db owner manta_app_user ENCODING = 'UTF-8';
