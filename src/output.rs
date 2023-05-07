use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::eyre;
use serde::{de::DeserializeOwned, Serialize};

pub struct Output<T> {
    path: PathBuf,
    pub data: RefCell<HashMap<String, T>>,
}

impl<T> Output<T>
where
    T: DeserializeOwned + Serialize,
{
    pub fn new(path: &str, filename: &str) -> Self {
        let mut s = Self {
            path: Path::new(path).join(filename),
            data: RefCell::new(HashMap::new()),
        };

        s.import_json().unwrap();

        s
    }

    fn create_dir_all(&self) -> color_eyre::Result<()> {
        let path_parent = self
            .path
            .parent()
            .ok_or_else(|| eyre!("Should've had a parent folder"))?;

        fs::create_dir_all(path_parent)?;

        Ok(())
    }

    fn import_json(&mut self) -> color_eyre::Result<()> {
        self.create_dir_all()?;

        let string = fs::read_to_string(&self.path).unwrap_or("{}".to_owned());

        self.data = serde_json::from_str(&string)?;

        Ok(())
    }

    pub fn export_json(&self) -> color_eyre::Result<()> {
        self.create_dir_all()?;

        let data = self.data.borrow();
        let data: HashMap<&String, &T> = data.iter().collect();

        // let one_week_ago = (Utc::now() - Duration::days(7)).date_naive();

        // let data: HashMap<&NaiveDate, &T> = data
        //     .iter()
        //     .filter(|(date_string, _)| NaiveDate::try_from(**date_string).into().lt(&&one_week_ago))
        //     .collect();

        fs::write(&self.path, serde_json::to_string_pretty(&data)?)?;

        Ok(())
    }
}
