use crate::errors::database::DatabaseError;
use crate::models::project::Project;
use crate::models::project::ProjectDocument;

// use chrono::prelude::*;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use mongodb::Database;

pub async fn find(db: &Database, limit: i64, page: i64) -> Result<Vec<Project>, DatabaseError> {
    let collection = db.collection::<ProjectDocument>("project");

    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = collection.find(None, find_options).await.map_err(|error| {
        eprintln!("{error}");
        DatabaseError::Database
    })?;

    let mut projects: Vec<Project> = vec![];

    while let Some(result) = cursor.try_next().await.map_err(|error| {
        eprintln!("{error}");
        DatabaseError::Database
    })? {
        let project_json = Project::from(result);
        projects.push(project_json);
    }

    Ok(projects)
}

pub async fn find_by_id(db: &Database, oid: ObjectId) -> Result<Project, DatabaseError> {
    let collection = db.collection::<ProjectDocument>("project");

    let project = Project::from(
        collection
            .find_one(doc! {"_id":oid }, None)
            .await
            .map_err(|error| {
                eprintln!("{error}");
                DatabaseError::Database
            })?
            .ok_or(DatabaseError::NotFound)?,
    );

    Ok(project)
}