#[macro_export]
macro_rules! create_component {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Debug)]
        struct $name {
            $($field_name: $field_type,)*
        }

        impl shipyard::Component for $name {
            type Tracking = shipyard::track::All;
        }



    }
}
