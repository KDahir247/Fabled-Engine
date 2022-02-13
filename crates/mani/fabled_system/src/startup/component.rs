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


#[test]
fn create_component() {
    create_component!(
        struct ecs_component {
            a: String,
            b: String,
        }
    );

    let b = ecs_component {
        a: "".to_string(),
        b: "".to_string(),
    };


    println!("{:?} {:?}", b.a, b.b);
}
