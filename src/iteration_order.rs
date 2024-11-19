use itertools::iproduct;

#[derive(Clone, Copy)]
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
}

#[derive(Clone, Copy)]
pub struct IterationOrder {
    row_order: Order,
    column_order: Order,
    channel_indices: [u8; 3],
    bit_order: Order,
    index_order: [u8; 4],
    inverse_order: [u8; 4],
}

impl IterationOrder {
    pub fn new(
        row_order: Order,
        column_order: Order,
        channel_indices: [u8; 3],
        bit_order: Order,
        index_order: [u8; 4],
    ) -> IterationOrder {
        for value in channel_indices {
            if value > 2 {
                panic!("`channel_indices` must only have values in [0, 2]")
            }
        }
        // TODO check for no repeats

        let mut inverse_order = [4u8; 4];
        for (key, value) in index_order.into_iter().enumerate() {
            if value > 3 {
                panic!("`order` must only have values in [0, 3]")
            }
            inverse_order[value as usize] = key as u8;
        }
        for value in inverse_order {
            if value == 4 {
                panic!("`order` must not have duplicate values")
            }
        }

        IterationOrder {
            row_order,
            column_order,
            channel_indices,
            bit_order,
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

    pub fn into_iter(
        self,
        width: u32,
        height: u32,
    ) -> Box<dyn Iterator<Item = (u32, u32, u32, u32)>> {
        let row_iter = self.row_order.apply_to(0..height);
        let column_iter = self.column_order.apply_to(0..width);
        let bit_iter = self.bit_order.apply_to(0..8);
        let iterators = [
            row_iter,
            column_iter,
            self.channel_indices.map(|x| x as u32).to_vec(),
            bit_iter,
        ];
        let product = iproduct!(
            iterators[self.index_order[0] as usize].clone(),
            iterators[self.index_order[1] as usize].clone(),
            iterators[self.index_order[2] as usize].clone(),
            iterators[self.index_order[3] as usize].clone()
        );

        Box::new(product.map(move |(v1, v2, v3, v4)| {
            let values = [v1, v2, v3, v4];
            (
                values[self.inverse_order[0] as usize],
                values[self.inverse_order[1] as usize],
                values[self.inverse_order[2] as usize],
                values[self.inverse_order[3] as usize],
            )
        }))
    }
}

impl Default for IterationOrder {
    fn default() -> Self {
        Self::new(
            Order::Forward,
            Order::Forward,
            [0, 1, 2],
            Order::Forward,
            [0, 1, 2, 3],
        )
    }
}
