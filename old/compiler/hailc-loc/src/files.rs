use crate::{Idx, Indexable};

/// A file in a file registry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct File<'a>(&'a str, &'a str);

impl<'a> Indexable for File<'a> {
    fn name() -> String {
        "File<'a>".to_string()
    }
}

/// A [`FileRegistry`], which keeps track of files being compiled in a workspace.
#[derive(Clone, Debug, PartialEq)]
pub struct FileRegistry<'a> {
    /// The file names in this [`FileRegistry`].
    private: Vec<File<'a>>,
}

impl<'a> FileRegistry<'a> {
    /// Creates an empty [`FileRegistry`].
    pub fn new() -> Self {
        Self { private: Vec::new() }
    }

    /// Returns a free index for the next registered file.
    pub fn next_idx(&self) -> Idx<File<'a>> {
        Idx::from_usize(self.private.len())
    }

    /// Registers a file in the [`FileRegistry`].
    /// 
    /// Once a file is registered, it cannot be modified.
    pub fn register_file(&mut self, name: &'a str, source: &'a str) -> Idx<File<'a>> {
        let file = File(name, source);
        let idx = self.next_idx();
        self.private.push(file);
        idx
    }

    /// Returns the file path of the provided source file.
    pub fn get_file_path(&self, source: Idx<File<'a>>) -> &'a str {
        self.private[source.as_usize()].0
    }

    /// Returns the raw source of the provided source file.
    pub fn get_file_source(&self, source: Idx<File<'a>>) -> &'a str {
        self.private[source.as_usize()].1
    }
}