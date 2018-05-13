use regex::RegexSet;
use regex::Error;

/// Filters a list of file names
pub struct FileFilter<'a> {
    target_list: &'a [&'a str],
    patterns: RegexSet
}

impl<'a> FileFilter<'a> {

    pub fn new(target_list: &'a [&'a str], patterns: &[&str]) -> Result<FileFilter<'a>, Error> {
        Ok(FileFilter {
            target_list,
            patterns: RegexSet::new(patterns)?
        })
    }

    pub fn apply_patterns(&'a self) -> Vec<& str> {
        self.target_list.iter()
            .cloned()
            .filter(|it| FileFilter::filter_full_match(it, &self.patterns))
            .collect()
    }

    fn filter_full_match(it: & str, regex: &RegexSet) -> bool {
        let r: Vec<_> = regex.matches(it).iter().collect();
        r.len() == regex.len()
    }
}

#[cfg(test)]
mod tests {
    use super::FileFilter;

    #[test]
    fn test_new() {
        let _new_filter = FileFilter::new(&["test"], &["test"]).unwrap();
    }

    #[test]
    fn test_reduce() {
        let target = ["readme.md", "new_script.sql", "old_script.sql", "create_sql.sh"];
        let patterns = ["sql", "script"];
        
        let f = FileFilter::new(&target, &patterns).unwrap();
        let new_list = f.apply_patterns();
        assert_eq!(new_list, vec!["new_script.sql", "old_script.sql"]);
    }
}