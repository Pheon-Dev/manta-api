---- Base app schema
---- - Timestamps 
----   - cid/ctime for the creator id and time. 
----   - mid/mtime for the last modifier id and time.

-- User
CREATE TABLE "user" (
  id bigint GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  username varchar(128) NOT NULL UNIQUE,
  email varchar(128) NOT NULL UNIQUE,
  username_norm varchar(128) NOT NULL UNIQUE,      -- column trigger generated (see below)

  -- Auth
  password varchar(256),
  password_salt uuid NOT NULL DEFAULT gen_random_uuid(),
  token_salt uuid NOT NULL DEFAULT gen_random_uuid(),

  -- Timestamps
  cid bigint NOT NULL,
  ctime timestamp with time zone NOT NULL,
  mid bigint NOT NULL,
  mtime timestamp with time zone NOT NULL  
);

-- Normalize the user.username to remove all special characters to constrain unicity rule.
CREATE OR REPLACE FUNCTION user_username_norm_tg_fn()
RETURNS TRIGGER AS $$
BEGIN
  -- This is a strickier rule when the app has full username control (.e.g., not accepting email addresses)
  -- NEW.username_norm := LOWER(REGEXP_REPLACE(TRIM(NEW.username), '[^a-zA-Z0-9]', '', 'g'));

  -- Fairly common rule compatible with most email providers.
  NEW.username_norm := LOWER(TRIM(NEW.username));

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER user_username_norm_tg
BEFORE INSERT OR UPDATE OF username
ON "user"
FOR EACH ROW
EXECUTE FUNCTION user_username_norm_tg_fn();


-- payment
CREATE TABLE payment (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  amount varchar(256) NOT NULL,
  sender varchar(256) NOT NULL,
  receiver varchar(256) NOT NULL,
  description varchar(256) NOT NULL,

  -- Timestamps
  cid bigint NOT NULL,
  ctime timestamp with time zone NOT NULL,
  mid bigint NOT NULL,
  mtime timestamp with time zone NOT NULL  
);

-- card
CREATE TABLE card (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  cname varchar(256) NOT NULL,
  cbalance varchar(256) NOT NULL,
  cnumber varchar(256) NOT NULL,
  ctype varchar(256) NOT NULL,
  caccount varchar(256) NOT NULL,
  cvalid varchar(256) NOT NULL,
  cvv varchar(256) NOT NULL,
  cdescription varchar(256) NOT NULL,

  -- Timestamps
  cid bigint NOT NULL,
  ctime timestamp with time zone NOT NULL,
  mid bigint NOT NULL,
  mtime timestamp with time zone NOT NULL  
);

-- contacts
CREATE TABLE contact (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  username varchar(256) NOT NULL,
  ref_id varchar(256) NOT NULL,
  association varchar(256) NOT NULL,
  email varchar(256) NOT NULL,
  name varchar(256) NOT NULL,

  -- Timestamps
  cid bigint NOT NULL,
  ctime timestamp with time zone NOT NULL,
  mid bigint NOT NULL,
  mtime timestamp with time zone NOT NULL  
);

-- account
CREATE TABLE account (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  balance varchar(256) NOT NULL,
  aid varchar(256) NOT NULL,
  cookie varchar(256) NOT NULL,
  email varchar(256) NOT NULL,
  username varchar(256) NOT NULL,

  -- Timestamps
  cid bigint NOT NULL,
  ctime timestamp with time zone NOT NULL,
  mid bigint NOT NULL,
  mtime timestamp with time zone NOT NULL  
);
