use std::{error::Error, vec};
pub struct DES {
    parity_drop_table: [usize; 56],
    shift_table: [usize; 16],
    key_compression_table: [usize; 48],
    initial_permutation_table: [usize; 64],
    expansion_permutation_table: [usize; 48],
    s_box_table: [[[usize; 16]; 4]; 8],
    p_box_permutation_table: [usize; 32],
    final_permutation_table: [usize; 64],
}

impl DES {
    pub fn new() -> DES {
        DES {
            parity_drop_table: [
                57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27,
                19, 11, 3, 60, 52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22,
                14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 28, 20, 12, 4,
            ],
            shift_table: [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1],
            key_compression_table: [
                14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20,
                13, 2, 41, 52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46,
                42, 50, 36, 29, 32,
            ],
            initial_permutation_table: [
                58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30,
                22, 14, 6, 64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43,
                35, 27, 19, 11, 3, 61, 53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
            ],
            expansion_permutation_table: [
                32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17,
                16, 17, 18, 19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30,
                31, 32, 1,
            ],
            s_box_table: [
                [
                    [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
                    [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
                    [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
                    [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
                ],
                [
                    [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
                    [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
                    [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
                    [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
                ],
                [
                    [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
                    [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
                    [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
                    [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
                ],
                [
                    [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
                    [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
                    [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
                    [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
                ],
                [
                    [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
                    [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
                    [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
                    [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
                ],
                [
                    [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
                    [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
                    [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
                    [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
                ],
                [
                    [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
                    [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
                    [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
                    [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
                ],
                [
                    [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
                    [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
                    [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
                    [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
                ],
            ],
            p_box_permutation_table: [
                16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27,
                3, 9, 19, 13, 30, 6, 22, 11, 4, 25,
            ],
            final_permutation_table: [
                40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54,
                22, 62, 30, 37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3,
                43, 11, 51, 19, 59, 27, 34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57,
                25,
            ],
        }
    }

    pub fn encrypt(
        &self,
        data: Vec<u8>,
        key: &[u8],
        add_padding: bool,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut data = data.clone();
        if key.len() != 8 {
            return Err(Box::from("Key length must be 8 bytes"));
        }
        if add_padding {
            data = Self::add_pkcs_7_padding(&data, 8)?;
        }

        Ok(self.desprocess(&data, key, true))
    }

    fn desprocess(&self, input_data: &[u8], encription_key: &[u8], ascending: bool) -> Vec<u8> {
        //don't know if this works
        let mut proccessed_data = vec![0; input_data.len()];
        let block_count = input_data.len() / 8;
        let round_keys = self.generate_keys(encription_key, ascending);
        // let mut block_buffer = [0; 8];
        let mut block_buffer = vec![0; 8];
        let mut left_half = vec![0; 4];
        let mut right_half = vec![0; 4];
        let mut expanded_right_half;
        let mut substituted_right_half = vec![0; 4];
        let mut temp_right_half;

        for block_num in 0..block_count {
            // for i in 0..64 {
            //     // *block_buffer.get_mut(i).unwrap() = input_data.get(block_num * 64 + i).unwrap();
            //     Self::set_bit_at(
            //         &mut block_buffer,
            //         i,
            //         Self::get_bit_at(input_data, block_num * 64 + i),
            //     );
            // }
            for i in 0..8 {
                block_buffer[i] = input_data[block_num * 8 + i];
            }
            block_buffer = Self::permute(&block_buffer, self.initial_permutation_table.to_vec());

            for round in 0..16 {
                // for i in 0..32 {
                //     // *left_half.get_mut(i).unwrap() = block_buffer.get(i).unwrap();
                //     // *right_half.get_mut(i).unwrap() = block_buffer.get(i + 32).unwrap();
                // }
                for i in 0..4 {
                    left_half[i] = block_buffer[i];
                    right_half[i] = block_buffer[i + 4];
                }

                expanded_right_half = Self::permute(
                    &right_half.clone(),
                    self.expansion_permutation_table.to_vec(),
                );

                expanded_right_half = Self::xor(&expanded_right_half, &round_keys[round]);

                for section in 0..8 {
                    let row = ((Self::get_bit_at(&expanded_right_half, section * 6) as usize) << 1)
                        | (Self::get_bit_at(&expanded_right_half, section * 6 + 5) as usize);
                    let mut column = 0;

                    for bit_index in 0..4 {
                        column |=
                            (Self::get_bit_at(&expanded_right_half, section * 6 + bit_index + 1)
                                << (3 - bit_index)) as usize;
                    }

                    let s_box_value = self.s_box_table[section][row][column];

                    for bit_index in 0..4 {
                        Self::set_bit_at(
                            &mut substituted_right_half,
                            section * 4 + bit_index,
                            ((s_box_value >> (3 - bit_index)) & 1) as u8,
                        );
                    }
                }

                substituted_right_half = Self::permute(
                    &substituted_right_half,
                    self.p_box_permutation_table.to_vec(),
                );

                // left_half.xor(&substituted_right_half);
                // temp_right_half = left_half.clone();
                temp_right_half = Self::xor(&left_half, &substituted_right_half);

                if round != 15 {
                    for i in 0..4 {
                        block_buffer[i] = right_half[i];
                        block_buffer[i + 4] = temp_right_half[i];
                    }
                } else {
                    for i in 0..4 {
                        block_buffer[i] = temp_right_half[i];
                        block_buffer[i + 4] = right_half[i];
                    }
                }
            }
            block_buffer = Self::permute(&block_buffer, self.final_permutation_table.to_vec());
            // for i in 0..64 {
            //     *proccessed_data.get_mut(block_num * 64 + i).unwrap() =
            //         block_buffer.get(i).unwrap();
            // }
            for i in 0..8 {
                proccessed_data[block_num * 8 + i] = block_buffer[i];
            }
        }

        proccessed_data
    }

    fn generate_keys(&self, initial_key: &[u8], ascending: bool) -> [Vec<u8>; 16] {
        // let mut round_keys: [BitVec; 16] = core::array::from_fn(|_| BitVec::new());
        let mut round_keys: [Vec<u8>; 16] = [const { Vec::new() }; 16];
        let mut permuted_key = Self::permute(initial_key, self.parity_drop_table.to_vec());
        for round in 0..16 {
            let left_half = Self::select_bits(&permuted_key, 0, 28);
            let right_half = Self::select_bits(&permuted_key, 28, 28);

            let left_half = Self::left_shift(&left_half, 28, self.shift_table[round]);
            let right_half = Self::left_shift(&right_half, 28, self.shift_table[round]);

            let combined_key = Self::join_key(&left_half, &right_half);
            round_keys[round] = Self::permute(&combined_key, self.key_compression_table.to_vec());
            permuted_key = combined_key;
        }

        if !ascending {
            round_keys.reverse();
        }

        round_keys
    }

    fn permute(source: &[u8], table: Vec<usize>) -> Vec<u8> {
        let length = table.len();
        let mut result = vec![0; length];
        for i in 0..table.len() {
            Self::set_bit_at(&mut result, i, Self::get_bit_at(source, table[i] - 1));
        }
        result
    }

    fn select_bits(source: &[u8], start: usize, count: usize) -> Vec<u8> {
        let mut result = vec![0; count];
        for i in 0..count {
            // Self::set_bit_at(&mut result, i, source.get(start + i).unwrap());
            Self::set_bit_at(&mut result, i, Self::get_bit_at(source, start + i));
        }

        result
    }

    fn left_shift(data: &[u8], len: usize, shift: usize) -> Vec<u8> {
        let mut outer: Vec<u8> = vec![0; (len - 1) / 8 + 1];
        for i in 0..len {
            // let val = data.get((i + shift) % len).unwrap();
            // Self::set_bit_at(&mut outer, i, val);
            let val = Self::get_bit_at(data, (i + shift) % len);
            Self::set_bit_at(&mut outer, i, val);
        }
        outer
    }

    fn join_key(left_half: &[u8], right_half: &[u8]) -> Vec<u8> {
        // let mut result = BitVec::from_elem(56, false);
        let mut result: Vec<u8> = vec![0; 7];
        for i in 0..3 {
            result[i] = left_half[i];
        }
        for j in 0..4 {
            let val = Self::get_bit_at(left_half, 24 + j);
            Self::set_bit_at(&mut result, 24 + j, val);
        }
        for w in 0..28 {
            let val = Self::get_bit_at(right_half, w);
            Self::set_bit_at(&mut result, 28 + w, val);
        }

        result
    }

    fn add_pkcs_7_padding(data: &[u8], block_size: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        if data.len() <= 0 {
            return Err(Box::from("Data cannot be null"));
        }

        if block_size <= 0 {
            return Err(Box::from("Block size must be greater than zero"));
        }

        let count = data.len();
        let padding_remainder = count % block_size;
        let mut padding_size = block_size - padding_remainder;

        if padding_size == 0 {
            padding_size = block_size;
        }

        let mut padded_data = vec![0; data.len() + padding_size];
        for i in 0..data.len() {
            padded_data[i] = data[i];
        }

        let padding_byte = padding_size as u8;

        for i in data.len()..padded_data.len() {
            padded_data[i] = padding_byte;
        }

        Ok(padded_data)
    }

    //don't know if this works
    fn set_bit_at(data: &mut [u8], position: usize, value: u8) {
        let pos_byte = position / 8;
        let pos_bit = position % 8;

        if value == 1 {
            data[pos_byte] |= 1 << (7 - pos_bit);
        } else {
            data[pos_byte] &= !(1 << (7 - pos_bit));
        }
    }

    fn get_bit_at(data: &[u8], position: usize) -> u8 {
        let pos_byte = position / 8;
        let pos_bit = position % 8;
        data[pos_byte] >> (7 - pos_bit) & 1
    }

    fn xor(first: &[u8], second: &[u8]) -> Vec<u8> {
        let mut result = vec![0; first.len()];
        for i in 0..first.len() {
            result[i] = first[i] ^ second[i];
        }

        result
    }
}
