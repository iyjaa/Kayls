//! Filesystem helpers: atomic write with fsync + rename.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Simpan `content` ke `path` secara atomik.
pub fn save_atomic(path: &Path, content: &str) -> io::Result<()> {
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path has no parent: {}", path.display()),
        )
    })?;

    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path has no file name: {}", path.display()),
        )
    })?;

    let mut temp_name = std::ffi::OsString::from(".");
    temp_name.push(file_name);
    temp_name.push(format!(".kayls.tmp.{}", std::process::id()));
    let temp_path = parent.join(temp_name);

    let write_result = (|| -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        Ok(())
    })();

    if let Err(e) = write_result {
        let _ = fs::remove_file(&temp_path);
        return Err(e);
    }

    if let Err(e) = fs::rename(&temp_path, path) {
        let _ = fs::remove_file(&temp_path);
        return Err(e);
    }

    if let Ok(dir) = File::open(parent) {
        let _ = dir.sync_all();
    }

    Ok(())
}

#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
fn temp_path_for(path: &Path) -> PathBuf {
    let parent = path.parent().expect("parent");
    let file_name = path.file_name().expect("file name");
    let mut name = std::ffi::OsString::from(".");
    name.push(file_name);
    name.push(format!(".kayls.tmp.{}", std::process::id()));
    parent.join(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn unique_dir(tag: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "kayls-fs-util-test-{}-{}-{}",
            tag,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(&p).expect("create temp dir");
        p
    }

    #[test]
    fn save_atomic_writes_content_and_cleans_temp() {
        let dir = unique_dir("basic");
        let path = dir.join("hello.txt");
        save_atomic(&path, "halo dunia\n").expect("save");

        let mut got = String::new();
        File::open(&path)
            .expect("open")
            .read_to_string(&mut got)
            .expect("read");
        assert_eq!(got, "halo dunia\n");

        let temp = temp_path_for(&path);
        assert!(!temp.exists(), "temp file should be cleaned: {:?}", temp);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_atomic_overwrites_existing_and_truncates() {
        let dir = unique_dir("overwrite");
        let path = dir.join("data.txt");
        fs::write(&path, "konten lama yang lebih panjang").expect("seed");

        save_atomic(&path, "baru").expect("save");

        let got = fs::read_to_string(&path).expect("read");
        assert_eq!(got, "baru");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_atomic_creates_missing_file() {
        let dir = unique_dir("create");
        let path = dir.join("nested.txt");
        assert!(!path.exists());
        save_atomic(&path, "x").expect("save");
        assert_eq!(fs::read_to_string(&path).expect("read"), "x");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_atomic_error_on_missing_parent() {
        let path = Path::new("/nonexistent-kayls-dir/xx/file.txt");
        let err = save_atomic(path, "x").unwrap_err();
        assert!(
            matches!(
                err.kind(),
                io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied
            ),
            "unexpected kind: {:?}",
            err.kind()
        );
    }

    #[test]
    fn save_atomic_does_not_corrupt_target_on_temp_failure() {
        // Kontrak atomic: file target TIDAK boleh berubah jika penulisan temp gagal.
        // Cara memicu kegagalan: buat target menjadi direktori (rename ke dir gagal),
        // lalu pastikan direktori itu masih utuh setelah save_atomic error.
        let dir = unique_dir("no-corrupt");
        let target = dir.join("target");
        fs::create_dir(&target).expect("create target dir");

        // Simpan konten penanda di dalam target (sebagai direktori, isi marker).
        let marker = target.join("marker.txt");
        fs::write(&marker, "utuh").expect("marker");

        let err = save_atomic(&target, "konten baru");
        assert!(err.is_err(), "save_atomic ke direktori harus error");

        // Target masih direktori, marker tidak berubah, temp tidak tersisa.
        assert!(target.is_dir(), "target harus tetap direktori");
        assert_eq!(fs::read_to_string(&marker).expect("read marker"), "utuh");
        let temp = temp_path_for(&target);
        assert!(!temp.exists(), "temp harus dibersihkan: {:?}", temp);

        let _ = fs::remove_dir_all(&dir);
    }
}
