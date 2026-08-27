use std::{ collections::HashMap, fs::OpenOptions, io::Write, path::Path };

use crate::{ utils::{ errors::AtpError }, watchers::ExecutionWindows::All };
use std::borrow::Cow;

pub mod default_watchers;

// A ideia desse enum é decidir quando determinado watcher será executado

#[derive(Clone)]
pub struct WatcherContext {
    pub current: String,
    pub before: String,
    pub after: Option<String>,
    pub instruction: String,
}

impl WatcherContext {
    pub fn new(
        current: String,
        before: String,
        after: Option<String>,
        instruction: String
    ) -> Self {
        WatcherContext {
            current,
            before,
            after,
            instruction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionWindows {
    All = 0,
    // Every(usize),
    // After(usize),
    // Before(usize),
}

pub struct WatcherList {
    result: HashMap<u64, HashMap<String, String>>,
    watchers: HashMap<&'static str, Box<dyn (Fn(WatcherContext) -> String) + 'static>>,
    schedule: HashMap<&'static str, ExecutionWindows>,
    counter: u64,
}

impl WatcherList {
    pub fn reset(&mut self) -> () {
        self.counter = 0;
        self.result = HashMap::new();
    }

    fn add_to_result(
        &mut self,
        counter: u64,
        watcher_name: String,
        return_value: String
    ) -> Result<(), AtpError> {
        self.result.entry(counter).or_insert_with(HashMap::new).insert(watcher_name, return_value);

        Ok(())
    }
    pub fn set_watcher<F>(&mut self, watcher_name: &'static str, watcher: F)
        where F: Fn(WatcherContext) -> String + 'static
    {
        self.watchers.insert(watcher_name, Box::new(watcher));
    }
    pub fn schedule_watcher(
        &mut self,
        watcher_name: &'static str,
        when: ExecutionWindows
    ) -> Result<(), AtpError> {
        if self.watchers.contains_key(watcher_name) {
            self.schedule.insert(watcher_name, when);
            return Ok(());
        } // Erro de placeholder criar tipo novo de erro pra watcher not found depois
        return Err(
            AtpError::new(
                crate::utils::errors::AtpErrorCode::WatcherNotFoundError(
                    Cow::from(format!("Watcher {} not found", watcher_name))
                ),
                Cow::from("WatcherList.schedule_watcher"),
                Cow::from(watcher_name.to_string())
            )
        );
    }

    pub fn run_watchers(&mut self, input: WatcherContext) -> Result<(), AtpError> {
        // iterate over a collected Vec to avoid holding an immutable borrow of
        // self.schedule while mutably borrowing self later when storing results
        for (watcher_name, when) in self.schedule
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>() {
            // Lookup and call the watcher inside a short-lived scope so the
            // immutable borrow of self.watchers ends before we mutably borrow
            // self to store the result.
            match when {
                All => {
                    let return_value = {
                        let watcher_fn = self.watchers.get(watcher_name).ok_or_else(||
                            AtpError::new(
                                crate::utils::errors::AtpErrorCode::IndexOutOfRange(
                                    Cow::from("Watcher Not Found")
                                ),

                                Cow::from(""),
                                Cow::from("")
                            )
                        )?;
                        watcher_fn(input.clone())
                    };

                    self.add_to_result(self.counter, watcher_name.to_string(), return_value)?;
                }

                // Lógica para outros executionWindows Aqui
            }
        }
        self.counter += 1;

        Ok(())
    }

    pub fn to_json(&self, filename: &Path) -> Result<(), AtpError> {
        let json = serde_json
            ::to_string_pretty(&self.result)
            .map_err(|e|
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::SerializationError(
                        Cow::from(e.to_string())
                    ),
                    Cow::from("WatcherList.to_json"),
                    Cow::from(filename.display().to_string())
                )
            )?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)
            .map_err(|_|
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::FileOpeningError(
                        "Failed opening File".into()
                    ),
                    "",
                    format!("{:?}", filename)
                )
            )?;

        file
            .write(json.as_bytes())
            .map_err(|_|
                AtpError::new(
                    crate::utils::errors::AtpErrorCode::FileWritingError(
                        "Failed writing text to atp file".into()
                    ),
                    "",
                    ""
                )
            )?;

        Ok(())
    }
}
