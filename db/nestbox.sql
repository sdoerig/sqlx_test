DROP TABLE mandants;
CREATE TABLE IF NOT EXISTS mandants (
    id uuid DEFAULT gen_random_uuid(), 
    association_name varchar(256) NOT NULL,
    website varchar(256) NOT NULL,
    email varchar(128) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

DROP TABLE users;
CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid(),
    mandants_id uuid NOT NULL,
    locked boolean NOT NULL DEFAULT false,
    username VARCHAR(64) NOT NULL,
    lastname VARCHAR(256) NOT NULL,
    email VARCHAR(128) NOT NULL,
    password_hash CHAR(64) NOT NULL,
    salt uuid NOT NULL,
    UNIQUE(email),
    UNIQUE(username),
    PRIMARY KEY(id),
    CONSTRAINT fk_mandants_id
      FOREIGN KEY(mandants_id) 
	  REFERENCES mandants(id)
);

DROP TABLE sessions;
CREATE TABLE IF NOT EXISTS sessions (
    id uuid DEFAULT gen_random_uuid(),
    users_id uuid NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    exprires_at timestamp NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_users_id
      FOREIGN KEY(users_id)
      REFERENCES users(id)
);

DROP TABLE nestboxes;
CREATE TABLE IF NOT EXISTS nestboxes (
    id uuid DEFAULT gen_random_uuid(),
    mandants_id uuid not null,
    public boolean not null DEFAULT true,
    created_at timestamp not null, 
    PRIMARY KEY (id),
    CONSTRAINT fk_mandants_id
      FOREIGN KEY(mandants_id) 
	  REFERENCES mandants(id)
);

DROP TABLE nestboxes_geolocations;
CREATE TABLE IF NOT EXISTS nestboxes_geolocations (
    id uuid DEFAULT gen_random_uuid(),
    nestboxes_id uuid not null,
    lat double precision not null,
    lng double precision not null,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired_at timestamp NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_nestboxes_id
      FOREIGN KEY(nestboxes_id) 
	  REFERENCES nestboxes(id)
);

DROP TABLE birds;
CREATE TABLE IF NOT EXISTS birds (
    id uuid DEFAULT gen_random_uuid(),
    mandants_id uuid not null,
    name VARCHAR(128) not null,
    created_at TIMESTAMP Not null DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    CONSTRAINT fk_mandants_id
      FOREIGN KEY(mandants_id) 
	  REFERENCES mandants(id)
);


DROP TABLE breeds;
CREATE TABLE IF NOT EXISTS breeds (
    id uuid DEFAULT gen_random_uuid(),
    nestboxes_id uuid not null,
    users_id uuid not null,
    birds_id uuid not null,
    created_at TIMESTAMP Not null DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    CONSTRAINT fk_nestboxes_id
      FOREIGN KEY(nestboxes_id) 
	  REFERENCES nestboxes(id),
    CONSTRAINT fk_users_id
      FOREIGN KEY(users_id) 
	  REFERENCES users(id),
    CONSTRAINT fk_birds_id
      FOREIGN KEY(birds_id) 
	  REFERENCES birds(id)
);









