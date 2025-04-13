use uuid::Uuid;

pub fn get_uuid(input: &str) -> String {
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, input.as_bytes());
    uuid.to_string().replace("-", "")
}
