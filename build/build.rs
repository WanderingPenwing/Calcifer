fn main() {
	// custom parameter to fix clipboard issue between calcifer and external
	println!("cargo:rustc-cfg=web_sys_unstable_apis");
}
