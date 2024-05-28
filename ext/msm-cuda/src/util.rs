// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

// GRT modify
use ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM};
use ark_std::UniformRand;

// GRT modify
pub fn generate_points_scalars<G: AffineRepr>(
    len: usize,
) -> (Vec<G>, Vec<G::ScalarField>)
    // where
    //     G::Group: VariableBaseMSM<MulBase = G>,
{
    let rand_gen: usize = std::cmp::min(1usize << 11, len);
    let mut rng = ChaCha20Rng::from_entropy();

    // GRT modify
    // let mut points =
    //     <G as AffineRepr>::Group::normalize_batch(
    //     <G as AffineRepr>::Group::batch_convert_to_mul_base(
    //     // G::Group::batch_normalization_into_affine(
    //         &(0..rand_gen)
    //             .map(|_| G::Group::rand(&mut rng))
    //             .collect::<Vec<_>>(),
    //     );

    // GRT modify
    let mut points = (0..rand_gen).map(|_| G::Group::rand(&mut rng).into_affine())
        .collect::<Vec<_>>();
    // Sprinkle in some infinity points
    if len > 2 {
        points[3] = G::zero();
    }
    let scalars = (0..len)
        .map(|_| G::ScalarField::rand(&mut rng))
        .collect::<Vec<_>>();

    while points.len() < len {
        points.append(&mut points.clone());
    }

    points.truncate(len);

    (points, scalars)
}
