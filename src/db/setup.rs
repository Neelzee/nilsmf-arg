use anyhow::{Context, Result};
use postgres::Client;

pub async fn setup(conn: &mut Client) -> Result<()> {
    conn.batch_execute(
        "
        CREATE TYPE TAG as ENUM('Project', 'Rust', 'Database', 'Postgresql', 'Rocket');

        CREATE TABLE article (
            iid INT GENERATED ALWAYS AS IDENTITY,
            content TEXT NOT NULL,
            description TEXT NOT NULL,
            PRIMARY KEY (iid)
        );
        
        CREATE TABLE meta_article (
            iid INT GENERATED ALWAYS AS IDENTITY,
            title TEXT NOT NULL,
            created_date VARCHAR(8) NOT NULL,
            created_time VARCHAR(7) NOT NULL,
            last_edit_date VARCHAR(8),
            last_edit_time VARCHAR(7),
            author_id INT,
            content_table_id INT,
            tags TAG[],
            PRIMARY KEY (iid)
            CONSTRAINT fk_author
                FOREIGN KEY(author_id)
                    REFERENCES author(iid)
            CONSTRAINT fk_article
                FOREIGN KEY(content_table_id)
                    REFERENCES article(iid)
        );

        CREATE TABLE author (
            iid INT GENERATED ALWAYS AS IDENTITY,
            uid INT,
            name TEXT,
            email TEXT
            CONSTRAINT fk_user
                FOREIGN KEY(uid)
                    REFERENCES user(iid)
        );

        CREATE TABLE meta_article_edit (
            iid INT GENERATED ALWAYS AS IDENTITY,
            edit_date VARCHAR(8),
            edit_time VARCHAR(7),
            article_id INT,
            changes_id INT,
            CONSTRAINT fk_article
                FOREIGN KEY(article_id)
                    REFERENCES article(iid)
            CONSTRAINT fk_changes
                FOREIGN KEY(changes_id)
                    REFERENCES article_edit(iid)
        );

        CREATE TABLE article_edit (
            iid INT GENERATED ALWAYS AS IDENTITY,
            new_content TEXT,
            add_images INT,
            rm_images INT,
            add_tags TAG[],
            rm_tags TAG[]
            CONSTRAINT fk_add_image_article
                FOREIGN KEY(add_images)
                    REFERENCES image_article(iid)
            CONSTRAINT fk_rm_image_article
                FOREIGN KEY(rm_images)
                    REFERENCES image_article(iid)
        );

        CREATE TABLE image (
            iid INT GENERATED ALWAYS AS IDENTITY,
            title TEXT NOT NULL,
            alt TEXT NOT NULL,
            src BLOB NOT NULL,
            tag TAG[]
        );

        CREATE TABLE image_article (
            iid INT GENERATED ALWAYS AS IDENTITY,
            image_id INT,
            article_id
            CONSTRAINT fk_image
                FOREIGN KEY(image_id)
                    REFERENCES image(iid)
            CONSTRAINT fk_article
                FOREIGN KEY(article_id)
                    REFERENCES article(iid)
        );
        ",
    )
    .context("Failed setting up db")
}

pub async fn serialize_test(conn: &mut Client) -> Result<()> {
    Ok(())
}
