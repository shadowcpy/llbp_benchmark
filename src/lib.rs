use std::{iter::repeat_with, process::Command};

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

pub fn try_cmd<'s>(cmd: &str, args: impl IntoIterator<Item = &'s str>) {
    Command::new(cmd)
        .args(args)
        .spawn()
        .inspect_err(|e| eprintln!("failed to run command '{cmd}': {e}"))
        .ok();
}

pub fn random_vec<T>(n: usize) -> Vec<T>
where
    StandardUniform: Distribution<T>,
{
    let mut rng = rand::rng();
    repeat_with(|| rng.random()).take(n).collect()
}
