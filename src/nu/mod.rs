use futures::executor::block_on;
use nu_errors::ShellError;
use nu_plugin::Plugin;
use nu_protocol::{CallInfo, ReturnValue, Signature, SyntaxShape};

use crate::handle;
use crate::handle::s3_helper;

impl Plugin for handle::Handle {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("s3")
            .desc("Load S3 resource into a cell, convert to table if possible (avoid by appending '--raw')")
            .required(
                "RESOURCE",
                SyntaxShape::String,
                "the RESOURCE to fetch the contents from",
            )
            .named(
                "access_key",
                SyntaxShape::Any,
                "the accessy key when authenticating",
                Some('a'),
            )
            .named(
                "secret_key",
                SyntaxShape::Any,
                "the secret key when authenticating",
                Some('s'),
            )
            .named(
                "region",
                SyntaxShape::Any,
                "the region of the resource",
                Some('R'),
            )
            .switch("raw", "fetch contents as text rather than a table", Some('r'))
            .switch("list-only", "only list the name of objects", Some('l'))
            .filter())
    }

    fn begin_filter(&mut self, callinfo: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        self.setup(callinfo)?;
        // Ok(vec![block_on(s3_helper(
        //     &self.path.clone().ok_or_else(|| {
        //         ShellError::labeled_error("internal error: path not set", "path not set", &self.tag)
        //     })?,
        //     self.has_raw,
        //     self.user.clone(),
        //     self.password.clone(),
        // ))])
        Ok(vec![])
    }
}
