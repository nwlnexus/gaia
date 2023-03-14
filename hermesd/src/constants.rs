pub const BIN_NAME: &str = "HERMESD";
pub const DOCS_URL: &str = "https://github.com/nwlnexus/hermesd";

pub const ABOUT: &str = "
hermesd registers this node and begins polling to send/receive messaging from the controller.
Use -h for short descriptions and --help for more details.
Project home page: https://github.com/nwlnexus/hermesd
";

pub const TEMPLATE: &str = "\
{bin} {version}
{author}
{about}

USAGE:
    {usage}

{all-args}
";

pub const ABORTED_BY_USER: &str = "Aborted by user";
pub const NON_INTERACTIVE_FAILURE: &str = "This command is only available in interactive mode";