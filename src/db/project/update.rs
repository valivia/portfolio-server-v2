use crate::errors::database::DatabaseError;
use crate::models::project::Project;
use crate::models::project::ProjectDocument;
use crate::models::project::ProjectInput;
use crate::models::tag::TagDocument;

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;
use rocket::serde::json::Json;

use super::collect_tags;

/// Returns (`new_project`, `old_project`)
pub async fn update(
    db: &Database,
    oid: ObjectId,
    input: Json<ProjectInput>,
) -> Result<(Project, Project), DatabaseError> {
    let collection = db.collection::<ProjectDocument>("project");

    // Find current version of the project.
    let old: ProjectDocument = collection
        .find_one(doc! {"_id":oid }, None)
        .await
        .map_err(|error| {
            eprintln!("{error}");
            DatabaseError::Database
        })?
        .ok_or(DatabaseError::NotFound)?;

    // Destructure input struct.
    let ProjectInput {
        created_at,
        name,
        description,
        markdown,
        status,
        is_pinned,
        is_project,
        tags,
    } = input.0;

    // Convert vec of objectIds to vec of Tags.
    let tags = match &tags {
        Some(data) => collect_tags(db, data.clone())
            .await
            .map_err(|_| DatabaseError::Mismatch)?,
        None => vec![] as Vec<TagDocument>,
    };

    // Generate DB data struct.
    let doc = ProjectDocument {
        id: old.id,

        created_at: created_at.into(),
        updated_at: bson::DateTime::now(),

        banner_id: old.banner_id,
        status,

        name,
        description,
        markdown,

        is_pinned,
        is_project,

        assets: old.assets.clone(),
        tags,
    };

    let doc = bson::to_bson(&doc).unwrap();

    let query_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    // Update the DB.
    let current = collection
        .find_one_and_update(doc! {"_id": oid}, doc! {"$set": doc}, query_options)
        .await
        .map_err(|error| {
            eprintln!("{error}");
            DatabaseError::Database
        })?
        .ok_or(DatabaseError::NotFound)?;

    Ok((current.into(), old.into()))
}

pub async fn update_banner(
    db: &Database,
    project_id: ObjectId,
    asset_id: ObjectId,
) -> Result<ProjectDocument, DatabaseError> {
    let collection = db.collection::<ProjectDocument>("project");

    let query_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let project = collection
        .find_one_and_update(
            doc! { "assets._id": asset_id, "_id": project_id },
            doc! { "$set": { "banner_id": asset_id }},
            query_options,
        )
        .await
        .map_err(|error| {
            eprintln!("{error}");
            DatabaseError::Database
        })?
        .ok_or(DatabaseError::NotFound)?;

    Ok(project)
}
