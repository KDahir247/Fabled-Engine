#[macro_export]
macro_rules! prop_proxy_getters {
    ($(
        $(#[$meta:meta])*
        $prop:ident -> $ty:ty {
            name = $name:expr,
            loader = $loader:expr,
            description = $description:expr,
            default: {
                $(#[$meta_default:meta])*
                $prop_default:ident = $default_value: expr
            }
        }
    )*) => {
        $(
            $(#[$meta])*
            pub fn $prop(&self) -> Result<Option<$ty>, anyhow::Error> {
                self.prop_handle
                    .get_property($name)
                    .map(|p| p.load_value($loader))
                    .transpose()
                    .map_err(|e| anyhow::format_err!("Failed to load {}: {}", $description, e))
            }

            $(#[$meta_default])*
            pub fn $prop_default(&self) -> Result<$ty, anyhow::Error> {
                self.$prop().map(|v| v.unwrap_or($default_value))
            }
        )*
    };
}
