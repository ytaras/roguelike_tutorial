use specs::prelude::*;

fn unique<T, R>(joinable: T) -> Option<R>
where
    T: Join<Type = R>,
{
    use specs::Join;

    // Not very performant?
    let mut res = joinable.join().collect::<Vec<_>>();
    assert!(res.len() <= 1);
    res.pop()
}

#[cfg(test)]
mod test {
    use super::*;
    use specs::prelude::*;
    use specs_derive::*;

    #[derive(Component, Debug, PartialEq)]
    struct MissingMarker;
    #[derive(Component, Debug)]
    struct Marker;
    #[derive(Component, Debug, PartialEq)]
    struct Data(i8);

    fn create_world() -> World {
        let mut w = World::new();
        w.register::<Marker>();
        w.register::<Data>();
        w.register::<MissingMarker>();

        w.create_entity().with(Marker).with(Data(0)).build();
        w.create_entity().with(Data(1)).build();
        w
    }
    #[test]
    fn test_unique_return_some() {
        let w = create_world();
        use specs::Join;
        let storages = (&w.read_storage::<Marker>(), &w.read_storage::<Data>());
        let (_, d) = unique(storages).unwrap();
        assert_eq!(0, d.0);
    }

    #[test]
    fn test_unique_return_none() {
        let w = create_world();
        use specs::Join;
        let storages = (
            &w.read_storage::<MissingMarker>(),
            &w.read_storage::<Data>(),
        );
        assert_eq!(None, unique(storages));
    }

    #[test]
    #[should_panic]
    fn test_unique_panics() {
        let w = create_world();
        use specs::Join;
        let storages = &w.read_storage::<Data>();
        unique(storages);
    }

}
