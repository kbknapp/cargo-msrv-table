use clap::{value_t_or_exit, arg_enum, ArgMatches};

arg_enum!{
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum TraverseOrder {
        Ascending,
        Descending
    }
}

pub(crate) struct Config {
    pub(crate) tgt_crate: String,
    pub(crate) rust_order: TraverseOrder,
    pub(crate) crate_order: TraverseOrder,
    pub(crate) eager_end: bool,
    pub(crate) timeout: Option<usize>,
    pub(crate) skip_prereleases: bool,

}

impl<'a, 'b> From<&'a ArgMatches<'b>> for Config {
    fn from(am: &ArgMatches) -> Self {
        Config {
            tgt_crate: am.value_of("CRATE").unwrap().into(),
            rust_order: value_t_or_exit!(am, "rust-order", TraverseOrder),
            crate_order: value_t_or_exit!(am, "crate-order", TraverseOrder),
            eager_end: !am.is_present("no-eager-end"),
            timeout: am.value_of("timeout").map(|v| v.parse::<usize>().unwrap()),
            skip_prereleases: !am.is_present("no-skip-prereleases"),
        }
    }
}
