use std::marker::PhantomData;

use specs::SystemData;
use specs::World;

pub trait Validation<'a> {
    type Input;
    type Output;
    type SD: SystemData<'a>;

    fn run(&self, i: Self::Input, data: Self::SD) -> Self::Output;

    fn exec(&self, i: Self::Input, world: &'a mut World) -> Self::Output {
        world.exec(|sd| self.run(i, sd))
    }

    fn register(world: &mut World) {
        <Self::SD as SystemData>::setup(&mut world.res);
    }

    fn cons<V2>(&'a self, other: &'a V2) -> ConsV<Self, V2>
    where
        V2: Validation<'a, Input = Self::Output>,
        Self: Sized,
    {
        ConsV {
            v1: &self,
            v2: other,
        }
    }
}

pub struct PureValidator<I, O> {
    func: Box<Fn(I) -> O>,
    id: PhantomData<I>,
    od: PhantomData<O>,
}

impl<I, O> PureValidator<I, O> {
    pub fn new(func: Box<Fn(I) -> O>) -> Self {
        PureValidator {
            func,
            id: Default::default(),
            od: Default::default(),
        }
    }

    pub fn from_closure<F>(f: F) -> Self
    where
        F: Fn(I) -> O + 'static,
    {
        let b = Box::new(f);
        Self::new(b)
    }
}

impl<'a, I, O> Validation<'a> for PureValidator<I, O> {
    type Input = I;
    type Output = O;
    type SD = ();

    fn run(
        &self,
        i: <Self as Validation<'a>>::Input,
        (): <Self as Validation<'a>>::SD,
    ) -> <Self as Validation<'a>>::Output {
        let f = &self.func;
        f(i)
    }
}

pub struct ConsV<'a, V1, V2>
where
    V1: 'a,
    V2: 'a,
{
    v1: &'a V1,
    v2: &'a V2,
}

impl<'a, V1, V2, I, IO, O> Validation<'a> for ConsV<'a, V1, V2>
where
    V1: Validation<'a, Input = I, Output = IO> + 'a,
    V2: Validation<'a, Input = IO, Output = O> + 'a,
{
    type Input = I;
    type Output = O;
    type SD = (<V1 as Validation<'a>>::SD, <V2 as Validation<'a>>::SD);

    fn run(
        &self,
        i: <Self as Validation<'a>>::Input,
        (v1sd, v2sd): <Self as Validation<'a>>::SD,
    ) -> <Self as Validation<'a>>::Output {
        let r1: IO = self.v1.run(i, v1sd);
        let r2: O = self.v2.run(r1, v2sd);
        r2
    }
}

#[cfg(test)]
mod test {
    use specs::World;

    use super::*;

    #[derive(Debug, Default)]
    struct MockResource {
        i: i32,
    }

    #[test]
    fn pure_validation() {
        let mut w = World::new();
        let validator = PureValidator::from_closure(|x: i32| x.to_string());

        let res = validator.exec(5, &mut w);
        assert_eq!("5", res);
    }

    #[test]
    fn cons_validation() {
        let mut w = World::new();
        let validator1 = PureValidator::from_closure(|x: i32| x as f32 + 0.1);

        let validator2 = PureValidator::from_closure(|x: f32| x.to_string());

        let composite = validator1.cons(&validator2);

        let res = composite.exec(5, &mut w);
        assert_eq!("5.1", res);
    }
}
