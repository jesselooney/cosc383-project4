use std::ops::RangeBounds;

use itertools::iproduct;

#[derive(Clone, Copy, Debug)]
pub enum Order {
    Forward,
    Reverse,
}

impl Order {
    pub fn apply_to<T, I: Iterator<Item = T> + DoubleEndedIterator>(self, iterator: I) -> Vec<T> where
    {
        match self {
            Self::Forward => iterator.collect(),
            Self::Reverse => iterator.rev().collect(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            &Self::Forward => "F",
            &Self::Reverse => "R",
        }
    }
}

#[derive(Clone, Debug)]
pub struct IterationOrder {
    row_order: Order,
    column_order: Order,
    channel_indices: Vec<u32>,
    bit_indices: Vec<u32>,
    // TODO rename
    index_order: Vec<u8>,
    inverse_order: Vec<u8>,
}

impl IterationOrder {
    pub fn new<I, J, K>(
        row_order: Order,
        column_order: Order,
        channel_indices: I,
        bit_indices: J,
        index_order: K,
    ) -> IterationOrder
    where
        I: IntoIterator<Item = u32>,
        J: IntoIterator<Item = u32>,
        K: IntoIterator<Item = u8>,
    {
        let channel_indices: Vec<u32> = channel_indices.into_iter().collect();
        let bit_indices: Vec<u32> = bit_indices.into_iter().collect();
        let index_order: Vec<u8> = index_order.into_iter().collect();

        // Validate `channel_indices`.
        if !(1..=3).contains(&channel_indices.len()) {
            panic!("`channel_indices` must have length in [1, 3]");
        }
        for value in channel_indices.iter() {
            if *value > 2 {
                panic!("`channel_indices` must only have values in [0, 2]")
            }
        }
        // TODO check for no repeats

        // Validate `bit_indices`.
        if !(1..=8).contains(&bit_indices.len()) {
            panic!("`bit_indices` must have length in [1, 8]");
        }

        // Validate `index_order` and build inverse mapping.
        let mut inverse_order = vec![4u8; 4];
        if index_order.len() != 4 {
            panic!("`index_order` must have length 4");
        }
        for (key, value) in index_order.iter().enumerate() {
            if *value > 3 {
                panic!("`order` must only have values in [0, 3]")
            }
            inverse_order[*value as usize] = key as u8;
        }
        for value in inverse_order.clone() {
            if value == 4 {
                panic!("`order` must not have duplicate values")
            }
        }

        IterationOrder {
            row_order,
            column_order,
            channel_indices,
            bit_indices,
            index_order,
            inverse_order,
        }
    }
    /*
    type X = std::iter::Map<
        itertools::ConsTuples<
            itertools::Product<
                itertools::ConsTuples<
                    itertools::Product<
                        itertools::Product<std::vec::IntoIter<u32>, std::vec::IntoIter<u32>>,
                        std::vec::IntoIter<u32>,
                    >,
                    ((u32, u32), u32),
                >,
                std::vec::IntoIter<u32>,
            >,
            ((u32, u32, u32), u32),
        >,
        impl FnMut((u32, u32, u32, u32)) -> (u32, u32, u32, u32),
    >;*/

    pub fn into_iter(self, width: u32, height: u32) -> impl Iterator<Item = (u32, u32, u32, u32)> {
        let row_iter = self.row_order.apply_to(0..height);
        let column_iter = self.column_order.apply_to(0..width);

        let iterators = [
            row_iter,
            column_iter,
            self.channel_indices,
            self.bit_indices,
        ];

        let product = iproduct!(
            iterators[self.index_order[0] as usize].clone(),
            iterators[self.index_order[1] as usize].clone(),
            iterators[self.index_order[2] as usize].clone(),
            iterators[self.index_order[3] as usize].clone()
        );

        product.map(move |(v1, v2, v3, v4)| {
            let values = [v1, v2, v3, v4];
            (
                values[self.inverse_order[0] as usize],
                values[self.inverse_order[1] as usize],
                values[self.inverse_order[2] as usize],
                values[self.inverse_order[3] as usize],
            )
        })
    }

    pub fn name(&self) -> String {
        format!(
            "rw{}cl{}ch{}bt{}",
            self.row_order.name(),
            self.column_order.name(),
            &self
                .channel_indices
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(""),
            self.bit_indices
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Default for IterationOrder {
    fn default() -> Self {
        Self::new(Order::Forward, Order::Forward, [0, 1, 2], [1], [0, 1, 2, 3])
    }
}
