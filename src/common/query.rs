use std::collections::HashMap;
use std::hash::Hash;

use specs::prelude::*;

pub fn singleton<T, R>(joinable: T) -> Result<R, ()>
where
    T: Join<Type = R>,
{
    unique(joinable).and_then(|opt| opt.ok_or(()))
}

pub fn unique<T, R>(joinable: T) -> Result<Option<R>, ()>
where
    T: Join<Type = R>,
{
    let mut iter = joinable.join();
    if let Some(res) = iter.next() {
        if iter.next().is_none() {
            Ok(Some(res))
        } else {
            Err(())
        }
    } else {
        Ok(None)
    }
}

pub fn hash<K, V, KK, VV>(key: K, value: V) -> HashMap<KK, VV>
where
    K: Join<Type = KK>,
    V: Join<Type = VV>,
    KK: Hash + Eq,
{
    (key, value).join().collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod test {
    use specs::prelude::*;
    use specs_derive::*;

    use super::*;

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
        let storages = (&w.read_storage::<Marker>(), &w.read_storage::<Data>());
        let (_, d) = unique(storages).unwrap().unwrap();
        assert_eq!(0, d.0);
    }

    #[test]
    fn test_unique_return_none() {
        let w = create_world();
        let storages = (
            &w.read_storage::<MissingMarker>(),
            &w.read_storage::<Data>(),
        );
        assert_eq!(None, unique(storages).unwrap());
    }

    #[test]

    fn test_unique_panics() {
        let w = create_world();
        let storage = &w.read_storage::<Data>();
        assert!(unique(storage).is_err());
    }

    #[test]
    fn test_unique_assign() {
        let w = create_world();
        {
            let storages = (&w.read_storage::<Marker>(), &mut w.write_storage::<Data>());

            if let Some((_, data)) = unique(storages).unwrap() {
                data.0 = 5;
            }
        }
        let storages = (&w.read_storage::<Marker>(), &mut w.write_storage::<Data>());

        let result = storages.join().map(|(_, d)| d).collect::<Vec<_>>();
        let expected = vec![&Data(5)];
        assert_eq!(expected, result);
    }

}
