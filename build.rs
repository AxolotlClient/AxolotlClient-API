fn main() {
	println!("cargo:rerun-if-changed=migrations");
	println!("cargo:rerun-if-changed=.sqlx-check.db");
}
