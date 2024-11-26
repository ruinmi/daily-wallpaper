-- CreateTable
CREATE TABLE wallpaper_info
(
    id       INTEGER NOT NULL PRIMARY KEY,
    idx       INTEGER NOT NULL,
    name     TEXT NOT NULL,
    page     INTEGER NOT NULL,
    location TEXT NOT NULL,
    img_src  TEXT NOT NULL,
    used     INTEGER DEFAULT 0,
    deleted  INTEGER DEFAULT 0
);