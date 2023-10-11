pub mod abi;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
#[interactive_clap(skip_default_from_cli)]
pub struct AbiCommand {
    /// Include rustdocs in the ABI file
    #[interactive_clap(long)]
    pub doc: bool,
    /// Generate compact (minified) JSON
    #[interactive_clap(long)]
    pub compact_abi: bool,
    /// Copy final artifacts to this directory
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_input_arg)]
    pub out_dir: Option<crate::types::utf8_path_buf::Utf8PathBufInner>,
    /// Path to the `Cargo.toml` of the contract to build
    #[interactive_clap(long)]
    #[interactive_clap(skip_default_input_arg)]
    pub manifest_path: Option<crate::types::utf8_path_buf::Utf8PathBufInner>,
    /// Coloring: auto, always, never
    #[interactive_clap(long)]
    #[interactive_clap(value_enum)]
    #[interactive_clap(skip_default_input_arg)]
    pub color: Option<crate::common::ColorPreference>,
}

impl interactive_clap::FromCli for AbiCommand {
    type FromCliContext = near_cli_rs::GlobalContext;
    type FromCliError = color_eyre::eyre::Error;
    fn from_cli(
        optional_clap_variant: Option<<Self as interactive_clap::ToCli>::CliVariant>,
        context: Self::FromCliContext,
    ) -> interactive_clap::ResultFromCli<
        <Self as interactive_clap::ToCli>::CliVariant,
        Self::FromCliError,
    >
    where
        Self: Sized + interactive_clap::ToCli,
    {
        let mut clap_variant = optional_clap_variant.unwrap_or_default();
        let doc = clap_variant.doc;
        let compact_abi = clap_variant.compact_abi;
        if clap_variant.out_dir.is_none() {
            clap_variant.out_dir = match Self::input_out_dir(&context) {
                Ok(optional_out_dir) => optional_out_dir,
                Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
            };
        };
        let out_dir = clap_variant.out_dir.clone();
        if clap_variant.manifest_path.is_none() {
            clap_variant.manifest_path = match Self::input_manifest_path(&context) {
                Ok(optional_manifest_path) => optional_manifest_path,
                Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
            };
        };
        let manifest_path = clap_variant.manifest_path.clone();
        if clap_variant.color.is_none() {
            clap_variant.color = match Self::input_color(&context) {
                Ok(optional_color) => optional_color,
                Err(err) => return interactive_clap::ResultFromCli::Err(Some(clap_variant), err),
            };
        };
        let color = clap_variant.color.clone();

        let args = Self {
            doc,
            compact_abi,
            out_dir,
            manifest_path,
            color,
        };
        if let Err(err) = self::abi::run(args) {
            interactive_clap::ResultFromCli::Err(Some(clap_variant), color_eyre::eyre::eyre!(err))
        } else {
            interactive_clap::ResultFromCli::Ok(clap_variant)
        }
    }
}

impl AbiCommand {
    fn input_color(
        _context: &near_cli_rs::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<crate::common::ColorPreference>> {
        Ok(None)
    }

    fn input_out_dir(
        _context: &near_cli_rs::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::utf8_path_buf::Utf8PathBufInner>> {
        Ok(None)
    }

    fn input_manifest_path(
        _context: &near_cli_rs::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::utf8_path_buf::Utf8PathBufInner>> {
        Ok(None)
    }
}