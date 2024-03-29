use lorecore::sql::{
    lore_database::LoreDatabase,
    relationships::EntityRelationship,
    search_params::{RelationshipSearchParams, SqlSearchText},
};
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[test]
fn write_single_relationship() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();
    let rel = EntityRelationship {
        parent: "testparent".to_string(),
        child: "testchild".to_string(),
        role: Some("testrole".to_string()),
    };
    db.write_relationships(vec![rel.clone()]).unwrap();
    let rel_out = db
        .read_relationships(RelationshipSearchParams::empty())
        .unwrap();
    assert!(rel_out.len() == 1);
    assert!(rel == rel_out[0]);
    temp_path.close().unwrap();
}

#[test]
fn write_many_relationships() {
    let (temp_path, db, rels) = create_example();

    let rels_out = db
        .read_relationships(RelationshipSearchParams::empty())
        .unwrap();
    assert!(rels.len() == rels_out.len());
    for rel in rels.iter() {
        assert!(rels_out.contains(rel));
    }
    temp_path.close().unwrap();
}

fn create_example() -> (tempfile::TempPath, LoreDatabase, Vec<EntityRelationship>) {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let parents = vec!["testparent1".to_string(), "testparent2".to_string()];
    let children = vec!["testchild1".to_string(), "testchild2".to_string()];
    let roles = vec![Some("testrole".to_string()), None];
    let mut rels: Vec<EntityRelationship> = Vec::new();
    for parent in parents.iter() {
        for child in children.iter() {
            for role in roles.iter() {
                rels.push(EntityRelationship {
                    parent: parent.clone(),
                    child: child.clone(),
                    role: role.clone(),
                });
            }
        }
    }
    rels.sort();

    db.write_relationships(rels.clone()).unwrap();
    (temp_path, db, rels)
}

#[test]
fn writing_several_roles_to_same_relationship() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    let path_in: PathBuf = temp_path.as_os_str().into();
    let db = LoreDatabase::open(path_in.clone()).unwrap();

    let parent = "testparent".to_string();
    let child = "testchild".to_string();
    let roles = vec!["testrole1".to_string(), "testrole2".to_string()];
    let mut rels: Vec<EntityRelationship> = Vec::new();
    for role in roles.iter() {
        rels.push(EntityRelationship {
            parent: parent.clone(),
            child: child.clone(),
            role: Some(role.clone()),
        });
    }

    db.write_relationships(rels.clone()).unwrap();

    let rels_out = db
        .read_relationships(RelationshipSearchParams::empty())
        .unwrap();
    assert!(rels.len() == rels_out.len());
    for rel in rels.iter() {
        assert!(rels_out.contains(rel));
    }
    temp_path.close().unwrap();
}

#[test]
fn get_relationships_without_filter_returns_all() {
    let (temp_path, db, rels) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::empty())
        .unwrap();
    assert!(out == rels);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            None,
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_rent1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.parent == "testparent1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent1")),
            None,
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_testparent1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.parent == "testparent1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("testparent1")),
            None,
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_rent_returns_all() {
    let (temp_path, db, rels) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent")),
            None,
        ))
        .unwrap();
    assert!(out == rels);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_parent_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::exact("fununu")),
            None,
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_parent_filter_rent_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::exact("rent")),
            None,
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_parent_filter_testparent1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.parent == "testparent1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::exact("testparent1")),
            None,
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_child_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_child_filter_ild1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.child == "testchild1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("ild1")),
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_child_filter_testchild1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.child == "testchild1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("testchild1")),
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_child_filter_ild_returns_all() {
    let (temp_path, db, rels) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::partial("ild")),
        ))
        .unwrap();
    assert!(out == rels);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_child_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::exact("fununu")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_child_filter_ild_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::exact("ild")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_exact_child_filter_testchild1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.child == "testchild1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            None,
            Some(SqlSearchText::exact("testchild1")),
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_fununu_and_child_filter_ild_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("fununu")),
            Some(SqlSearchText::partial("ild")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_rent_and_child_filter_fununu_returns_none() {
    let (temp_path, db, _) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent")),
            Some(SqlSearchText::partial("fununu")),
        ))
        .unwrap();
    assert!(out.is_empty());

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_rent1_and_child_filter_ild1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.parent == "testparent1".to_string())
        .filter(|rel| rel.child == "testchild1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent1")),
            Some(SqlSearchText::partial("ild1")),
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_testparent1_and_child_filter_testchild1_returns_some() {
    let (temp_path, db, rels) = create_example();
    let expected = rels
        .iter()
        .filter(|rel| rel.parent == "testparent1".to_string())
        .filter(|rel| rel.child == "testchild1".to_string())
        .cloned()
        .collect::<Vec<_>>();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("testparent1")),
            Some(SqlSearchText::partial("testchild1")),
        ))
        .unwrap();
    assert!(out == expected);

    temp_path.close().unwrap();
}

#[test]
fn get_relationships_with_parent_filter_rent_and_child_filter_ild_returns_all() {
    let (temp_path, db, rels) = create_example();

    let out = db
        .read_relationships(RelationshipSearchParams::new(
            Some(SqlSearchText::partial("rent")),
            Some(SqlSearchText::partial("ild")),
        ))
        .unwrap();
    assert!(out == rels);

    temp_path.close().unwrap();
}

#[test]
fn test_write_read_relationships_after_db_deletion() {
    let (temp_path, db, _) = create_example();
    temp_path.close().unwrap();

    let write_result = db.write_relationships(vec![EntityRelationship {
        parent: "testparent".to_string(),
        child: "testchild".to_string(),
        role: None,
    }]);
    assert!(
        write_result.is_err(),
        "Expected an error when writing to a deleted database"
    );

    let read_result = db.read_relationships(RelationshipSearchParams::new(None, None));
    assert!(
        read_result.is_err(),
        "Expected an error when reading from a deleted database"
    );
}
