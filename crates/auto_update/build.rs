fn main() {
    for name in ["ZED_NO_AI_RELEASE_VERSION", "ZED_NO_AI_TEAM_ID"] {
        println!("cargo:rerun-if-env-changed={name}");
    }
}
