fn main() {
    #[cfg(windows)]
    {
        let res = winres::WindowsResource::new();
        res.compile().unwrap();
    }
}
