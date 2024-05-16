use crate::{Component, ComponentQuery, Context};

pub trait QueryRunner {
    fn run(&self, context: &Context) -> Vec<u64>;
}

impl<T0> QueryRunner for ComponentQuery<T0>
where
    T0: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        context.entities_with_component::<T0>()
    }
}

impl<T0, T1> QueryRunner for ComponentQuery<(T0, T1)>
where
    T0: 'static + Component,
    T1: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        let vs0 = context.entities_with_component::<T0>();
        let vs1 = context.entities_with_component::<T1>();
        vs0.into_iter()
            .filter(|v0| vs1.iter().any(|v1| *v0 == *v1))
            .collect()
    }
}

impl<T0, T1, T2> QueryRunner for ComponentQuery<(T0, T1, T2)>
where
    T0: 'static + Component,
    T1: 'static + Component,
    T2: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        let vs0 = context.entities_with_component::<T0>();
        let vs1 = context.entities_with_component::<T1>();
        let vs2 = context.entities_with_component::<T2>();
        vs0.into_iter()
            .filter(|v0| vs1.iter().any(|v1| *v0 == *v1) && vs2.iter().any(|v2| *v0 == *v2))
            .collect()
    }
}

impl<T0, T1, T2, T3> QueryRunner for ComponentQuery<(T0, T1, T2, T3)>
where
    T0: 'static + Component,
    T1: 'static + Component,
    T2: 'static + Component,
    T3: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        let vs0 = context.entities_with_component::<T0>();
        let vs1 = context.entities_with_component::<T1>();
        let vs2 = context.entities_with_component::<T2>();
        let vs3 = context.entities_with_component::<T3>();
        vs0.into_iter()
            .filter(|v0| {
                vs1.iter().any(|v1| *v0 == *v1)
                    && vs2.iter().any(|v2| *v0 == *v2)
                    && vs3.iter().any(|v3| *v0 == *v3)
            })
            .collect()
    }
}

impl<T0, T1, T2, T3, T4> QueryRunner for ComponentQuery<(T0, T1, T2, T3, T4)>
where
    T0: 'static + Component,
    T1: 'static + Component,
    T2: 'static + Component,
    T3: 'static + Component,
    T4: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        let vs0 = context.entities_with_component::<T0>();
        let vs1 = context.entities_with_component::<T1>();
        let vs2 = context.entities_with_component::<T2>();
        let vs3 = context.entities_with_component::<T3>();
        let vs4 = context.entities_with_component::<T4>();
        vs0.into_iter()
            .filter(|v0| {
                vs1.iter().any(|v| *v0 == *v)
                    && vs2.iter().any(|v| *v0 == *v)
                    && vs3.iter().any(|v| *v0 == *v)
                    && vs4.iter().any(|v| *v0 == *v)
            })
            .collect()
    }
}

impl<T0, T1, T2, T3, T4, T5> QueryRunner for ComponentQuery<(T0, T1, T2, T3, T4, T5)>
where
    T0: 'static + Component,
    T1: 'static + Component,
    T2: 'static + Component,
    T3: 'static + Component,
    T4: 'static + Component,
    T5: 'static + Component,
{
    fn run(&self, context: &Context) -> Vec<u64> {
        let vs0 = context.entities_with_component::<T0>();
        let vs1 = context.entities_with_component::<T1>();
        let vs2 = context.entities_with_component::<T2>();
        let vs3 = context.entities_with_component::<T3>();
        let vs4 = context.entities_with_component::<T4>();
        let vs5 = context.entities_with_component::<T5>();
        vs0.into_iter()
            .filter(|v0| {
                vs1.iter().any(|v| *v0 == *v)
                    && vs2.iter().any(|v| *v0 == *v)
                    && vs3.iter().any(|v| *v0 == *v)
                    && vs4.iter().any(|v| *v0 == *v)
                    && vs5.iter().any(|v| *v0 == *v)
            })
            .collect()
    }
}
