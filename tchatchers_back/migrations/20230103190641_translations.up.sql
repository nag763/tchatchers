CREATE TABLE LANGUAGE (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL UNIQUE,
	short VARCHAR NOT NULL
);

CREATE TABLE LOCALE (
	id SERIAL PRIMARY KEY,
	language_id INTEGER NOT NULL,
	short_name VARCHAR NOT NULL UNIQUE,
    long_name VARCHAR NOT NULL UNIQUE,
    CONSTRAINT fk_language
        FOREIGN KEY(language_id)
        REFERENCES LANGUAGE(id)
        ON DELETE CASCADE
);

CREATE TABLE LABEL (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    default_translation VARCHAR NOT NULL
);

CREATE TABLE TRANSLATION (
    id SERIAL PRIMARY KEY,
    label_id INTEGER NOT NULL,
    locale_id INTEGER NOT NULL,
    translation VARCHAR DEFAULT NULL,
    CONSTRAINT fk_label
        FOREIGN KEY(label_id)
        REFERENCES LABEL(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_locale
        FOREIGN KEY(locale_id)
        REFERENCES LOCALE(id)
        ON DELETE CASCADE
);

INSERT INTO LANGUAGE(id, name, short) VALUES (1, 'English', 'en'), (2, 'Français', 'fr');
INSERT INTO LOCALE(language_id, short_name, long_name) VALUES (1, 'en_UK', 'English (UK)'), (2, 'fr_FR', 'Français (France)'), (1, 'en_US', 'English (US)');

ALTER TABLE CHATTER
ADD COLUMN locale_id INTEGER NOT NULL DEFAULT 1,
ADD CONSTRAINT fk_locale FOREIGN KEY(locale_id) REFERENCES LOCALE(id) ON DELETE CASCADE;