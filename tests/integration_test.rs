extern crate e2e_matrix;

#[test]
fn megolm_session_creation() {
    let room_id = String::from("!#super_secret_room:matrix.org");
    let sender_key = String::from("OXIP!=S_ENDER_KEY");
    let device_id = String::from("PLAYROOM");
    let mut megol_session = e2e_matrix::crypto::megolm_sha2::MegolmSession::new(room_id);
    let message = megol_session.create_message(sender_key, device_id, "Hello world");

    println!("{:#?}", message);
}
