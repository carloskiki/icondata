use std::path::PathBuf;
use anyhow::Result;

use tracing::trace;

use crate::fs::readme_md::Readme;



#[derive(Debug)]
pub(crate) struct BaseRepo {
    readme_md: Readme<BaseRepo>,
}

impl BaseRepo {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            readme_md: Readme {
                path: path.join("README.md"),
                _phantom: std::marker::PhantomData,
            },
        }
    }

    pub(crate) async fn generate(&self) -> Result<()> {
        trace!("Writing the base repo's readme.");
        self.readme_md.reset().await?;
        self.readme_md.write_readme().await
    }
}
