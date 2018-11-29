use std::marker::PhantomData;

use specs::SystemData;
use specs::World;

trait Validation<'a> {
    type Input;
    type Output;
    type SD: SystemData<'a>;

    fn run(&mut self, i: Self::Input, data: Self::SD) -> Self::Output;

    fn exec(&mut self, i: Self::Input, world: &'a mut World) -> Self::Output {
        world.exec(|sd|
            self.run(i, sd)
        )
    }
}

struct PureValidator<I, O> {
    func: Box<Fn(I) -> O>,
    id: PhantomData<I>,
    od: PhantomData<O>,
}

impl<'a, I, O> Validation<'a> for PureValidator<I, O> {
    type Input = I;
    type Output = O;
    type SD = ();

    fn run(&mut self, i: <Self as Validation<'a>>::Input, (): <Self as Validation<'a>>::SD) -> <Self as Validation<'a>>::Output {
        let f = &self.func;
        f(i)
    }
}

struct ConsV<V1, V2> {
    v1: V1,
    v2: V2,
}

impl<'a, V1, V2, I, IO, O> Validation<'a> for ConsV<V1, V2>
    where V1: Validation<'a, Input=I, Output=IO> + 'a,
          V2: Validation<'a, Input=IO, Output=O> + 'a,
{
    type Input = I;
    type Output = O;
    type SD = (
        <V1 as Validation<'a>>::SD,
        <V2 as Validation<'a>>::SD,
    );

    fn run(&mut self, i: <Self as Validation<'a>>::Input, (v1sd, v2sd): <Self as Validation<'a>>::SD) -> <Self as Validation<'a>>::Output {
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
        let mut validator = PureValidator {
            func: Box::new(|x: i32| x.to_string()),
            id: Default::default(),
            od: Default::default(),
        };

        let res = validator.exec(5, &mut w);
        assert_eq!("5", res);
    }

    #[test]
    fn cons_validation() {
        let mut w = World::new();
        let validator1 = PureValidator {
            func: Box::new(|x: i32| x as f32 + 0.1),
            id: Default::default(),
            od: Default::default(),
        };
        let validator2 = PureValidator {
            func: Box::new(|x: f32| x.to_string()),
            id: Default::default(),
            od: Default::default(),
        };

        let mut composite = ConsV {
            v1: validator1,
            v2: validator2,
        };

        let res = composite.exec(5, &mut w);
        assert_eq!("5.1", res);
    }
}