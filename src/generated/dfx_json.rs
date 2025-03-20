#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Represents the log level of the bitcoin adapter."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents the log level of the bitcoin adapter.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"critical\","]
#[doc = "    \"error\","]
#[doc = "    \"warning\","]
#[doc = "    \"info\","]
#[doc = "    \"debug\","]
#[doc = "    \"trace\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BitcoinAdapterLogLevel {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}
impl ::std::convert::From<&Self> for BitcoinAdapterLogLevel {
    fn from(value: &BitcoinAdapterLogLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BitcoinAdapterLogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Critical => write!(f, "critical"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
            Self::Trace => write!(f, "trace"),
        }
    }
}
impl ::std::str::FromStr for BitcoinAdapterLogLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "critical" => Ok(Self::Critical),
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "trace" => Ok(Self::Trace),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BitcoinAdapterLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BitcoinAdapterLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BitcoinAdapterLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A quantity of bytes. Representable either as an integer, or as an SI unit string"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Byte Count\","]
#[doc = "  \"description\": \"A quantity of bytes. Representable either as an integer, or as an SI unit string\","]
#[doc = "  \"examples\": ["]
#[doc = "    72,"]
#[doc = "    \"2KB\","]
#[doc = "    \"4 MiB\""]
#[doc = "  ],"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"string\""]
#[doc = "  ],"]
#[doc = "  \"pattern\": \"^[0-9]+( *([KkMmGgTtPpEeZzYy]i?)?[Bb])?$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Byte {
    String(ByteString),
    Integer(i64),
}
impl ::std::convert::From<&Self> for Byte {
    fn from(value: &Byte) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Byte {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for Byte {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Byte {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Byte {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for Byte {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<ByteString> for Byte {
    fn from(value: ByteString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<i64> for Byte {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "ByteString"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9]+( *([KkMmGgTtPpEeZzYy]i?)?[Bb])?$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ByteString(::std::string::String);
impl ::std::ops::Deref for ByteString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ByteString> for ::std::string::String {
    fn from(value: ByteString) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ByteString> for ByteString {
    fn from(value: &ByteString) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ByteString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[0-9]+( *([KkMmGgTtPpEeZzYy]i?)?[Bb])?$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^[0-9]+( *([KkMmGgTtPpEeZzYy]i?)?[Bb])?$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ByteString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ByteString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ByteString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Configurations about which canister interface declarations to generate, and where to generate them."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Declarations Configuration\","]
#[doc = "  \"description\": \"Configurations about which canister interface declarations to generate, and where to generate them.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"bindings\": {"]
#[doc = "      \"title\": \"Languages to generate\","]
#[doc = "      \"description\": \"A list of languages to generate type declarations. Supported options are 'js', 'ts', 'did', 'mo'. Default is ['js', 'ts', 'did'].\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"env_override\": {"]
#[doc = "      \"title\": \"Canister ID ENV Override\","]
#[doc = "      \"description\": \"A string that will replace process.env.CANISTER_ID_{canister_name_uppercase} in the 'src/dfx/assets/language_bindings/canister.js' template.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"node_compatibility\": {"]
#[doc = "      \"title\": \"Node compatibility flag\","]
#[doc = "      \"description\": \"Flag to pre-populate generated declarations with better defaults for various types of projects Default is false\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"output\": {"]
#[doc = "      \"title\": \"Declaration Output Directory\","]
#[doc = "      \"description\": \"Directory to place declarations for that canister. Default is 'src/declarations/<canister_name>'.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CanisterDeclarationsConfig {
    #[doc = "A list of languages to generate type declarations. Supported options are 'js', 'ts', 'did', 'mo'. Default is ['js', 'ts', 'did']."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "A string that will replace process.env.CANISTER_ID_{canister_name_uppercase} in the 'src/dfx/assets/language_bindings/canister.js' template."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub env_override: ::std::option::Option<::std::string::String>,
    #[doc = "Flag to pre-populate generated declarations with better defaults for various types of projects Default is false"]
    #[serde(default)]
    pub node_compatibility: bool,
    #[doc = "Directory to place declarations for that canister. Default is 'src/declarations/<canister_name>'."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CanisterDeclarationsConfig> for CanisterDeclarationsConfig {
    fn from(value: &CanisterDeclarationsConfig) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CanisterDeclarationsConfig {
    fn default() -> Self {
        Self {
            bindings: Default::default(),
            env_override: Default::default(),
            node_compatibility: Default::default(),
            output: Default::default(),
        }
    }
}
impl CanisterDeclarationsConfig {
    pub fn builder() -> builder::CanisterDeclarationsConfig {
        Default::default()
    }
}
#[doc = "CanisterLogVisibility"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"controllers\","]
#[doc = "        \"public\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"allowed_viewers\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"allowed_viewers\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum CanisterLogVisibility {
    #[serde(rename = "controllers")]
    Controllers,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "allowed_viewers")]
    AllowedViewers(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<&Self> for CanisterLogVisibility {
    fn from(value: &CanisterLogVisibility) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for CanisterLogVisibility {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::AllowedViewers(value)
    }
}
#[doc = "Configures a custom metadata section for the canister wasm. dfx uses the first definition of a given name matching the current network, ignoring any of the same name that follow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Canister Metadata Configuration\","]
#[doc = "  \"description\": \"Configures a custom metadata section for the canister wasm. dfx uses the first definition of a given name matching the current network, ignoring any of the same name that follow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"title\": \"Content\","]
#[doc = "      \"description\": \"Content of this metadata section. Conflicts with `path`.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"description\": \"The name of the wasm section\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"networks\": {"]
#[doc = "      \"title\": \"Networks\","]
#[doc = "      \"description\": \"Networks this section applies to. If this field is absent, then it applies to all networks. An empty array means this element will not apply to any network.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"title\": \"Path\","]
#[doc = "      \"description\": \"Path to file containing section contents. Conflicts with `content`. For sections with name=`candid:service`, this field is optional, and if not specified, dfx will use the canister's candid definition. If specified for a Motoko canister, the service defined in the specified path must be a valid subtype of the canister's actual candid service definition.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"visibility\": {"]
#[doc = "      \"title\": \"Visibility\","]
#[doc = "      \"default\": \"public\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/MetadataVisibility\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CanisterMetadataSection {
    #[doc = "Content of this metadata section. Conflicts with `path`."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the wasm section"]
    pub name: ::std::string::String,
    #[doc = "Networks this section applies to. If this field is absent, then it applies to all networks. An empty array means this element will not apply to any network."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub networks: ::std::option::Option<Vec<::std::string::String>>,
    #[doc = "Path to file containing section contents. Conflicts with `content`. For sections with name=`candid:service`, this field is optional, and if not specified, dfx will use the canister's candid definition. If specified for a Motoko canister, the service defined in the specified path must be a valid subtype of the canister's actual candid service definition."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(default = "defaults::canister_metadata_section_visibility")]
    pub visibility: MetadataVisibility,
}
impl ::std::convert::From<&CanisterMetadataSection> for CanisterMetadataSection {
    fn from(value: &CanisterMetadataSection) -> Self {
        value.clone()
    }
}
impl CanisterMetadataSection {
    pub fn builder() -> builder::CanisterMetadataSection {
        Default::default()
    }
}
#[doc = "Configurations for a single canister."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Canister Configuration\","]
#[doc = "  \"description\": \"Configurations for a single canister.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Rust-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"candid\","]
#[doc = "        \"package\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"candid\": {"]
#[doc = "          \"title\": \"Candid File\","]
#[doc = "          \"description\": \"Path of this canister's candid interface declaration.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"crate\": {"]
#[doc = "          \"title\": \"Crate name\","]
#[doc = "          \"description\": \"Name of the Rust crate that compiles to this canister's Wasm. If left unspecified, defaults to the crate with the same name as the package.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"package\": {"]
#[doc = "          \"title\": \"Package Name\","]
#[doc = "          \"description\": \"Name of the Rust package that compiles this canister's Wasm.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"rust\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Asset-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"build\": {"]
#[doc = "          \"title\": \"Build Commands\","]
#[doc = "          \"description\": \"Commands that are executed in order to produce this canister's assets. Expected to produce assets in one of the paths specified by the 'source' field. Optional if there is no build necessary or the assets can be built using the default `npm run build` command.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"title\": \"Asset Source Folder\","]
#[doc = "          \"description\": \"Folders from which assets are uploaded.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"assets\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"title\": \"NPM workspace\","]
#[doc = "          \"description\": \"The workspace in package.json that this canister is in, if it is not in the root workspace.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Custom-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"candid\","]
#[doc = "        \"type\","]
#[doc = "        \"wasm\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"build\": {"]
#[doc = "          \"title\": \"Build Commands\","]
#[doc = "          \"description\": \"Commands that are executed in order to produce this canister's Wasm module. Expected to produce the Wasm in the path specified by the 'wasm' field. No build commands are allowed if the `wasm` field is a URL. These commands are executed in the root of the project.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"candid\": {"]
#[doc = "          \"title\": \"Candid File\","]
#[doc = "          \"description\": \"Path to this canister's candid interface declaration.  A URL to a candid file is also acceptable.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"custom\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"wasm\": {"]
#[doc = "          \"title\": \"Wasm Path\","]
#[doc = "          \"description\": \"Path to Wasm to be installed. URLs to a Wasm module are also acceptable. A canister that has a URL to a Wasm module can not also have `build` steps.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Motoko-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"motoko\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Pull-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"title\": \"Canister ID\","]
#[doc = "          \"description\": \"Principal of the canister on the ic network.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"pull\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"args\": {"]
#[doc = "      \"title\": \"Canister-Specific Build Argument\","]
#[doc = "      \"description\": \"This field defines an additional argument to pass to the Motoko compiler when building the canister.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"declarations\": {"]
#[doc = "      \"title\": \"Declarations Configuration\","]
#[doc = "      \"description\": \"Defines which canister interface declarations to generate, and where to generate them.\","]
#[doc = "      \"default\": {"]
#[doc = "        \"bindings\": null,"]
#[doc = "        \"env_override\": null,"]
#[doc = "        \"node_compatibility\": false,"]
#[doc = "        \"output\": null"]
#[doc = "      },"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CanisterDeclarationsConfig\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"dependencies\": {"]
#[doc = "      \"title\": \"Dependencies\","]
#[doc = "      \"description\": \"Defines on which canisters this canister depends on.\","]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"frontend\": {"]
#[doc = "      \"title\": \"Force Frontend URL\","]
#[doc = "      \"description\": \"Mostly unused. If this value is not null, a frontend URL is displayed after deployment even if the canister type is not 'asset'.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"gzip\": {"]
#[doc = "      \"title\": \"Gzip Canister Wasm\","]
#[doc = "      \"description\": \"Disabled by default.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"init_arg\": {"]
#[doc = "      \"title\": \"Init Arg\","]
#[doc = "      \"description\": \"The Candid initialization argument for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg` field will be ignored.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"init_arg_file\": {"]
#[doc = "      \"title\": \"Init Arg File\","]
#[doc = "      \"description\": \"The Candid initialization argument file for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg_file` field will be ignored.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"initialization_values\": {"]
#[doc = "      \"title\": \"Resource Allocation Settings\","]
#[doc = "      \"description\": \"Defines initial values for resource allocation settings.\","]
#[doc = "      \"default\": {"]
#[doc = "        \"compute_allocation\": null,"]
#[doc = "        \"freezing_threshold\": null,"]
#[doc = "        \"log_visibility\": null,"]
#[doc = "        \"memory_allocation\": null,"]
#[doc = "        \"reserved_cycles_limit\": null,"]
#[doc = "        \"wasm_memory_limit\": null"]
#[doc = "      },"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/InitializationValues\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"main\": {"]
#[doc = "      \"title\": \"Path to Canister Entry Point\","]
#[doc = "      \"description\": \"Entry point for e.g. Motoko Compiler.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"title\": \"Metadata\","]
#[doc = "      \"description\": \"Defines metadata sections to set in the canister .wasm\","]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/CanisterMetadataSection\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"optimize\": {"]
#[doc = "      \"title\": \"Optimize Canister Wasm\","]
#[doc = "      \"description\": \"Invoke wasm level optimizations after building the canister. Optimization level can be set to \\\"cycles\\\" to optimize for cycle usage, \\\"size\\\" to optimize for binary size, or any of \\\"O4, O3, O2, O1, O0, Oz, Os\\\". Disabled by default. If this option is specified, the `shrink` option will be ignored.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/WasmOptLevel\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"post_install\": {"]
#[doc = "      \"title\": \"Post-Install Commands\","]
#[doc = "      \"description\": \"One or more commands to run post canister installation. These commands are executed in the root of the project.\","]
#[doc = "      \"default\": [],"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pullable\": {"]
#[doc = "      \"title\": \"Pullable\","]
#[doc = "      \"description\": \"Defines required properties so that this canister is ready for `dfx deps pull` by other projects.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Pullable\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"remote\": {"]
#[doc = "      \"title\": \"Remote Configuration\","]
#[doc = "      \"description\": \"Used to mark the canister as 'remote' on certain networks.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigCanistersCanisterRemote\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"shrink\": {"]
#[doc = "      \"title\": \"Shrink Canister Wasm\","]
#[doc = "      \"description\": \"Whether run `ic-wasm shrink` after building the Canister. Enabled by default for Rust/Motoko canisters. Disabled by default for custom canisters.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"specified_id\": {"]
#[doc = "      \"title\": \"Specified Canister ID\","]
#[doc = "      \"description\": \"Attempts to create the canister with this Canister ID. This option only works with non-mainnet replica. If the `--specified-id` argument is also provided, this `specified_id` field will be ignored.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tech_stack\": {"]
#[doc = "      \"title\": \"Tech Stack\","]
#[doc = "      \"description\": \"Defines the tech stack used to build this canister.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TechStack\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConfigCanistersCanister {
    Variant0(ConfigCanistersCanisterVariant0),
    Variant1(ConfigCanistersCanisterVariant1),
    Variant2(ConfigCanistersCanisterVariant2),
    Variant3(ConfigCanistersCanisterVariant3),
    Variant4(ConfigCanistersCanisterVariant4),
}
impl ::std::convert::From<&Self> for ConfigCanistersCanister {
    fn from(value: &ConfigCanistersCanister) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ConfigCanistersCanisterVariant0> for ConfigCanistersCanister {
    fn from(value: ConfigCanistersCanisterVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<ConfigCanistersCanisterVariant1> for ConfigCanistersCanister {
    fn from(value: ConfigCanistersCanisterVariant1) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<ConfigCanistersCanisterVariant2> for ConfigCanistersCanister {
    fn from(value: ConfigCanistersCanisterVariant2) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<ConfigCanistersCanisterVariant3> for ConfigCanistersCanister {
    fn from(value: ConfigCanistersCanisterVariant3) -> Self {
        Self::Variant3(value)
    }
}
impl ::std::convert::From<ConfigCanistersCanisterVariant4> for ConfigCanistersCanister {
    fn from(value: ConfigCanistersCanisterVariant4) -> Self {
        Self::Variant4(value)
    }
}
#[doc = "This field allows canisters to be marked 'remote' for certain networks. On networks where this canister contains a remote ID, the canister is not deployed. Instead it is assumed to exist already under control of a different project."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Remote Canister Configuration\","]
#[doc = "  \"description\": \"This field allows canisters to be marked 'remote' for certain networks. On networks where this canister contains a remote ID, the canister is not deployed. Instead it is assumed to exist already under control of a different project.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"candid\": {"]
#[doc = "      \"title\": \"Remote Candid File\","]
#[doc = "      \"description\": \"On networks where this canister is marked 'remote', this candid file is used instead of the one declared in the canister settings.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"Network to Remote ID Mapping\","]
#[doc = "      \"description\": \"This field contains mappings from network names to remote canister IDs (Principals). For all networks listed here, this canister is considered 'remote'.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigCanistersCanisterRemote {
    #[doc = "On networks where this canister is marked 'remote', this candid file is used instead of the one declared in the canister settings."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub candid: ::std::option::Option<::std::string::String>,
    #[doc = "This field contains mappings from network names to remote canister IDs (Principals). For all networks listed here, this canister is considered 'remote'."]
    pub id: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ::std::convert::From<&ConfigCanistersCanisterRemote> for ConfigCanistersCanisterRemote {
    fn from(value: &ConfigCanistersCanisterRemote) -> Self {
        value.clone()
    }
}
impl ConfigCanistersCanisterRemote {
    pub fn builder() -> builder::ConfigCanistersCanisterRemote {
        Default::default()
    }
}
#[doc = "ConfigCanistersCanisterVariant0"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"args\": {"]
#[doc = "          \"title\": \"Canister-Specific Build Argument\","]
#[doc = "          \"description\": \"This field defines an additional argument to pass to the Motoko compiler when building the canister.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"declarations\": {"]
#[doc = "          \"title\": \"Declarations Configuration\","]
#[doc = "          \"description\": \"Defines which canister interface declarations to generate, and where to generate them.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"bindings\": null,"]
#[doc = "            \"env_override\": null,"]
#[doc = "            \"node_compatibility\": false,"]
#[doc = "            \"output\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CanisterDeclarationsConfig\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"dependencies\": {"]
#[doc = "          \"title\": \"Dependencies\","]
#[doc = "          \"description\": \"Defines on which canisters this canister depends on.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"frontend\": {"]
#[doc = "          \"title\": \"Force Frontend URL\","]
#[doc = "          \"description\": \"Mostly unused. If this value is not null, a frontend URL is displayed after deployment even if the canister type is not 'asset'.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"object\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"gzip\": {"]
#[doc = "          \"title\": \"Gzip Canister Wasm\","]
#[doc = "          \"description\": \"Disabled by default.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg\": {"]
#[doc = "          \"title\": \"Init Arg\","]
#[doc = "          \"description\": \"The Candid initialization argument for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg_file\": {"]
#[doc = "          \"title\": \"Init Arg File\","]
#[doc = "          \"description\": \"The Candid initialization argument file for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg_file` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"initialization_values\": {"]
#[doc = "          \"title\": \"Resource Allocation Settings\","]
#[doc = "          \"description\": \"Defines initial values for resource allocation settings.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"compute_allocation\": null,"]
#[doc = "            \"freezing_threshold\": null,"]
#[doc = "            \"log_visibility\": null,"]
#[doc = "            \"memory_allocation\": null,"]
#[doc = "            \"reserved_cycles_limit\": null,"]
#[doc = "            \"wasm_memory_limit\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/InitializationValues\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"main\": {"]
#[doc = "          \"title\": \"Path to Canister Entry Point\","]
#[doc = "          \"description\": \"Entry point for e.g. Motoko Compiler.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"title\": \"Metadata\","]
#[doc = "          \"description\": \"Defines metadata sections to set in the canister .wasm\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/CanisterMetadataSection\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"optimize\": {"]
#[doc = "          \"title\": \"Optimize Canister Wasm\","]
#[doc = "          \"description\": \"Invoke wasm level optimizations after building the canister. Optimization level can be set to \\\"cycles\\\" to optimize for cycle usage, \\\"size\\\" to optimize for binary size, or any of \\\"O4, O3, O2, O1, O0, Oz, Os\\\". Disabled by default. If this option is specified, the `shrink` option will be ignored.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/WasmOptLevel\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"post_install\": {"]
#[doc = "          \"title\": \"Post-Install Commands\","]
#[doc = "          \"description\": \"One or more commands to run post canister installation. These commands are executed in the root of the project.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pullable\": {"]
#[doc = "          \"title\": \"Pullable\","]
#[doc = "          \"description\": \"Defines required properties so that this canister is ready for `dfx deps pull` by other projects.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Pullable\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"remote\": {"]
#[doc = "          \"title\": \"Remote Configuration\","]
#[doc = "          \"description\": \"Used to mark the canister as 'remote' on certain networks.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ConfigCanistersCanisterRemote\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"shrink\": {"]
#[doc = "          \"title\": \"Shrink Canister Wasm\","]
#[doc = "          \"description\": \"Whether run `ic-wasm shrink` after building the Canister. Enabled by default for Rust/Motoko canisters. Disabled by default for custom canisters.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"specified_id\": {"]
#[doc = "          \"title\": \"Specified Canister ID\","]
#[doc = "          \"description\": \"Attempts to create the canister with this Canister ID. This option only works with non-mainnet replica. If the `--specified-id` argument is also provided, this `specified_id` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tech_stack\": {"]
#[doc = "          \"title\": \"Tech Stack\","]
#[doc = "          \"description\": \"Defines the tech stack used to build this canister.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/TechStack\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Rust-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"candid\","]
#[doc = "        \"package\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"candid\": {"]
#[doc = "          \"title\": \"Candid File\","]
#[doc = "          \"description\": \"Path of this canister's candid interface declaration.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"crate\": {"]
#[doc = "          \"title\": \"Crate name\","]
#[doc = "          \"description\": \"Name of the Rust crate that compiles to this canister's Wasm. If left unspecified, defaults to the crate with the same name as the package.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"package\": {"]
#[doc = "          \"title\": \"Package Name\","]
#[doc = "          \"description\": \"Name of the Rust package that compiles this canister's Wasm.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"rust\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Asset-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"source\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's assets. Expected to produce assets in one of the paths specified by the 'source' field. Optional if there is no build necessary or the assets can be built using the default `npm run build` command.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"title\": \"Asset Source Folder\","]
#[doc = "            \"description\": \"Folders from which assets are uploaded.\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"assets\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"workspace\": {"]
#[doc = "            \"title\": \"NPM workspace\","]
#[doc = "            \"description\": \"The workspace in package.json that this canister is in, if it is not in the root workspace.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Custom-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"type\","]
#[doc = "          \"wasm\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's Wasm module. Expected to produce the Wasm in the path specified by the 'wasm' field. No build commands are allowed if the `wasm` field is a URL. These commands are executed in the root of the project.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path to this canister's candid interface declaration.  A URL to a candid file is also acceptable.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"custom\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"wasm\": {"]
#[doc = "            \"title\": \"Wasm Path\","]
#[doc = "            \"description\": \"Path to Wasm to be installed. URLs to a Wasm module are also acceptable. A canister that has a URL to a Wasm module can not also have `build` steps.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Motoko-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"motoko\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Pull-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"id\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"id\": {"]
#[doc = "            \"title\": \"Canister ID\","]
#[doc = "            \"description\": \"Principal of the canister on the ic network.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"pull\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum ConfigCanistersCanisterVariant0 {}
impl ::std::convert::From<&Self> for ConfigCanistersCanisterVariant0 {
    fn from(value: &ConfigCanistersCanisterVariant0) -> Self {
        value.clone()
    }
}
#[doc = "ConfigCanistersCanisterVariant1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"args\": {"]
#[doc = "          \"title\": \"Canister-Specific Build Argument\","]
#[doc = "          \"description\": \"This field defines an additional argument to pass to the Motoko compiler when building the canister.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"declarations\": {"]
#[doc = "          \"title\": \"Declarations Configuration\","]
#[doc = "          \"description\": \"Defines which canister interface declarations to generate, and where to generate them.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"bindings\": null,"]
#[doc = "            \"env_override\": null,"]
#[doc = "            \"node_compatibility\": false,"]
#[doc = "            \"output\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CanisterDeclarationsConfig\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"dependencies\": {"]
#[doc = "          \"title\": \"Dependencies\","]
#[doc = "          \"description\": \"Defines on which canisters this canister depends on.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"frontend\": {"]
#[doc = "          \"title\": \"Force Frontend URL\","]
#[doc = "          \"description\": \"Mostly unused. If this value is not null, a frontend URL is displayed after deployment even if the canister type is not 'asset'.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"object\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"gzip\": {"]
#[doc = "          \"title\": \"Gzip Canister Wasm\","]
#[doc = "          \"description\": \"Disabled by default.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg\": {"]
#[doc = "          \"title\": \"Init Arg\","]
#[doc = "          \"description\": \"The Candid initialization argument for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg_file\": {"]
#[doc = "          \"title\": \"Init Arg File\","]
#[doc = "          \"description\": \"The Candid initialization argument file for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg_file` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"initialization_values\": {"]
#[doc = "          \"title\": \"Resource Allocation Settings\","]
#[doc = "          \"description\": \"Defines initial values for resource allocation settings.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"compute_allocation\": null,"]
#[doc = "            \"freezing_threshold\": null,"]
#[doc = "            \"log_visibility\": null,"]
#[doc = "            \"memory_allocation\": null,"]
#[doc = "            \"reserved_cycles_limit\": null,"]
#[doc = "            \"wasm_memory_limit\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/InitializationValues\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"main\": {"]
#[doc = "          \"title\": \"Path to Canister Entry Point\","]
#[doc = "          \"description\": \"Entry point for e.g. Motoko Compiler.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"title\": \"Metadata\","]
#[doc = "          \"description\": \"Defines metadata sections to set in the canister .wasm\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/CanisterMetadataSection\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"optimize\": {"]
#[doc = "          \"title\": \"Optimize Canister Wasm\","]
#[doc = "          \"description\": \"Invoke wasm level optimizations after building the canister. Optimization level can be set to \\\"cycles\\\" to optimize for cycle usage, \\\"size\\\" to optimize for binary size, or any of \\\"O4, O3, O2, O1, O0, Oz, Os\\\". Disabled by default. If this option is specified, the `shrink` option will be ignored.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/WasmOptLevel\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"post_install\": {"]
#[doc = "          \"title\": \"Post-Install Commands\","]
#[doc = "          \"description\": \"One or more commands to run post canister installation. These commands are executed in the root of the project.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pullable\": {"]
#[doc = "          \"title\": \"Pullable\","]
#[doc = "          \"description\": \"Defines required properties so that this canister is ready for `dfx deps pull` by other projects.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Pullable\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"remote\": {"]
#[doc = "          \"title\": \"Remote Configuration\","]
#[doc = "          \"description\": \"Used to mark the canister as 'remote' on certain networks.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ConfigCanistersCanisterRemote\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"shrink\": {"]
#[doc = "          \"title\": \"Shrink Canister Wasm\","]
#[doc = "          \"description\": \"Whether run `ic-wasm shrink` after building the Canister. Enabled by default for Rust/Motoko canisters. Disabled by default for custom canisters.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"specified_id\": {"]
#[doc = "          \"title\": \"Specified Canister ID\","]
#[doc = "          \"description\": \"Attempts to create the canister with this Canister ID. This option only works with non-mainnet replica. If the `--specified-id` argument is also provided, this `specified_id` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tech_stack\": {"]
#[doc = "          \"title\": \"Tech Stack\","]
#[doc = "          \"description\": \"Defines the tech stack used to build this canister.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/TechStack\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Asset-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"build\": {"]
#[doc = "          \"title\": \"Build Commands\","]
#[doc = "          \"description\": \"Commands that are executed in order to produce this canister's assets. Expected to produce assets in one of the paths specified by the 'source' field. Optional if there is no build necessary or the assets can be built using the default `npm run build` command.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"title\": \"Asset Source Folder\","]
#[doc = "          \"description\": \"Folders from which assets are uploaded.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"assets\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"title\": \"NPM workspace\","]
#[doc = "          \"description\": \"The workspace in package.json that this canister is in, if it is not in the root workspace.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Rust-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"package\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path of this canister's candid interface declaration.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"crate\": {"]
#[doc = "            \"title\": \"Crate name\","]
#[doc = "            \"description\": \"Name of the Rust crate that compiles to this canister's Wasm. If left unspecified, defaults to the crate with the same name as the package.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"package\": {"]
#[doc = "            \"title\": \"Package Name\","]
#[doc = "            \"description\": \"Name of the Rust package that compiles this canister's Wasm.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"rust\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Custom-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"type\","]
#[doc = "          \"wasm\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's Wasm module. Expected to produce the Wasm in the path specified by the 'wasm' field. No build commands are allowed if the `wasm` field is a URL. These commands are executed in the root of the project.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path to this canister's candid interface declaration.  A URL to a candid file is also acceptable.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"custom\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"wasm\": {"]
#[doc = "            \"title\": \"Wasm Path\","]
#[doc = "            \"description\": \"Path to Wasm to be installed. URLs to a Wasm module are also acceptable. A canister that has a URL to a Wasm module can not also have `build` steps.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Motoko-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"motoko\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Pull-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"id\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"id\": {"]
#[doc = "            \"title\": \"Canister ID\","]
#[doc = "            \"description\": \"Principal of the canister on the ic network.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"pull\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum ConfigCanistersCanisterVariant1 {}
impl ::std::convert::From<&Self> for ConfigCanistersCanisterVariant1 {
    fn from(value: &ConfigCanistersCanisterVariant1) -> Self {
        value.clone()
    }
}
#[doc = "ConfigCanistersCanisterVariant2"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"args\": {"]
#[doc = "          \"title\": \"Canister-Specific Build Argument\","]
#[doc = "          \"description\": \"This field defines an additional argument to pass to the Motoko compiler when building the canister.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"declarations\": {"]
#[doc = "          \"title\": \"Declarations Configuration\","]
#[doc = "          \"description\": \"Defines which canister interface declarations to generate, and where to generate them.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"bindings\": null,"]
#[doc = "            \"env_override\": null,"]
#[doc = "            \"node_compatibility\": false,"]
#[doc = "            \"output\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CanisterDeclarationsConfig\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"dependencies\": {"]
#[doc = "          \"title\": \"Dependencies\","]
#[doc = "          \"description\": \"Defines on which canisters this canister depends on.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"frontend\": {"]
#[doc = "          \"title\": \"Force Frontend URL\","]
#[doc = "          \"description\": \"Mostly unused. If this value is not null, a frontend URL is displayed after deployment even if the canister type is not 'asset'.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"object\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"gzip\": {"]
#[doc = "          \"title\": \"Gzip Canister Wasm\","]
#[doc = "          \"description\": \"Disabled by default.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg\": {"]
#[doc = "          \"title\": \"Init Arg\","]
#[doc = "          \"description\": \"The Candid initialization argument for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg_file\": {"]
#[doc = "          \"title\": \"Init Arg File\","]
#[doc = "          \"description\": \"The Candid initialization argument file for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg_file` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"initialization_values\": {"]
#[doc = "          \"title\": \"Resource Allocation Settings\","]
#[doc = "          \"description\": \"Defines initial values for resource allocation settings.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"compute_allocation\": null,"]
#[doc = "            \"freezing_threshold\": null,"]
#[doc = "            \"log_visibility\": null,"]
#[doc = "            \"memory_allocation\": null,"]
#[doc = "            \"reserved_cycles_limit\": null,"]
#[doc = "            \"wasm_memory_limit\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/InitializationValues\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"main\": {"]
#[doc = "          \"title\": \"Path to Canister Entry Point\","]
#[doc = "          \"description\": \"Entry point for e.g. Motoko Compiler.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"title\": \"Metadata\","]
#[doc = "          \"description\": \"Defines metadata sections to set in the canister .wasm\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/CanisterMetadataSection\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"optimize\": {"]
#[doc = "          \"title\": \"Optimize Canister Wasm\","]
#[doc = "          \"description\": \"Invoke wasm level optimizations after building the canister. Optimization level can be set to \\\"cycles\\\" to optimize for cycle usage, \\\"size\\\" to optimize for binary size, or any of \\\"O4, O3, O2, O1, O0, Oz, Os\\\". Disabled by default. If this option is specified, the `shrink` option will be ignored.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/WasmOptLevel\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"post_install\": {"]
#[doc = "          \"title\": \"Post-Install Commands\","]
#[doc = "          \"description\": \"One or more commands to run post canister installation. These commands are executed in the root of the project.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pullable\": {"]
#[doc = "          \"title\": \"Pullable\","]
#[doc = "          \"description\": \"Defines required properties so that this canister is ready for `dfx deps pull` by other projects.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Pullable\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"remote\": {"]
#[doc = "          \"title\": \"Remote Configuration\","]
#[doc = "          \"description\": \"Used to mark the canister as 'remote' on certain networks.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ConfigCanistersCanisterRemote\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"shrink\": {"]
#[doc = "          \"title\": \"Shrink Canister Wasm\","]
#[doc = "          \"description\": \"Whether run `ic-wasm shrink` after building the Canister. Enabled by default for Rust/Motoko canisters. Disabled by default for custom canisters.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"specified_id\": {"]
#[doc = "          \"title\": \"Specified Canister ID\","]
#[doc = "          \"description\": \"Attempts to create the canister with this Canister ID. This option only works with non-mainnet replica. If the `--specified-id` argument is also provided, this `specified_id` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tech_stack\": {"]
#[doc = "          \"title\": \"Tech Stack\","]
#[doc = "          \"description\": \"Defines the tech stack used to build this canister.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/TechStack\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Custom-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"candid\","]
#[doc = "        \"type\","]
#[doc = "        \"wasm\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"build\": {"]
#[doc = "          \"title\": \"Build Commands\","]
#[doc = "          \"description\": \"Commands that are executed in order to produce this canister's Wasm module. Expected to produce the Wasm in the path specified by the 'wasm' field. No build commands are allowed if the `wasm` field is a URL. These commands are executed in the root of the project.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"candid\": {"]
#[doc = "          \"title\": \"Candid File\","]
#[doc = "          \"description\": \"Path to this canister's candid interface declaration.  A URL to a candid file is also acceptable.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"custom\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"wasm\": {"]
#[doc = "          \"title\": \"Wasm Path\","]
#[doc = "          \"description\": \"Path to Wasm to be installed. URLs to a Wasm module are also acceptable. A canister that has a URL to a Wasm module can not also have `build` steps.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Rust-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"package\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path of this canister's candid interface declaration.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"crate\": {"]
#[doc = "            \"title\": \"Crate name\","]
#[doc = "            \"description\": \"Name of the Rust crate that compiles to this canister's Wasm. If left unspecified, defaults to the crate with the same name as the package.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"package\": {"]
#[doc = "            \"title\": \"Package Name\","]
#[doc = "            \"description\": \"Name of the Rust package that compiles this canister's Wasm.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"rust\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Asset-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"source\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's assets. Expected to produce assets in one of the paths specified by the 'source' field. Optional if there is no build necessary or the assets can be built using the default `npm run build` command.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"title\": \"Asset Source Folder\","]
#[doc = "            \"description\": \"Folders from which assets are uploaded.\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"assets\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"workspace\": {"]
#[doc = "            \"title\": \"NPM workspace\","]
#[doc = "            \"description\": \"The workspace in package.json that this canister is in, if it is not in the root workspace.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Motoko-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"motoko\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Pull-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"id\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"id\": {"]
#[doc = "            \"title\": \"Canister ID\","]
#[doc = "            \"description\": \"Principal of the canister on the ic network.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"pull\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum ConfigCanistersCanisterVariant2 {}
impl ::std::convert::From<&Self> for ConfigCanistersCanisterVariant2 {
    fn from(value: &ConfigCanistersCanisterVariant2) -> Self {
        value.clone()
    }
}
#[doc = "ConfigCanistersCanisterVariant3"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"args\": {"]
#[doc = "          \"title\": \"Canister-Specific Build Argument\","]
#[doc = "          \"description\": \"This field defines an additional argument to pass to the Motoko compiler when building the canister.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"declarations\": {"]
#[doc = "          \"title\": \"Declarations Configuration\","]
#[doc = "          \"description\": \"Defines which canister interface declarations to generate, and where to generate them.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"bindings\": null,"]
#[doc = "            \"env_override\": null,"]
#[doc = "            \"node_compatibility\": false,"]
#[doc = "            \"output\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CanisterDeclarationsConfig\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"dependencies\": {"]
#[doc = "          \"title\": \"Dependencies\","]
#[doc = "          \"description\": \"Defines on which canisters this canister depends on.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"frontend\": {"]
#[doc = "          \"title\": \"Force Frontend URL\","]
#[doc = "          \"description\": \"Mostly unused. If this value is not null, a frontend URL is displayed after deployment even if the canister type is not 'asset'.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"object\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"gzip\": {"]
#[doc = "          \"title\": \"Gzip Canister Wasm\","]
#[doc = "          \"description\": \"Disabled by default.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg\": {"]
#[doc = "          \"title\": \"Init Arg\","]
#[doc = "          \"description\": \"The Candid initialization argument for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg_file\": {"]
#[doc = "          \"title\": \"Init Arg File\","]
#[doc = "          \"description\": \"The Candid initialization argument file for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg_file` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"initialization_values\": {"]
#[doc = "          \"title\": \"Resource Allocation Settings\","]
#[doc = "          \"description\": \"Defines initial values for resource allocation settings.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"compute_allocation\": null,"]
#[doc = "            \"freezing_threshold\": null,"]
#[doc = "            \"log_visibility\": null,"]
#[doc = "            \"memory_allocation\": null,"]
#[doc = "            \"reserved_cycles_limit\": null,"]
#[doc = "            \"wasm_memory_limit\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/InitializationValues\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"main\": {"]
#[doc = "          \"title\": \"Path to Canister Entry Point\","]
#[doc = "          \"description\": \"Entry point for e.g. Motoko Compiler.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"title\": \"Metadata\","]
#[doc = "          \"description\": \"Defines metadata sections to set in the canister .wasm\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/CanisterMetadataSection\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"optimize\": {"]
#[doc = "          \"title\": \"Optimize Canister Wasm\","]
#[doc = "          \"description\": \"Invoke wasm level optimizations after building the canister. Optimization level can be set to \\\"cycles\\\" to optimize for cycle usage, \\\"size\\\" to optimize for binary size, or any of \\\"O4, O3, O2, O1, O0, Oz, Os\\\". Disabled by default. If this option is specified, the `shrink` option will be ignored.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/WasmOptLevel\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"post_install\": {"]
#[doc = "          \"title\": \"Post-Install Commands\","]
#[doc = "          \"description\": \"One or more commands to run post canister installation. These commands are executed in the root of the project.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pullable\": {"]
#[doc = "          \"title\": \"Pullable\","]
#[doc = "          \"description\": \"Defines required properties so that this canister is ready for `dfx deps pull` by other projects.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Pullable\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"remote\": {"]
#[doc = "          \"title\": \"Remote Configuration\","]
#[doc = "          \"description\": \"Used to mark the canister as 'remote' on certain networks.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ConfigCanistersCanisterRemote\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"shrink\": {"]
#[doc = "          \"title\": \"Shrink Canister Wasm\","]
#[doc = "          \"description\": \"Whether run `ic-wasm shrink` after building the Canister. Enabled by default for Rust/Motoko canisters. Disabled by default for custom canisters.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"specified_id\": {"]
#[doc = "          \"title\": \"Specified Canister ID\","]
#[doc = "          \"description\": \"Attempts to create the canister with this Canister ID. This option only works with non-mainnet replica. If the `--specified-id` argument is also provided, this `specified_id` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tech_stack\": {"]
#[doc = "          \"title\": \"Tech Stack\","]
#[doc = "          \"description\": \"Defines the tech stack used to build this canister.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/TechStack\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Motoko-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"motoko\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Rust-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"package\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path of this canister's candid interface declaration.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"crate\": {"]
#[doc = "            \"title\": \"Crate name\","]
#[doc = "            \"description\": \"Name of the Rust crate that compiles to this canister's Wasm. If left unspecified, defaults to the crate with the same name as the package.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"package\": {"]
#[doc = "            \"title\": \"Package Name\","]
#[doc = "            \"description\": \"Name of the Rust package that compiles this canister's Wasm.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"rust\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Asset-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"source\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's assets. Expected to produce assets in one of the paths specified by the 'source' field. Optional if there is no build necessary or the assets can be built using the default `npm run build` command.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"title\": \"Asset Source Folder\","]
#[doc = "            \"description\": \"Folders from which assets are uploaded.\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"assets\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"workspace\": {"]
#[doc = "            \"title\": \"NPM workspace\","]
#[doc = "            \"description\": \"The workspace in package.json that this canister is in, if it is not in the root workspace.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Custom-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"type\","]
#[doc = "          \"wasm\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's Wasm module. Expected to produce the Wasm in the path specified by the 'wasm' field. No build commands are allowed if the `wasm` field is a URL. These commands are executed in the root of the project.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path to this canister's candid interface declaration.  A URL to a candid file is also acceptable.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"custom\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"wasm\": {"]
#[doc = "            \"title\": \"Wasm Path\","]
#[doc = "            \"description\": \"Path to Wasm to be installed. URLs to a Wasm module are also acceptable. A canister that has a URL to a Wasm module can not also have `build` steps.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Pull-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"id\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"id\": {"]
#[doc = "            \"title\": \"Canister ID\","]
#[doc = "            \"description\": \"Principal of the canister on the ic network.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"pull\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum ConfigCanistersCanisterVariant3 {}
impl ::std::convert::From<&Self> for ConfigCanistersCanisterVariant3 {
    fn from(value: &ConfigCanistersCanisterVariant3) -> Self {
        value.clone()
    }
}
#[doc = "ConfigCanistersCanisterVariant4"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"args\": {"]
#[doc = "          \"title\": \"Canister-Specific Build Argument\","]
#[doc = "          \"description\": \"This field defines an additional argument to pass to the Motoko compiler when building the canister.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"declarations\": {"]
#[doc = "          \"title\": \"Declarations Configuration\","]
#[doc = "          \"description\": \"Defines which canister interface declarations to generate, and where to generate them.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"bindings\": null,"]
#[doc = "            \"env_override\": null,"]
#[doc = "            \"node_compatibility\": false,"]
#[doc = "            \"output\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/CanisterDeclarationsConfig\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"dependencies\": {"]
#[doc = "          \"title\": \"Dependencies\","]
#[doc = "          \"description\": \"Defines on which canisters this canister depends on.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"frontend\": {"]
#[doc = "          \"title\": \"Force Frontend URL\","]
#[doc = "          \"description\": \"Mostly unused. If this value is not null, a frontend URL is displayed after deployment even if the canister type is not 'asset'.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"object\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"gzip\": {"]
#[doc = "          \"title\": \"Gzip Canister Wasm\","]
#[doc = "          \"description\": \"Disabled by default.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg\": {"]
#[doc = "          \"title\": \"Init Arg\","]
#[doc = "          \"description\": \"The Candid initialization argument for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"init_arg_file\": {"]
#[doc = "          \"title\": \"Init Arg File\","]
#[doc = "          \"description\": \"The Candid initialization argument file for installing the canister. If the `--argument` or `--argument-file` argument is also provided, this `init_arg_file` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"initialization_values\": {"]
#[doc = "          \"title\": \"Resource Allocation Settings\","]
#[doc = "          \"description\": \"Defines initial values for resource allocation settings.\","]
#[doc = "          \"default\": {"]
#[doc = "            \"compute_allocation\": null,"]
#[doc = "            \"freezing_threshold\": null,"]
#[doc = "            \"log_visibility\": null,"]
#[doc = "            \"memory_allocation\": null,"]
#[doc = "            \"reserved_cycles_limit\": null,"]
#[doc = "            \"wasm_memory_limit\": null"]
#[doc = "          },"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/InitializationValues\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"main\": {"]
#[doc = "          \"title\": \"Path to Canister Entry Point\","]
#[doc = "          \"description\": \"Entry point for e.g. Motoko Compiler.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"title\": \"Metadata\","]
#[doc = "          \"description\": \"Defines metadata sections to set in the canister .wasm\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/CanisterMetadataSection\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"optimize\": {"]
#[doc = "          \"title\": \"Optimize Canister Wasm\","]
#[doc = "          \"description\": \"Invoke wasm level optimizations after building the canister. Optimization level can be set to \\\"cycles\\\" to optimize for cycle usage, \\\"size\\\" to optimize for binary size, or any of \\\"O4, O3, O2, O1, O0, Oz, Os\\\". Disabled by default. If this option is specified, the `shrink` option will be ignored.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/WasmOptLevel\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"post_install\": {"]
#[doc = "          \"title\": \"Post-Install Commands\","]
#[doc = "          \"description\": \"One or more commands to run post canister installation. These commands are executed in the root of the project.\","]
#[doc = "          \"default\": [],"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pullable\": {"]
#[doc = "          \"title\": \"Pullable\","]
#[doc = "          \"description\": \"Defines required properties so that this canister is ready for `dfx deps pull` by other projects.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Pullable\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"remote\": {"]
#[doc = "          \"title\": \"Remote Configuration\","]
#[doc = "          \"description\": \"Used to mark the canister as 'remote' on certain networks.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ConfigCanistersCanisterRemote\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"shrink\": {"]
#[doc = "          \"title\": \"Shrink Canister Wasm\","]
#[doc = "          \"description\": \"Whether run `ic-wasm shrink` after building the Canister. Enabled by default for Rust/Motoko canisters. Disabled by default for custom canisters.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"boolean\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"specified_id\": {"]
#[doc = "          \"title\": \"Specified Canister ID\","]
#[doc = "          \"description\": \"Attempts to create the canister with this Canister ID. This option only works with non-mainnet replica. If the `--specified-id` argument is also provided, this `specified_id` field will be ignored.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tech_stack\": {"]
#[doc = "          \"title\": \"Tech Stack\","]
#[doc = "          \"description\": \"Defines the tech stack used to build this canister.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/TechStack\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Pull-Specific Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"title\": \"Canister ID\","]
#[doc = "          \"description\": \"Principal of the canister on the ic network.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"pull\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Rust-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"package\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path of this canister's candid interface declaration.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"crate\": {"]
#[doc = "            \"title\": \"Crate name\","]
#[doc = "            \"description\": \"Name of the Rust crate that compiles to this canister's Wasm. If left unspecified, defaults to the crate with the same name as the package.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"package\": {"]
#[doc = "            \"title\": \"Package Name\","]
#[doc = "            \"description\": \"Name of the Rust package that compiles this canister's Wasm.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"rust\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Asset-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"source\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's assets. Expected to produce assets in one of the paths specified by the 'source' field. Optional if there is no build necessary or the assets can be built using the default `npm run build` command.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"source\": {"]
#[doc = "            \"title\": \"Asset Source Folder\","]
#[doc = "            \"description\": \"Folders from which assets are uploaded.\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"assets\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"workspace\": {"]
#[doc = "            \"title\": \"NPM workspace\","]
#[doc = "            \"description\": \"The workspace in package.json that this canister is in, if it is not in the root workspace.\","]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Custom-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"candid\","]
#[doc = "          \"type\","]
#[doc = "          \"wasm\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"build\": {"]
#[doc = "            \"title\": \"Build Commands\","]
#[doc = "            \"description\": \"Commands that are executed in order to produce this canister's Wasm module. Expected to produce the Wasm in the path specified by the 'wasm' field. No build commands are allowed if the `wasm` field is a URL. These commands are executed in the root of the project.\","]
#[doc = "            \"default\": [],"]
#[doc = "            \"allOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"candid\": {"]
#[doc = "            \"title\": \"Candid File\","]
#[doc = "            \"description\": \"Path to this canister's candid interface declaration.  A URL to a candid file is also acceptable.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"custom\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"wasm\": {"]
#[doc = "            \"title\": \"Wasm Path\","]
#[doc = "            \"description\": \"Path to Wasm to be installed. URLs to a Wasm module are also acceptable. A canister that has a URL to a Wasm module can not also have `build` steps.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"title\": \"Motoko-Specific Properties\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"motoko\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum ConfigCanistersCanisterVariant4 {}
impl ::std::convert::From<&Self> for ConfigCanistersCanisterVariant4 {
    fn from(value: &ConfigCanistersCanisterVariant4) -> Self {
        value.clone()
    }
}
#[doc = "Defaults to use on dfx start."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defaults to use on dfx start.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"bitcoin\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsBitcoin\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"bootstrap\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsBootstrap\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"build\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsBuild\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"canister_http\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsCanisterHttp\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"proxy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsProxy\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"replica\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsReplica\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigDefaults {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bitcoin: ::std::option::Option<ConfigDefaultsBitcoin>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bootstrap: ::std::option::Option<ConfigDefaultsBootstrap>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub build: ::std::option::Option<ConfigDefaultsBuild>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub canister_http: ::std::option::Option<ConfigDefaultsCanisterHttp>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proxy: ::std::option::Option<ConfigDefaultsProxy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub replica: ::std::option::Option<ConfigDefaultsReplica>,
}
impl ::std::convert::From<&ConfigDefaults> for ConfigDefaults {
    fn from(value: &ConfigDefaults) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigDefaults {
    fn default() -> Self {
        Self {
            bitcoin: Default::default(),
            bootstrap: Default::default(),
            build: Default::default(),
            canister_http: Default::default(),
            proxy: Default::default(),
            replica: Default::default(),
        }
    }
}
impl ConfigDefaults {
    pub fn builder() -> builder::ConfigDefaults {
        Default::default()
    }
}
#[doc = "ConfigDefaultsBitcoin"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Bitcoin Adapter Configuration\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"canister_init_arg\": {"]
#[doc = "      \"title\": \"Initialization Argument\","]
#[doc = "      \"description\": \"The initialization argument for the bitcoin canister.\","]
#[doc = "      \"default\": \"(record { stability_threshold = 0 : nat; network = variant { regtest }; blocks_source = principal \\\"aaaaa-aa\\\"; fees = record { get_utxos_base = 0 : nat; get_utxos_cycles_per_ten_instructions = 0 : nat; get_utxos_maximum = 0 : nat; get_balance = 0 : nat; get_balance_maximum = 0 : nat; get_current_fee_percentiles = 0 : nat; get_current_fee_percentiles_maximum = 0 : nat;  send_transaction_base = 0 : nat; send_transaction_per_byte = 0 : nat; }; syncing = variant { enabled }; api_access = variant { enabled }; disable_api_if_not_fully_synced = variant { enabled }})\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"title\": \"Enable Bitcoin Adapter\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"log_level\": {"]
#[doc = "      \"title\": \"Logging Level\","]
#[doc = "      \"description\": \"The logging level of the adapter.\","]
#[doc = "      \"default\": \"info\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/BitcoinAdapterLogLevel\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"title\": \"Available Nodes\","]
#[doc = "      \"description\": \"Addresses of nodes to connect to (in case discovery from seeds is not possible/sufficient).\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigDefaultsBitcoin {
    #[doc = "The initialization argument for the bitcoin canister."]
    #[serde(default = "defaults::config_defaults_bitcoin_canister_init_arg")]
    pub canister_init_arg: ::std::string::String,
    #[serde(default)]
    pub enabled: bool,
    #[doc = "The logging level of the adapter."]
    #[serde(default = "defaults::config_defaults_bitcoin_log_level")]
    pub log_level: BitcoinAdapterLogLevel,
    #[doc = "Addresses of nodes to connect to (in case discovery from seeds is not possible/sufficient)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nodes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ::std::convert::From<&ConfigDefaultsBitcoin> for ConfigDefaultsBitcoin {
    fn from(value: &ConfigDefaultsBitcoin) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigDefaultsBitcoin {
    fn default() -> Self {
        Self {
            canister_init_arg: defaults::config_defaults_bitcoin_canister_init_arg(),
            enabled: Default::default(),
            log_level: defaults::config_defaults_bitcoin_log_level(),
            nodes: Default::default(),
        }
    }
}
impl ConfigDefaultsBitcoin {
    pub fn builder() -> builder::ConfigDefaultsBitcoin {
        Default::default()
    }
}
#[doc = "The bootstrap command has been removed.  All of these fields are ignored."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Bootstrap Server Configuration\","]
#[doc = "  \"description\": \"The bootstrap command has been removed.  All of these fields are ignored.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"ip\": {"]
#[doc = "      \"description\": \"Specifies the IP address that the bootstrap server listens on. Defaults to 127.0.0.1.\","]
#[doc = "      \"default\": \"127.0.0.1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"ip\""]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"Specifies the port number that the bootstrap server listens on. Defaults to 8081.\","]
#[doc = "      \"default\": 8081,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"timeout\": {"]
#[doc = "      \"description\": \"Specifies the maximum number of seconds that the bootstrap server will wait for upstream requests to complete. Defaults to 30.\","]
#[doc = "      \"default\": 30,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigDefaultsBootstrap {
    #[doc = "Specifies the IP address that the bootstrap server listens on. Defaults to 127.0.0.1."]
    #[serde(default = "defaults::config_defaults_bootstrap_ip")]
    pub ip: std::net::IpAddr,
    #[doc = "Specifies the port number that the bootstrap server listens on. Defaults to 8081."]
    #[serde(default = "defaults::default_u64::<u16, 8081>")]
    pub port: u16,
    #[doc = "Specifies the maximum number of seconds that the bootstrap server will wait for upstream requests to complete. Defaults to 30."]
    #[serde(default = "defaults::default_u64::<u64, 30>")]
    pub timeout: u64,
}
impl ::std::convert::From<&ConfigDefaultsBootstrap> for ConfigDefaultsBootstrap {
    fn from(value: &ConfigDefaultsBootstrap) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigDefaultsBootstrap {
    fn default() -> Self {
        Self {
            ip: defaults::config_defaults_bootstrap_ip(),
            port: defaults::default_u64::<u16, 8081>(),
            timeout: defaults::default_u64::<u64, 30>(),
        }
    }
}
impl ConfigDefaultsBootstrap {
    pub fn builder() -> builder::ConfigDefaultsBootstrap {
        Default::default()
    }
}
#[doc = "ConfigDefaultsBuild"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Build Process Configuration\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"args\": {"]
#[doc = "      \"description\": \"Arguments for packtool.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"packtool\": {"]
#[doc = "      \"description\": \"Main command to run the packtool. This command is executed in the root of the project.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigDefaultsBuild {
    #[doc = "Arguments for packtool."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub args: ::std::option::Option<::std::string::String>,
    #[doc = "Main command to run the packtool. This command is executed in the root of the project."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub packtool: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ConfigDefaultsBuild> for ConfigDefaultsBuild {
    fn from(value: &ConfigDefaultsBuild) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigDefaultsBuild {
    fn default() -> Self {
        Self {
            args: Default::default(),
            packtool: Default::default(),
        }
    }
}
impl ConfigDefaultsBuild {
    pub fn builder() -> builder::ConfigDefaultsBuild {
        Default::default()
    }
}
#[doc = "ConfigDefaultsCanisterHttp"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"HTTP Adapter Configuration\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"title\": \"Enable HTTP Adapter\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"log_level\": {"]
#[doc = "      \"title\": \"Logging Level\","]
#[doc = "      \"description\": \"The logging level of the adapter.\","]
#[doc = "      \"default\": \"error\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/HttpAdapterLogLevel\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigDefaultsCanisterHttp {
    #[serde(default = "defaults::default_bool::<true>")]
    pub enabled: bool,
    #[doc = "The logging level of the adapter."]
    #[serde(default = "defaults::config_defaults_canister_http_log_level")]
    pub log_level: HttpAdapterLogLevel,
}
impl ::std::convert::From<&ConfigDefaultsCanisterHttp> for ConfigDefaultsCanisterHttp {
    fn from(value: &ConfigDefaultsCanisterHttp) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigDefaultsCanisterHttp {
    fn default() -> Self {
        Self {
            enabled: defaults::default_bool::<true>(),
            log_level: defaults::config_defaults_canister_http_log_level(),
        }
    }
}
impl ConfigDefaultsCanisterHttp {
    pub fn builder() -> builder::ConfigDefaultsCanisterHttp {
        Default::default()
    }
}
#[doc = "Configuration for the HTTP gateway."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Configuration for the HTTP gateway.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"domain\": {"]
#[doc = "      \"description\": \"A list of domains that can be served. These are used for canister resolution [default: localhost]\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/SerdeVec_for_String\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigDefaultsProxy {
    #[doc = "A list of domains that can be served. These are used for canister resolution [default: localhost]"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub domain: ::std::option::Option<SerdeVecForString>,
}
impl ::std::convert::From<&ConfigDefaultsProxy> for ConfigDefaultsProxy {
    fn from(value: &ConfigDefaultsProxy) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigDefaultsProxy {
    fn default() -> Self {
        Self {
            domain: Default::default(),
        }
    }
}
impl ConfigDefaultsProxy {
    pub fn builder() -> builder::ConfigDefaultsProxy {
        Default::default()
    }
}
#[doc = "ConfigDefaultsReplica"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Local Replica Configuration\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"log_level\": {"]
#[doc = "      \"description\": \"Run replica with the provided log level. Default is 'error'. Debug prints still get displayed\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReplicaLogLevel\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"Port the replica listens on.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"subnet_type\": {"]
#[doc = "      \"title\": \"Subnet Type\","]
#[doc = "      \"description\": \"Determines the subnet type the replica will run as. Affects things like cycles accounting, message size limits, cycle limits. Defaults to 'application'.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ReplicaSubnetType\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigDefaultsReplica {
    #[doc = "Run replica with the provided log level. Default is 'error'. Debug prints still get displayed"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub log_level: ::std::option::Option<ReplicaLogLevel>,
    #[doc = "Port the replica listens on."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub port: ::std::option::Option<u16>,
    #[doc = "Determines the subnet type the replica will run as. Affects things like cycles accounting, message size limits, cycle limits. Defaults to 'application'."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subnet_type: ::std::option::Option<ReplicaSubnetType>,
}
impl ::std::convert::From<&ConfigDefaultsReplica> for ConfigDefaultsReplica {
    fn from(value: &ConfigDefaultsReplica) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigDefaultsReplica {
    fn default() -> Self {
        Self {
            log_level: Default::default(),
            port: Default::default(),
            subnet_type: Default::default(),
        }
    }
}
impl ConfigDefaultsReplica {
    pub fn builder() -> builder::ConfigDefaultsReplica {
        Default::default()
    }
}
#[doc = "ConfigLocalProvider"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Local Replica Configuration\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"bind\": {"]
#[doc = "      \"description\": \"Bind address for the webserver. For the shared local network, the default is 127.0.0.1:4943. For project-specific local networks, the default is 127.0.0.1:8000.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"bitcoin\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsBitcoin\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"bootstrap\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsBootstrap\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"canister_http\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsCanisterHttp\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"playground\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PlaygroundConfig\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"proxy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsProxy\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"replica\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaultsReplica\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"Persistence type of this network.\","]
#[doc = "      \"default\": \"ephemeral\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/NetworkType\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigLocalProvider {
    #[doc = "Bind address for the webserver. For the shared local network, the default is 127.0.0.1:4943. For project-specific local networks, the default is 127.0.0.1:8000."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bind: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bitcoin: ::std::option::Option<ConfigDefaultsBitcoin>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bootstrap: ::std::option::Option<ConfigDefaultsBootstrap>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub canister_http: ::std::option::Option<ConfigDefaultsCanisterHttp>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub playground: ::std::option::Option<PlaygroundConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proxy: ::std::option::Option<ConfigDefaultsProxy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub replica: ::std::option::Option<ConfigDefaultsReplica>,
    #[doc = "Persistence type of this network."]
    #[serde(rename = "type", default = "defaults::config_local_provider_type")]
    pub type_: NetworkType,
}
impl ::std::convert::From<&ConfigLocalProvider> for ConfigLocalProvider {
    fn from(value: &ConfigLocalProvider) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConfigLocalProvider {
    fn default() -> Self {
        Self {
            bind: Default::default(),
            bitcoin: Default::default(),
            bootstrap: Default::default(),
            canister_http: Default::default(),
            playground: Default::default(),
            proxy: Default::default(),
            replica: Default::default(),
            type_: defaults::config_local_provider_type(),
        }
    }
}
impl ConfigLocalProvider {
    pub fn builder() -> builder::ConfigLocalProvider {
        Default::default()
    }
}
#[doc = "ConfigNetwork"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ConfigNetworkProvider\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ConfigLocalProvider\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConfigNetwork {
    NetworkProvider(ConfigNetworkProvider),
    LocalProvider(ConfigLocalProvider),
}
impl ::std::convert::From<&Self> for ConfigNetwork {
    fn from(value: &ConfigNetwork) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ConfigNetworkProvider> for ConfigNetwork {
    fn from(value: ConfigNetworkProvider) -> Self {
        Self::NetworkProvider(value)
    }
}
impl ::std::convert::From<ConfigLocalProvider> for ConfigNetwork {
    fn from(value: ConfigLocalProvider) -> Self {
        Self::LocalProvider(value)
    }
}
#[doc = "ConfigNetworkProvider"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Custom Network Configuration\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"providers\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"playground\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PlaygroundConfig\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"providers\": {"]
#[doc = "      \"description\": \"The URL(s) this network can be reached at.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"Persistence type of this network.\","]
#[doc = "      \"default\": \"persistent\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/NetworkType\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConfigNetworkProvider {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub playground: ::std::option::Option<PlaygroundConfig>,
    #[doc = "The URL(s) this network can be reached at."]
    pub providers: ::std::vec::Vec<::std::string::String>,
    #[doc = "Persistence type of this network."]
    #[serde(rename = "type", default = "defaults::config_network_provider_type")]
    pub type_: NetworkType,
}
impl ::std::convert::From<&ConfigNetworkProvider> for ConfigNetworkProvider {
    fn from(value: &ConfigNetworkProvider) -> Self {
        value.clone()
    }
}
impl ConfigNetworkProvider {
    pub fn builder() -> builder::ConfigNetworkProvider {
        Default::default()
    }
}
#[doc = "DfxJson"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"dfx.json\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"canisters\": {"]
#[doc = "      \"description\": \"Mapping between canisters and their settings.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/ConfigCanistersCanister\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"defaults\": {"]
#[doc = "      \"description\": \"Defaults for dfx start.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ConfigDefaults\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"dfx\": {"]
#[doc = "      \"title\": \"dfx version\","]
#[doc = "      \"description\": \"Pins the dfx version for this project.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"networks\": {"]
#[doc = "      \"description\": \"Mapping between network names and their configurations. Networks 'ic' and 'local' are implicitly defined.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/ConfigNetwork\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"output_env_file\": {"]
#[doc = "      \"description\": \"If set, environment variables will be output to this file (without overwriting any user-defined variables, if the file already exists).\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"profile\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Profile\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"Used to keep track of dfx.json versions.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DfxJson {
    #[doc = "Mapping between canisters and their settings."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub canisters: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ConfigCanistersCanister>,
    >,
    #[doc = "Defaults for dfx start."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub defaults: ::std::option::Option<ConfigDefaults>,
    #[doc = "Pins the dfx version for this project."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dfx: ::std::option::Option<::std::string::String>,
    #[doc = "Mapping between network names and their configurations. Networks 'ic' and 'local' are implicitly defined."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub networks:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, ConfigNetwork>>,
    #[doc = "If set, environment variables will be output to this file (without overwriting any user-defined variables, if the file already exists)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub output_env_file: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub profile: ::std::option::Option<Profile>,
    #[doc = "Used to keep track of dfx.json versions."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<u32>,
}
impl ::std::convert::From<&DfxJson> for DfxJson {
    fn from(value: &DfxJson) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DfxJson {
    fn default() -> Self {
        Self {
            canisters: Default::default(),
            defaults: Default::default(),
            dfx: Default::default(),
            networks: Default::default(),
            output_env_file: Default::default(),
            profile: Default::default(),
            version: Default::default(),
        }
    }
}
impl DfxJson {
    pub fn builder() -> builder::DfxJson {
        Default::default()
    }
}
#[doc = "Represents the log level of the HTTP adapter."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents the log level of the HTTP adapter.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"critical\","]
#[doc = "    \"error\","]
#[doc = "    \"warning\","]
#[doc = "    \"info\","]
#[doc = "    \"debug\","]
#[doc = "    \"trace\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HttpAdapterLogLevel {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}
impl ::std::convert::From<&Self> for HttpAdapterLogLevel {
    fn from(value: &HttpAdapterLogLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for HttpAdapterLogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Critical => write!(f, "critical"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
            Self::Trace => write!(f, "trace"),
        }
    }
}
impl ::std::str::FromStr for HttpAdapterLogLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "critical" => Ok(Self::Critical),
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "trace" => Ok(Self::Trace),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HttpAdapterLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HttpAdapterLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HttpAdapterLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "InitializationValues"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Initial Resource Allocations\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"compute_allocation\": {"]
#[doc = "      \"title\": \"Compute Allocation\","]
#[doc = "      \"description\": \"Must be a number between 0 and 100, inclusively. It indicates how much compute power should be guaranteed to this canister, expressed as a percentage of the maximum compute power that a single canister can allocate.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PossiblyStr_for_uint64\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"freezing_threshold\": {"]
#[doc = "      \"title\": \"Freezing Threshold\","]
#[doc = "      \"description\": \"Freezing threshould of the canister, measured in seconds. Valid inputs are numbers (seconds) or strings parsable by humantime (e.g. \\\"15days 2min 2s\\\").\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"log_visibility\": {"]
#[doc = "      \"title\": \"Log Visibility\","]
#[doc = "      \"description\": \"Specifies who is allowed to read the canister's logs.\\n\\nCan be \\\"public\\\", \\\"controllers\\\" or \\\"allowed_viewers\\\" with a list of principals.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/CanisterLogVisibility\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"memory_allocation\": {"]
#[doc = "      \"title\": \"Memory Allocation\","]
#[doc = "      \"description\": \"Maximum memory (in bytes) this canister is allowed to occupy. Can be specified as an integer, or as an SI unit string (e.g. \\\"4KB\\\", \\\"2 MiB\\\")\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Byte\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"reserved_cycles_limit\": {"]
#[doc = "      \"title\": \"Reserved Cycles Limit\","]
#[doc = "      \"description\": \"Specifies the upper limit of the canister's reserved cycles balance.\\n\\nReserved cycles are cycles that the system sets aside for future use by the canister. If a subnet's storage exceeds 450 GiB, then every time a canister allocates new storage bytes, the system sets aside some amount of cycles from the main balance of the canister. These reserved cycles will be used to cover future payments for the newly allocated bytes. The reserved cycles are not transferable and the amount of reserved cycles depends on how full the subnet is.\\n\\nA setting of 0 means that the canister will trap if it tries to allocate new storage while the subnet's memory usage exceeds 450 GiB.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint128\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"wasm_memory_limit\": {"]
#[doc = "      \"title\": \"Wasm Memory Limit\","]
#[doc = "      \"description\": \"Specifies a soft limit (in bytes) on the Wasm memory usage of the canister.\\n\\nUpdate calls, timers, heartbeats, installs, and post-upgrades fail if the Wasm memory usage exceeds this limit. The main purpose of this setting is to protect against the case when the canister reaches the hard 4GiB limit.\\n\\nMust be a number of bytes between 0 and 2^48 (i.e. 256 TiB), inclusive. Can be specified as an integer, or as an SI unit string (e.g. \\\"4KB\\\", \\\"2 MiB\\\")\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Byte\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializationValues {
    #[doc = "Must be a number between 0 and 100, inclusively. It indicates how much compute power should be guaranteed to this canister, expressed as a percentage of the maximum compute power that a single canister can allocate."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub compute_allocation: ::std::option::Option<PossiblyStrForUint64>,
    #[doc = "Freezing threshould of the canister, measured in seconds. Valid inputs are numbers (seconds) or strings parsable by humantime (e.g. \"15days 2min 2s\")."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub freezing_threshold: ::std::option::Option<::std::string::String>,
    #[doc = "Specifies who is allowed to read the canister's logs.\n\nCan be \"public\", \"controllers\" or \"allowed_viewers\" with a list of principals."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub log_visibility: ::std::option::Option<CanisterLogVisibility>,
    #[doc = "Maximum memory (in bytes) this canister is allowed to occupy. Can be specified as an integer, or as an SI unit string (e.g. \"4KB\", \"2 MiB\")"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_allocation: ::std::option::Option<Byte>,
    #[doc = "Specifies the upper limit of the canister's reserved cycles balance.\n\nReserved cycles are cycles that the system sets aside for future use by the canister. If a subnet's storage exceeds 450 GiB, then every time a canister allocates new storage bytes, the system sets aside some amount of cycles from the main balance of the canister. These reserved cycles will be used to cover future payments for the newly allocated bytes. The reserved cycles are not transferable and the amount of reserved cycles depends on how full the subnet is.\n\nA setting of 0 means that the canister will trap if it tries to allocate new storage while the subnet's memory usage exceeds 450 GiB."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reserved_cycles_limit: ::std::option::Option<u64>,
    #[doc = "Specifies a soft limit (in bytes) on the Wasm memory usage of the canister.\n\nUpdate calls, timers, heartbeats, installs, and post-upgrades fail if the Wasm memory usage exceeds this limit. The main purpose of this setting is to protect against the case when the canister reaches the hard 4GiB limit.\n\nMust be a number of bytes between 0 and 2^48 (i.e. 256 TiB), inclusive. Can be specified as an integer, or as an SI unit string (e.g. \"4KB\", \"2 MiB\")"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wasm_memory_limit: ::std::option::Option<Byte>,
}
impl ::std::convert::From<&InitializationValues> for InitializationValues {
    fn from(value: &InitializationValues) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for InitializationValues {
    fn default() -> Self {
        Self {
            compute_allocation: Default::default(),
            freezing_threshold: Default::default(),
            log_visibility: Default::default(),
            memory_allocation: Default::default(),
            reserved_cycles_limit: Default::default(),
            wasm_memory_limit: Default::default(),
        }
    }
}
impl InitializationValues {
    pub fn builder() -> builder::InitializationValues {
        Default::default()
    }
}
#[doc = "MetadataVisibility"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Anyone can query the metadata\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"public\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Only the controllers of the canister can query the metadata.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"private\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MetadataVisibility {
    #[doc = "Anyone can query the metadata"]
    #[serde(rename = "public")]
    Public,
    #[doc = "Only the controllers of the canister can query the metadata."]
    #[serde(rename = "private")]
    Private,
}
impl ::std::convert::From<&Self> for MetadataVisibility {
    fn from(value: &MetadataVisibility) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MetadataVisibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
        }
    }
}
impl ::std::str::FromStr for MetadataVisibility {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MetadataVisibility {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MetadataVisibility {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MetadataVisibility {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type 'ephemeral' is used for networks that are regularly reset. Type 'persistent' is used for networks that last for a long time and where it is preferred that canister IDs get stored in source control."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Network Type\","]
#[doc = "  \"description\": \"Type 'ephemeral' is used for networks that are regularly reset. Type 'persistent' is used for networks that last for a long time and where it is preferred that canister IDs get stored in source control.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ephemeral\","]
#[doc = "    \"persistent\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NetworkType {
    #[serde(rename = "ephemeral")]
    Ephemeral,
    #[serde(rename = "persistent")]
    Persistent,
}
impl ::std::convert::From<&Self> for NetworkType {
    fn from(value: &NetworkType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ephemeral => write!(f, "ephemeral"),
            Self::Persistent => write!(f, "persistent"),
        }
    }
}
impl ::std::str::FromStr for NetworkType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ephemeral" => Ok(Self::Ephemeral),
            "persistent" => Ok(Self::Persistent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NetworkType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NetworkType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NetworkType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Playground config to borrow canister from instead of creating new canisters."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Playground config to borrow canister from instead of creating new canisters.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"playground_canister\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"playground_canister\": {"]
#[doc = "      \"description\": \"Canister ID of the playground canister\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timeout_seconds\": {"]
#[doc = "      \"description\": \"How many seconds a canister can be borrowed for\","]
#[doc = "      \"default\": 1200,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PlaygroundConfig {
    #[doc = "Canister ID of the playground canister"]
    pub playground_canister: ::std::string::String,
    #[doc = "How many seconds a canister can be borrowed for"]
    #[serde(default = "defaults::default_u64::<u64, 1200>")]
    pub timeout_seconds: u64,
}
impl ::std::convert::From<&PlaygroundConfig> for PlaygroundConfig {
    fn from(value: &PlaygroundConfig) -> Self {
        value.clone()
    }
}
impl PlaygroundConfig {
    pub fn builder() -> builder::PlaygroundConfig {
        Default::default()
    }
}
#[doc = "PossiblyStrForUint64"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint64\","]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PossiblyStrForUint64(pub u64);
impl ::std::ops::Deref for PossiblyStrForUint64 {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<PossiblyStrForUint64> for u64 {
    fn from(value: PossiblyStrForUint64) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PossiblyStrForUint64> for PossiblyStrForUint64 {
    fn from(value: &PossiblyStrForUint64) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<u64> for PossiblyStrForUint64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PossiblyStrForUint64 {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PossiblyStrForUint64 {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for PossiblyStrForUint64 {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PossiblyStrForUint64 {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for PossiblyStrForUint64 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "Profile"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Debug\","]
#[doc = "    \"Release\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Profile {
    Debug,
    Release,
}
impl ::std::convert::From<&Self> for Profile {
    fn from(value: &Profile) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Profile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Debug => write!(f, "Debug"),
            Self::Release => write!(f, "Release"),
        }
    }
}
impl ::std::str::FromStr for Profile {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Debug" => Ok(Self::Debug),
            "Release" => Ok(Self::Release),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Profile {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Profile {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Profile {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Pullable"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dependencies\","]
#[doc = "    \"init_guide\","]
#[doc = "    \"wasm_url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dependencies\": {"]
#[doc = "      \"title\": \"dependencies\","]
#[doc = "      \"description\": \"Canister IDs (Principal) of direct dependencies.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"init_arg\": {"]
#[doc = "      \"title\": \"init_arg\","]
#[doc = "      \"description\": \"A default initialization argument for the canister that consumers can use.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"init_guide\": {"]
#[doc = "      \"title\": \"init_guide\","]
#[doc = "      \"description\": \"A message to guide consumers how to initialize the canister.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"wasm_hash\": {"]
#[doc = "      \"title\": \"wasm_hash\","]
#[doc = "      \"description\": \"SHA256 hash of the wasm module located at wasm_url. Only define this if the on-chain canister wasm is expected not to match the wasm at wasm_url. The hash can also be specified via a URL using the `wasm_hash_url` field. If both are defined, the `wasm_hash_url` field will be ignored.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"wasm_hash_url\": {"]
#[doc = "      \"title\": \"wasm_hash_url\","]
#[doc = "      \"description\": \"Specify the SHA256 hash of the wasm module via this URL. Only define this if the on-chain canister wasm is expected not to match the wasm at wasm_url. The hash can also be specified directly using the `wasm_hash` field. If both are defined, the `wasm_hash_url` field will be ignored.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"wasm_url\": {"]
#[doc = "      \"title\": \"wasm_url\","]
#[doc = "      \"description\": \"The Url to download canister wasm.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Pullable {
    #[doc = "Canister IDs (Principal) of direct dependencies."]
    pub dependencies: ::std::vec::Vec<::std::string::String>,
    #[doc = "A default initialization argument for the canister that consumers can use."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub init_arg: ::std::option::Option<::std::string::String>,
    #[doc = "A message to guide consumers how to initialize the canister."]
    pub init_guide: ::std::string::String,
    #[doc = "SHA256 hash of the wasm module located at wasm_url. Only define this if the on-chain canister wasm is expected not to match the wasm at wasm_url. The hash can also be specified via a URL using the `wasm_hash_url` field. If both are defined, the `wasm_hash_url` field will be ignored."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wasm_hash: ::std::option::Option<::std::string::String>,
    #[doc = "Specify the SHA256 hash of the wasm module via this URL. Only define this if the on-chain canister wasm is expected not to match the wasm at wasm_url. The hash can also be specified directly using the `wasm_hash` field. If both are defined, the `wasm_hash_url` field will be ignored."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wasm_hash_url: ::std::option::Option<::std::string::String>,
    #[doc = "The Url to download canister wasm."]
    pub wasm_url: ::std::string::String,
}
impl ::std::convert::From<&Pullable> for Pullable {
    fn from(value: &Pullable) -> Self {
        value.clone()
    }
}
impl Pullable {
    pub fn builder() -> builder::Pullable {
        Default::default()
    }
}
#[doc = "ReplicaLogLevel"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"critical\","]
#[doc = "    \"error\","]
#[doc = "    \"warning\","]
#[doc = "    \"info\","]
#[doc = "    \"debug\","]
#[doc = "    \"trace\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReplicaLogLevel {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}
impl ::std::convert::From<&Self> for ReplicaLogLevel {
    fn from(value: &ReplicaLogLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ReplicaLogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Critical => write!(f, "critical"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
            Self::Trace => write!(f, "trace"),
        }
    }
}
impl ::std::str::FromStr for ReplicaLogLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "critical" => Ok(Self::Critical),
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "trace" => Ok(Self::Trace),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReplicaLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReplicaLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReplicaLogLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ReplicaSubnetType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"system\","]
#[doc = "    \"application\","]
#[doc = "    \"verifiedapplication\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReplicaSubnetType {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "verifiedapplication")]
    Verifiedapplication,
}
impl ::std::convert::From<&Self> for ReplicaSubnetType {
    fn from(value: &ReplicaSubnetType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ReplicaSubnetType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::System => write!(f, "system"),
            Self::Application => write!(f, "application"),
            Self::Verifiedapplication => write!(f, "verifiedapplication"),
        }
    }
}
impl ::std::str::FromStr for ReplicaSubnetType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "system" => Ok(Self::System),
            "application" => Ok(Self::Application),
            "verifiedapplication" => Ok(Self::Verifiedapplication),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReplicaSubnetType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReplicaSubnetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReplicaSubnetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SerdeVecForString"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SerdeVecForString {
    Variant0(::std::string::String),
    Variant1(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<&Self> for SerdeVecForString {
    fn from(value: &SerdeVecForString) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for SerdeVecForString {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "The tech stack used to build a canister."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Tech Stack\","]
#[doc = "  \"description\": \"The tech stack used to build a canister.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cdk\": {"]
#[doc = "      \"title\": \"cdk\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"language\": {"]
#[doc = "      \"title\": \"language\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"lib\": {"]
#[doc = "      \"title\": \"lib\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"other\": {"]
#[doc = "      \"title\": \"other\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tool\": {"]
#[doc = "      \"title\": \"tool\","]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TechStack {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cdk: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lib: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub other: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tool: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
}
impl ::std::convert::From<&TechStack> for TechStack {
    fn from(value: &TechStack) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for TechStack {
    fn default() -> Self {
        Self {
            cdk: Default::default(),
            language: Default::default(),
            lib: Default::default(),
            other: Default::default(),
            tool: Default::default(),
        }
    }
}
impl TechStack {
    pub fn builder() -> builder::TechStack {
        Default::default()
    }
}
#[doc = "Wasm optimization levels that are passed to `wasm-opt`. \"cycles\" defaults to O3, \"size\" defaults to Oz. O4 through O0 focus on performance (with O0 performing no optimizations), and Oz and Os focus on reducing binary size, where Oz is more aggressive than Os. O3 and Oz empirically give best cycle savings and code size savings respectively."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Wasm Optimization Levels\","]
#[doc = "  \"description\": \"Wasm optimization levels that are passed to `wasm-opt`. \\\"cycles\\\" defaults to O3, \\\"size\\\" defaults to Oz. O4 through O0 focus on performance (with O0 performing no optimizations), and Oz and Os focus on reducing binary size, where Oz is more aggressive than Os. O3 and Oz empirically give best cycle savings and code size savings respectively.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"cycles\","]
#[doc = "    \"size\","]
#[doc = "    \"O4\","]
#[doc = "    \"O3\","]
#[doc = "    \"O2\","]
#[doc = "    \"O1\","]
#[doc = "    \"O0\","]
#[doc = "    \"Oz\","]
#[doc = "    \"Os\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WasmOptLevel {
    #[serde(rename = "cycles")]
    Cycles,
    #[serde(rename = "size")]
    Size,
    O4,
    O3,
    O2,
    O1,
    O0,
    Oz,
    Os,
}
impl ::std::convert::From<&Self> for WasmOptLevel {
    fn from(value: &WasmOptLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WasmOptLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cycles => write!(f, "cycles"),
            Self::Size => write!(f, "size"),
            Self::O4 => write!(f, "O4"),
            Self::O3 => write!(f, "O3"),
            Self::O2 => write!(f, "O2"),
            Self::O1 => write!(f, "O1"),
            Self::O0 => write!(f, "O0"),
            Self::Oz => write!(f, "Oz"),
            Self::Os => write!(f, "Os"),
        }
    }
}
impl ::std::str::FromStr for WasmOptLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cycles" => Ok(Self::Cycles),
            "size" => Ok(Self::Size),
            "O4" => Ok(Self::O4),
            "O3" => Ok(Self::O3),
            "O2" => Ok(Self::O2),
            "O1" => Ok(Self::O1),
            "O0" => Ok(Self::O0),
            "Oz" => Ok(Self::Oz),
            "Os" => Ok(Self::Os),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WasmOptLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WasmOptLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WasmOptLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CanisterDeclarationsConfig {
        bindings: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        env_override: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        node_compatibility: ::std::result::Result<bool, ::std::string::String>,
        output: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CanisterDeclarationsConfig {
        fn default() -> Self {
            Self {
                bindings: Ok(Default::default()),
                env_override: Ok(Default::default()),
                node_compatibility: Ok(Default::default()),
                output: Ok(Default::default()),
            }
        }
    }
    impl CanisterDeclarationsConfig {
        pub fn bindings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.bindings = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bindings: {}", e));
            self
        }
        pub fn env_override<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.env_override = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env_override: {}", e));
            self
        }
        pub fn node_compatibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.node_compatibility = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for node_compatibility: {}",
                    e
                )
            });
            self
        }
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CanisterDeclarationsConfig> for super::CanisterDeclarationsConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CanisterDeclarationsConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bindings: value.bindings?,
                env_override: value.env_override?,
                node_compatibility: value.node_compatibility?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::CanisterDeclarationsConfig> for CanisterDeclarationsConfig {
        fn from(value: super::CanisterDeclarationsConfig) -> Self {
            Self {
                bindings: Ok(value.bindings),
                env_override: Ok(value.env_override),
                node_compatibility: Ok(value.node_compatibility),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CanisterMetadataSection {
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        networks: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
        path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        visibility: ::std::result::Result<super::MetadataVisibility, ::std::string::String>,
    }
    impl ::std::default::Default for CanisterMetadataSection {
        fn default() -> Self {
            Self {
                content: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                networks: Ok(Default::default()),
                path: Ok(Default::default()),
                visibility: Ok(super::defaults::canister_metadata_section_visibility()),
            }
        }
    }
    impl CanisterMetadataSection {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn networks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<::std::string::String>>>,
            T::Error: ::std::fmt::Display,
        {
            self.networks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for networks: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn visibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MetadataVisibility>,
            T::Error: ::std::fmt::Display,
        {
            self.visibility = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for visibility: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CanisterMetadataSection> for super::CanisterMetadataSection {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CanisterMetadataSection,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                name: value.name?,
                networks: value.networks?,
                path: value.path?,
                visibility: value.visibility?,
            })
        }
    }
    impl ::std::convert::From<super::CanisterMetadataSection> for CanisterMetadataSection {
        fn from(value: super::CanisterMetadataSection) -> Self {
            Self {
                content: Ok(value.content),
                name: Ok(value.name),
                networks: Ok(value.networks),
                path: Ok(value.path),
                visibility: Ok(value.visibility),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigCanistersCanisterRemote {
        candid: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConfigCanistersCanisterRemote {
        fn default() -> Self {
            Self {
                candid: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl ConfigCanistersCanisterRemote {
        pub fn candid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.candid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for candid: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigCanistersCanisterRemote>
        for super::ConfigCanistersCanisterRemote
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigCanistersCanisterRemote,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                candid: value.candid?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigCanistersCanisterRemote> for ConfigCanistersCanisterRemote {
        fn from(value: super::ConfigCanistersCanisterRemote) -> Self {
            Self {
                candid: Ok(value.candid),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigDefaults {
        bitcoin: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsBitcoin>,
            ::std::string::String,
        >,
        bootstrap: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsBootstrap>,
            ::std::string::String,
        >,
        build: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsBuild>,
            ::std::string::String,
        >,
        canister_http: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsCanisterHttp>,
            ::std::string::String,
        >,
        proxy: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsProxy>,
            ::std::string::String,
        >,
        replica: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsReplica>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConfigDefaults {
        fn default() -> Self {
            Self {
                bitcoin: Ok(Default::default()),
                bootstrap: Ok(Default::default()),
                build: Ok(Default::default()),
                canister_http: Ok(Default::default()),
                proxy: Ok(Default::default()),
                replica: Ok(Default::default()),
            }
        }
    }
    impl ConfigDefaults {
        pub fn bitcoin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsBitcoin>>,
            T::Error: ::std::fmt::Display,
        {
            self.bitcoin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bitcoin: {}", e));
            self
        }
        pub fn bootstrap<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsBootstrap>>,
            T::Error: ::std::fmt::Display,
        {
            self.bootstrap = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bootstrap: {}", e));
            self
        }
        pub fn build<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsBuild>>,
            T::Error: ::std::fmt::Display,
        {
            self.build = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for build: {}", e));
            self
        }
        pub fn canister_http<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsCanisterHttp>>,
            T::Error: ::std::fmt::Display,
        {
            self.canister_http = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for canister_http: {}", e));
            self
        }
        pub fn proxy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsProxy>>,
            T::Error: ::std::fmt::Display,
        {
            self.proxy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proxy: {}", e));
            self
        }
        pub fn replica<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsReplica>>,
            T::Error: ::std::fmt::Display,
        {
            self.replica = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for replica: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigDefaults> for super::ConfigDefaults {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigDefaults,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bitcoin: value.bitcoin?,
                bootstrap: value.bootstrap?,
                build: value.build?,
                canister_http: value.canister_http?,
                proxy: value.proxy?,
                replica: value.replica?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigDefaults> for ConfigDefaults {
        fn from(value: super::ConfigDefaults) -> Self {
            Self {
                bitcoin: Ok(value.bitcoin),
                bootstrap: Ok(value.bootstrap),
                build: Ok(value.build),
                canister_http: Ok(value.canister_http),
                proxy: Ok(value.proxy),
                replica: Ok(value.replica),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigDefaultsBitcoin {
        canister_init_arg: ::std::result::Result<::std::string::String, ::std::string::String>,
        enabled: ::std::result::Result<bool, ::std::string::String>,
        log_level: ::std::result::Result<super::BitcoinAdapterLogLevel, ::std::string::String>,
        nodes: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConfigDefaultsBitcoin {
        fn default() -> Self {
            Self {
                canister_init_arg: Ok(super::defaults::config_defaults_bitcoin_canister_init_arg()),
                enabled: Ok(Default::default()),
                log_level: Ok(super::defaults::config_defaults_bitcoin_log_level()),
                nodes: Ok(Default::default()),
            }
        }
    }
    impl ConfigDefaultsBitcoin {
        pub fn canister_init_arg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.canister_init_arg = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for canister_init_arg: {}",
                    e
                )
            });
            self
        }
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {}", e));
            self
        }
        pub fn log_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BitcoinAdapterLogLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.log_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_level: {}", e));
            self
        }
        pub fn nodes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.nodes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nodes: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigDefaultsBitcoin> for super::ConfigDefaultsBitcoin {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigDefaultsBitcoin,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                canister_init_arg: value.canister_init_arg?,
                enabled: value.enabled?,
                log_level: value.log_level?,
                nodes: value.nodes?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigDefaultsBitcoin> for ConfigDefaultsBitcoin {
        fn from(value: super::ConfigDefaultsBitcoin) -> Self {
            Self {
                canister_init_arg: Ok(value.canister_init_arg),
                enabled: Ok(value.enabled),
                log_level: Ok(value.log_level),
                nodes: Ok(value.nodes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigDefaultsBootstrap {
        ip: ::std::result::Result<std::net::IpAddr, ::std::string::String>,
        port: ::std::result::Result<u16, ::std::string::String>,
        timeout: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for ConfigDefaultsBootstrap {
        fn default() -> Self {
            Self {
                ip: Ok(super::defaults::config_defaults_bootstrap_ip()),
                port: Ok(super::defaults::default_u64::<u16, 8081>()),
                timeout: Ok(super::defaults::default_u64::<u64, 30>()),
            }
        }
    }
    impl ConfigDefaultsBootstrap {
        pub fn ip<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<std::net::IpAddr>,
            T::Error: ::std::fmt::Display,
        {
            self.ip = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ip: {}", e));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.port = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for port: {}", e));
            self
        }
        pub fn timeout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigDefaultsBootstrap> for super::ConfigDefaultsBootstrap {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigDefaultsBootstrap,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ip: value.ip?,
                port: value.port?,
                timeout: value.timeout?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigDefaultsBootstrap> for ConfigDefaultsBootstrap {
        fn from(value: super::ConfigDefaultsBootstrap) -> Self {
            Self {
                ip: Ok(value.ip),
                port: Ok(value.port),
                timeout: Ok(value.timeout),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigDefaultsBuild {
        args: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        packtool: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConfigDefaultsBuild {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                packtool: Ok(Default::default()),
            }
        }
    }
    impl ConfigDefaultsBuild {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {}", e));
            self
        }
        pub fn packtool<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.packtool = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for packtool: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigDefaultsBuild> for super::ConfigDefaultsBuild {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigDefaultsBuild,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                packtool: value.packtool?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigDefaultsBuild> for ConfigDefaultsBuild {
        fn from(value: super::ConfigDefaultsBuild) -> Self {
            Self {
                args: Ok(value.args),
                packtool: Ok(value.packtool),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigDefaultsCanisterHttp {
        enabled: ::std::result::Result<bool, ::std::string::String>,
        log_level: ::std::result::Result<super::HttpAdapterLogLevel, ::std::string::String>,
    }
    impl ::std::default::Default for ConfigDefaultsCanisterHttp {
        fn default() -> Self {
            Self {
                enabled: Ok(super::defaults::default_bool::<true>()),
                log_level: Ok(super::defaults::config_defaults_canister_http_log_level()),
            }
        }
    }
    impl ConfigDefaultsCanisterHttp {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {}", e));
            self
        }
        pub fn log_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HttpAdapterLogLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.log_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_level: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigDefaultsCanisterHttp> for super::ConfigDefaultsCanisterHttp {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigDefaultsCanisterHttp,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
                log_level: value.log_level?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigDefaultsCanisterHttp> for ConfigDefaultsCanisterHttp {
        fn from(value: super::ConfigDefaultsCanisterHttp) -> Self {
            Self {
                enabled: Ok(value.enabled),
                log_level: Ok(value.log_level),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigDefaultsProxy {
        domain: ::std::result::Result<
            ::std::option::Option<super::SerdeVecForString>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConfigDefaultsProxy {
        fn default() -> Self {
            Self {
                domain: Ok(Default::default()),
            }
        }
    }
    impl ConfigDefaultsProxy {
        pub fn domain<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SerdeVecForString>>,
            T::Error: ::std::fmt::Display,
        {
            self.domain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domain: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigDefaultsProxy> for super::ConfigDefaultsProxy {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigDefaultsProxy,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                domain: value.domain?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigDefaultsProxy> for ConfigDefaultsProxy {
        fn from(value: super::ConfigDefaultsProxy) -> Self {
            Self {
                domain: Ok(value.domain),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigDefaultsReplica {
        log_level: ::std::result::Result<
            ::std::option::Option<super::ReplicaLogLevel>,
            ::std::string::String,
        >,
        port: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
        subnet_type: ::std::result::Result<
            ::std::option::Option<super::ReplicaSubnetType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConfigDefaultsReplica {
        fn default() -> Self {
            Self {
                log_level: Ok(Default::default()),
                port: Ok(Default::default()),
                subnet_type: Ok(Default::default()),
            }
        }
    }
    impl ConfigDefaultsReplica {
        pub fn log_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReplicaLogLevel>>,
            T::Error: ::std::fmt::Display,
        {
            self.log_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_level: {}", e));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u16>>,
            T::Error: ::std::fmt::Display,
        {
            self.port = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for port: {}", e));
            self
        }
        pub fn subnet_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReplicaSubnetType>>,
            T::Error: ::std::fmt::Display,
        {
            self.subnet_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subnet_type: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigDefaultsReplica> for super::ConfigDefaultsReplica {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigDefaultsReplica,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                log_level: value.log_level?,
                port: value.port?,
                subnet_type: value.subnet_type?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigDefaultsReplica> for ConfigDefaultsReplica {
        fn from(value: super::ConfigDefaultsReplica) -> Self {
            Self {
                log_level: Ok(value.log_level),
                port: Ok(value.port),
                subnet_type: Ok(value.subnet_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigLocalProvider {
        bind: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        bitcoin: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsBitcoin>,
            ::std::string::String,
        >,
        bootstrap: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsBootstrap>,
            ::std::string::String,
        >,
        canister_http: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsCanisterHttp>,
            ::std::string::String,
        >,
        playground: ::std::result::Result<
            ::std::option::Option<super::PlaygroundConfig>,
            ::std::string::String,
        >,
        proxy: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsProxy>,
            ::std::string::String,
        >,
        replica: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaultsReplica>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::NetworkType, ::std::string::String>,
    }
    impl ::std::default::Default for ConfigLocalProvider {
        fn default() -> Self {
            Self {
                bind: Ok(Default::default()),
                bitcoin: Ok(Default::default()),
                bootstrap: Ok(Default::default()),
                canister_http: Ok(Default::default()),
                playground: Ok(Default::default()),
                proxy: Ok(Default::default()),
                replica: Ok(Default::default()),
                type_: Ok(super::defaults::config_local_provider_type()),
            }
        }
    }
    impl ConfigLocalProvider {
        pub fn bind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bind: {}", e));
            self
        }
        pub fn bitcoin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsBitcoin>>,
            T::Error: ::std::fmt::Display,
        {
            self.bitcoin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bitcoin: {}", e));
            self
        }
        pub fn bootstrap<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsBootstrap>>,
            T::Error: ::std::fmt::Display,
        {
            self.bootstrap = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bootstrap: {}", e));
            self
        }
        pub fn canister_http<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsCanisterHttp>>,
            T::Error: ::std::fmt::Display,
        {
            self.canister_http = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for canister_http: {}", e));
            self
        }
        pub fn playground<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PlaygroundConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.playground = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for playground: {}", e));
            self
        }
        pub fn proxy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsProxy>>,
            T::Error: ::std::fmt::Display,
        {
            self.proxy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proxy: {}", e));
            self
        }
        pub fn replica<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaultsReplica>>,
            T::Error: ::std::fmt::Display,
        {
            self.replica = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for replica: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NetworkType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigLocalProvider> for super::ConfigLocalProvider {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigLocalProvider,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bind: value.bind?,
                bitcoin: value.bitcoin?,
                bootstrap: value.bootstrap?,
                canister_http: value.canister_http?,
                playground: value.playground?,
                proxy: value.proxy?,
                replica: value.replica?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigLocalProvider> for ConfigLocalProvider {
        fn from(value: super::ConfigLocalProvider) -> Self {
            Self {
                bind: Ok(value.bind),
                bitcoin: Ok(value.bitcoin),
                bootstrap: Ok(value.bootstrap),
                canister_http: Ok(value.canister_http),
                playground: Ok(value.playground),
                proxy: Ok(value.proxy),
                replica: Ok(value.replica),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConfigNetworkProvider {
        playground: ::std::result::Result<
            ::std::option::Option<super::PlaygroundConfig>,
            ::std::string::String,
        >,
        providers:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<super::NetworkType, ::std::string::String>,
    }
    impl ::std::default::Default for ConfigNetworkProvider {
        fn default() -> Self {
            Self {
                playground: Ok(Default::default()),
                providers: Err("no value supplied for providers".to_string()),
                type_: Ok(super::defaults::config_network_provider_type()),
            }
        }
    }
    impl ConfigNetworkProvider {
        pub fn playground<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PlaygroundConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.playground = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for playground: {}", e));
            self
        }
        pub fn providers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.providers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for providers: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NetworkType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConfigNetworkProvider> for super::ConfigNetworkProvider {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConfigNetworkProvider,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                playground: value.playground?,
                providers: value.providers?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ConfigNetworkProvider> for ConfigNetworkProvider {
        fn from(value: super::ConfigNetworkProvider) -> Self {
            Self {
                playground: Ok(value.playground),
                providers: Ok(value.providers),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DfxJson {
        canisters: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, super::ConfigCanistersCanister>,
            >,
            ::std::string::String,
        >,
        defaults: ::std::result::Result<
            ::std::option::Option<super::ConfigDefaults>,
            ::std::string::String,
        >,
        dfx: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        networks: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, super::ConfigNetwork>,
            >,
            ::std::string::String,
        >,
        output_env_file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        profile:
            ::std::result::Result<::std::option::Option<super::Profile>, ::std::string::String>,
        version: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
    }
    impl ::std::default::Default for DfxJson {
        fn default() -> Self {
            Self {
                canisters: Ok(Default::default()),
                defaults: Ok(Default::default()),
                dfx: Ok(Default::default()),
                networks: Ok(Default::default()),
                output_env_file: Ok(Default::default()),
                profile: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl DfxJson {
        pub fn canisters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            super::ConfigCanistersCanister,
                        >,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.canisters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for canisters: {}", e));
            self
        }
        pub fn defaults<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConfigDefaults>>,
            T::Error: ::std::fmt::Display,
        {
            self.defaults = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for defaults: {}", e));
            self
        }
        pub fn dfx<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.dfx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dfx: {}", e));
            self
        }
        pub fn networks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::collections::HashMap<::std::string::String, super::ConfigNetwork>,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.networks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for networks: {}", e));
            self
        }
        pub fn output_env_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_env_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_env_file: {}", e));
            self
        }
        pub fn profile<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Profile>>,
            T::Error: ::std::fmt::Display,
        {
            self.profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profile: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DfxJson> for super::DfxJson {
        type Error = super::error::ConversionError;
        fn try_from(value: DfxJson) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                canisters: value.canisters?,
                defaults: value.defaults?,
                dfx: value.dfx?,
                networks: value.networks?,
                output_env_file: value.output_env_file?,
                profile: value.profile?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::DfxJson> for DfxJson {
        fn from(value: super::DfxJson) -> Self {
            Self {
                canisters: Ok(value.canisters),
                defaults: Ok(value.defaults),
                dfx: Ok(value.dfx),
                networks: Ok(value.networks),
                output_env_file: Ok(value.output_env_file),
                profile: Ok(value.profile),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InitializationValues {
        compute_allocation: ::std::result::Result<
            ::std::option::Option<super::PossiblyStrForUint64>,
            ::std::string::String,
        >,
        freezing_threshold: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        log_visibility: ::std::result::Result<
            ::std::option::Option<super::CanisterLogVisibility>,
            ::std::string::String,
        >,
        memory_allocation:
            ::std::result::Result<::std::option::Option<super::Byte>, ::std::string::String>,
        reserved_cycles_limit:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        wasm_memory_limit:
            ::std::result::Result<::std::option::Option<super::Byte>, ::std::string::String>,
    }
    impl ::std::default::Default for InitializationValues {
        fn default() -> Self {
            Self {
                compute_allocation: Ok(Default::default()),
                freezing_threshold: Ok(Default::default()),
                log_visibility: Ok(Default::default()),
                memory_allocation: Ok(Default::default()),
                reserved_cycles_limit: Ok(Default::default()),
                wasm_memory_limit: Ok(Default::default()),
            }
        }
    }
    impl InitializationValues {
        pub fn compute_allocation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PossiblyStrForUint64>>,
            T::Error: ::std::fmt::Display,
        {
            self.compute_allocation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for compute_allocation: {}",
                    e
                )
            });
            self
        }
        pub fn freezing_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.freezing_threshold = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for freezing_threshold: {}",
                    e
                )
            });
            self
        }
        pub fn log_visibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CanisterLogVisibility>>,
            T::Error: ::std::fmt::Display,
        {
            self.log_visibility = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_visibility: {}", e));
            self
        }
        pub fn memory_allocation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Byte>>,
            T::Error: ::std::fmt::Display,
        {
            self.memory_allocation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for memory_allocation: {}",
                    e
                )
            });
            self
        }
        pub fn reserved_cycles_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.reserved_cycles_limit = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reserved_cycles_limit: {}",
                    e
                )
            });
            self
        }
        pub fn wasm_memory_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Byte>>,
            T::Error: ::std::fmt::Display,
        {
            self.wasm_memory_limit = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for wasm_memory_limit: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<InitializationValues> for super::InitializationValues {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InitializationValues,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                compute_allocation: value.compute_allocation?,
                freezing_threshold: value.freezing_threshold?,
                log_visibility: value.log_visibility?,
                memory_allocation: value.memory_allocation?,
                reserved_cycles_limit: value.reserved_cycles_limit?,
                wasm_memory_limit: value.wasm_memory_limit?,
            })
        }
    }
    impl ::std::convert::From<super::InitializationValues> for InitializationValues {
        fn from(value: super::InitializationValues) -> Self {
            Self {
                compute_allocation: Ok(value.compute_allocation),
                freezing_threshold: Ok(value.freezing_threshold),
                log_visibility: Ok(value.log_visibility),
                memory_allocation: Ok(value.memory_allocation),
                reserved_cycles_limit: Ok(value.reserved_cycles_limit),
                wasm_memory_limit: Ok(value.wasm_memory_limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PlaygroundConfig {
        playground_canister: ::std::result::Result<::std::string::String, ::std::string::String>,
        timeout_seconds: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for PlaygroundConfig {
        fn default() -> Self {
            Self {
                playground_canister: Err("no value supplied for playground_canister".to_string()),
                timeout_seconds: Ok(super::defaults::default_u64::<u64, 1200>()),
            }
        }
    }
    impl PlaygroundConfig {
        pub fn playground_canister<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.playground_canister = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for playground_canister: {}",
                    e
                )
            });
            self
        }
        pub fn timeout_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout_seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout_seconds: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PlaygroundConfig> for super::PlaygroundConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PlaygroundConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                playground_canister: value.playground_canister?,
                timeout_seconds: value.timeout_seconds?,
            })
        }
    }
    impl ::std::convert::From<super::PlaygroundConfig> for PlaygroundConfig {
        fn from(value: super::PlaygroundConfig) -> Self {
            Self {
                playground_canister: Ok(value.playground_canister),
                timeout_seconds: Ok(value.timeout_seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Pullable {
        dependencies:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        init_arg: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        init_guide: ::std::result::Result<::std::string::String, ::std::string::String>,
        wasm_hash: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        wasm_hash_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        wasm_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Pullable {
        fn default() -> Self {
            Self {
                dependencies: Err("no value supplied for dependencies".to_string()),
                init_arg: Ok(Default::default()),
                init_guide: Err("no value supplied for init_guide".to_string()),
                wasm_hash: Ok(Default::default()),
                wasm_hash_url: Ok(Default::default()),
                wasm_url: Err("no value supplied for wasm_url".to_string()),
            }
        }
    }
    impl Pullable {
        pub fn dependencies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.dependencies = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dependencies: {}", e));
            self
        }
        pub fn init_arg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.init_arg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for init_arg: {}", e));
            self
        }
        pub fn init_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.init_guide = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for init_guide: {}", e));
            self
        }
        pub fn wasm_hash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.wasm_hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wasm_hash: {}", e));
            self
        }
        pub fn wasm_hash_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.wasm_hash_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wasm_hash_url: {}", e));
            self
        }
        pub fn wasm_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.wasm_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wasm_url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Pullable> for super::Pullable {
        type Error = super::error::ConversionError;
        fn try_from(value: Pullable) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dependencies: value.dependencies?,
                init_arg: value.init_arg?,
                init_guide: value.init_guide?,
                wasm_hash: value.wasm_hash?,
                wasm_hash_url: value.wasm_hash_url?,
                wasm_url: value.wasm_url?,
            })
        }
    }
    impl ::std::convert::From<super::Pullable> for Pullable {
        fn from(value: super::Pullable) -> Self {
            Self {
                dependencies: Ok(value.dependencies),
                init_arg: Ok(value.init_arg),
                init_guide: Ok(value.init_guide),
                wasm_hash: Ok(value.wasm_hash),
                wasm_hash_url: Ok(value.wasm_hash_url),
                wasm_url: Ok(value.wasm_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TechStack {
        cdk: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            >,
            ::std::string::String,
        >,
        language: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            >,
            ::std::string::String,
        >,
        lib: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            >,
            ::std::string::String,
        >,
        other: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            >,
            ::std::string::String,
        >,
        tool: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TechStack {
        fn default() -> Self {
            Self {
                cdk: Ok(Default::default()),
                language: Ok(Default::default()),
                lib: Ok(Default::default()),
                other: Ok(Default::default()),
                tool: Ok(Default::default()),
            }
        }
    }
    impl TechStack {
        pub fn cdk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            ::std::collections::HashMap<
                                ::std::string::String,
                                ::std::string::String,
                            >,
                        >,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.cdk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cdk: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            ::std::collections::HashMap<
                                ::std::string::String,
                                ::std::string::String,
                            >,
                        >,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {}", e));
            self
        }
        pub fn lib<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            ::std::collections::HashMap<
                                ::std::string::String,
                                ::std::string::String,
                            >,
                        >,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.lib = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lib: {}", e));
            self
        }
        pub fn other<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            ::std::collections::HashMap<
                                ::std::string::String,
                                ::std::string::String,
                            >,
                        >,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.other = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for other: {}", e));
            self
        }
        pub fn tool<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            ::std::collections::HashMap<
                                ::std::string::String,
                                ::std::string::String,
                            >,
                        >,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.tool = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TechStack> for super::TechStack {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TechStack,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cdk: value.cdk?,
                language: value.language?,
                lib: value.lib?,
                other: value.other?,
                tool: value.tool?,
            })
        }
    }
    impl ::std::convert::From<super::TechStack> for TechStack {
        fn from(value: super::TechStack) -> Self {
            Self {
                cdk: Ok(value.cdk),
                language: Ok(value.language),
                lib: Ok(value.lib),
                other: Ok(value.other),
                tool: Ok(value.tool),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn canister_metadata_section_visibility() -> super::MetadataVisibility {
        super::MetadataVisibility::Public
    }
    pub(super) fn config_defaults_bitcoin_canister_init_arg() -> ::std::string::String {
        "(record { stability_threshold = 0 : nat; network = variant { regtest }; blocks_source = principal \"aaaaa-aa\"; fees = record { get_utxos_base = 0 : nat; get_utxos_cycles_per_ten_instructions = 0 : nat; get_utxos_maximum = 0 : nat; get_balance = 0 : nat; get_balance_maximum = 0 : nat; get_current_fee_percentiles = 0 : nat; get_current_fee_percentiles_maximum = 0 : nat;  send_transaction_base = 0 : nat; send_transaction_per_byte = 0 : nat; }; syncing = variant { enabled }; api_access = variant { enabled }; disable_api_if_not_fully_synced = variant { enabled }})" . to_string ()
    }
    pub(super) fn config_defaults_bitcoin_log_level() -> super::BitcoinAdapterLogLevel {
        super::BitcoinAdapterLogLevel::Info
    }
    pub(super) fn config_defaults_bootstrap_ip() -> std::net::IpAddr {
        serde_json::from_str::<std::net::IpAddr>("\"127.0.0.1\"").unwrap()
    }
    pub(super) fn config_defaults_canister_http_log_level() -> super::HttpAdapterLogLevel {
        super::HttpAdapterLogLevel::Error
    }
    pub(super) fn config_local_provider_type() -> super::NetworkType {
        super::NetworkType::Ephemeral
    }
    pub(super) fn config_network_provider_type() -> super::NetworkType {
        super::NetworkType::Persistent
    }
}
