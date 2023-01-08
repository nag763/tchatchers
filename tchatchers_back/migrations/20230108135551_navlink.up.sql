-- Add up migration script here
CREATE TABLE NAVLINK (
    id SERIAL PRIMARY KEY,
    href VARCHAR NOT NULL UNIQUE,
    label_id INTEGER NOT NULL,
    before INTEGER,
    CONSTRAINT fk_label
        FOREIGN KEY(label_id)
        REFERENCES LABEL(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_sort
        FOREIGN KEY(before)
        REFERENCES NAVLINK(id)
        ON DELETE NO ACTION
);

CREATE TABLE NAVLINK_PROFILE (
    profile_id INTEGER,
    navlink_id INTEGER,
    PRIMARY KEY(profile_id, navlink_id),
    CONSTRAINT fk_profile
        FOREIGN KEY(profile_id)
        REFERENCES PROFILE(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_navlink
        FOREIGN KEY(navlink_id)
        REFERENCES NAVLINK(id)
        ON DELETE CASCADE
);

INSERT INTO LABEL(name, default_translation) 
VALUES 
('logout_menu', 'Log out'), 
('settings_menu', 'Settings') 
ON CONFLICT DO NOTHING; 

INSERT INTO TRANSLATION(label_id, locale_id, translation) 
VALUES
((SELECT id FROM LABEL WHERE name='logout_menu'), 2, 'Se déconnecter'),
((SELECT id FROM LABEL WHERE name='settings_menu'), 2, 'Paramètres')
ON CONFLICT DO NOTHING; 

INSERT INTO NAVLINK(href, label_id) VALUES 
('/logout', (SELECT id FROM LABEL WHERE name='logout_menu'));


INSERT INTO NAVLINK(href, label_id, before) VALUES 
('/settings', (SELECT id FROM LABEL WHERE name='settings_menu'), (SELECT id FROM NAVLINK WHERE href='/logout'));

INSERT INTO NAVLINK_PROFILE VALUES 
(1, (SELECT id FROM NAVLINK WHERE href='/logout')),
(1, (SELECT id FROM NAVLINK WHERE href='/settings')),
(2, (SELECT id FROM NAVLINK WHERE href='/logout')),
(2, (SELECT id FROM NAVLINK WHERE href='/settings')),
(3, (SELECT id FROM NAVLINK WHERE href='/logout')),
(3, (SELECT id FROM NAVLINK WHERE href='/settings'));