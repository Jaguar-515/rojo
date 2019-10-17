use std::fs;

use insta::assert_yaml_snapshot;

use crate::serve_util::{run_serve_test, InternAndRedact};

#[test]
fn empty() {
    run_serve_test("empty", |session, mut redactions| {
        let info = session.get_api_rojo().unwrap();
        let root_id = info.root_instance_id;

        assert_yaml_snapshot!("empty_info", redactions.redacted_yaml(info));

        let read_response = session.get_api_read(root_id).unwrap();
        assert_yaml_snapshot!(
            "empty_all",
            read_response.intern_and_redact(&mut redactions, root_id)
        );
    });
}

#[test]
fn scripts() {
    run_serve_test("scripts", |session, mut redactions| {
        let info = session.get_api_rojo().unwrap();
        let root_id = info.root_instance_id;

        assert_yaml_snapshot!("scripts_info", redactions.redacted_yaml(info));

        let read_response = session.get_api_read(root_id).unwrap();
        assert_yaml_snapshot!(
            "scripts_all",
            read_response.intern_and_redact(&mut redactions, root_id)
        );

        fs::write(session.path().join("foo.lua"), "Updated foo!").unwrap();

        let subscribe_response = session.get_api_subscribe(0).unwrap();
        assert_yaml_snapshot!(
            "scripts_subscribe",
            redactions.redacted_yaml(subscribe_response)
        );

        let read_response = session.get_api_read(root_id).unwrap();
        assert_yaml_snapshot!(
            "scripts_all-2",
            read_response.intern_and_redact(&mut redactions, root_id)
        );
    });
}

#[test]
fn just_txt() {
    run_serve_test("just_txt.txt", |session, mut redactions| {
        let info = session.get_api_rojo().unwrap();
        let root_id = info.root_instance_id;

        assert_yaml_snapshot!("just_txt_info", redactions.redacted_yaml(info));

        let read_response = session.get_api_read(root_id).unwrap();
        assert_yaml_snapshot!(
            "just_txt_all",
            read_response.intern_and_redact(&mut redactions, root_id)
        );

        fs::write(session.path(), "Changed content!").unwrap();

        // TODO: Directly served files currently don't trigger changed events!
    });
}