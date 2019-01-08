use std::path::PathBuf;

pub fn get_root_dir(paths: &[PathBuf]) -> Option<PathBuf> {
    if paths.len() < 2 {
        return None;
    }

    let mut buf = paths.get(0).unwrap().clone();

    'outer: for path in paths {
        let mut path_ancestors = path.ancestors();
        let mut diff = (buf.components().count() as i32) - (path.components().count() as i32);

        // chop off extra components
        if diff > 0 {
            // chop off extra buffer components
            while diff > 0 {
                buf.pop();
                diff -= 1;
            }
        } else if diff < 0 {
            // chop off extra path components
            while diff < 0 {
                path_ancestors.next();
                diff += 1;
            }
        }

        let buf_copy = buf.clone();
        let mut buf_ancestors = buf_copy.ancestors();

        loop {
            if buf_ancestors.next() == path_ancestors.next() {
                continue 'outer;
            }

            buf.pop();
        }
    }

    Some(buf)
}

#[cfg(test)]
mod test {
    use super::get_root_dir;
    use std::path::PathBuf;

    #[test]
    fn test_no_paths_provided() {
        assert_eq!(get_root_dir(&Vec::new()), None);
    }

    #[test]
    fn test_one_path_provided() {
        assert_eq!(get_root_dir(&vec![PathBuf::from(r"/root")]), None);
    }

    #[test]
    fn test_returns_common_root_same_path_size() {
        let paths = vec![
            PathBuf::from(r"/root/e/file.a"),
            PathBuf::from(r"/root/f/file.b"),
        ];

        assert_eq!(get_root_dir(&paths), Some(PathBuf::from(r"/root")));
    }

    #[test]
    fn test_returns_common_root_first_is_larger() {
        let paths = vec![
            PathBuf::from(r"/root/a/b/file.a"),
            PathBuf::from(r"/root/d/file.b"),
        ];

        assert_eq!(get_root_dir(&paths), Some(PathBuf::from(r"/root")));
    }

    #[test]
    fn test_returns_common_root_second_is_larger() {
        let paths = vec![
            PathBuf::from(r"/root/e/file.a"),
            PathBuf::from(r"/root/c/d/file.b"),
        ];

        assert_eq!(get_root_dir(&paths), Some(PathBuf::from(r"/root")));
    }
}
