use rusqlite::Connection;

pub fn setup(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "
        CREATE TABLE article (
            id INTEGER PRIMARY KEY,
            title NVARCHAR(255),
            content NVARCHAR(255),
            path NVARCHAR(255),
            created DATE NOT NULL,
            edited DATE,
            published BOOLEAN NOT NULL,
            tags NVARCHAR(255)
          );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE img_ref (
            id INTEGER PRIMARY KEY,
            article_id INTEGER NOT NULL,
            img_id INTEGER NOT NULL,
            FOREIGN KEY (article_id) REFERENCES article (id),
            FOREIGN KEY (img_id) REFERENCES img (id)
          );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE img (
            id INTEGER PRIMARY KEY,
            img BLOB NOT NULL,
            path NVARCHAR(255),
            tags NVARCHAR(255)
          );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE mini_article (
            id INTEGER PRIMARY KEY,
            title NVARCHAR(255),
            content NVARCHAR(255),
            img_id INTEGER,
            FOREIGN KEY (img_id) REFERENCES img (id)
          );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE img_ref_article (
            img_ref_article_id INTEGER,
            article_id INTEGER,
            PRIMARY KEY (img_ref_article_id, article_id),
            FOREIGN KEY (img_ref_article_id) REFERENCES img_ref (id),
            FOREIGN KEY (article_id) REFERENCES article (id)
          );
        ",
        [],
    )?;

    Ok(())
}

/*

-- Create article table


-- Create img_ref table


-- Create img table


-- Create mini_article table


-- Create img_ref_article table


*/
