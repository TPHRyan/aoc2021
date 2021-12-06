pub fn int_from_bit_iter<'a, T, U>(bit_iter: T) -> u32
where
    T: ExactSizeIterator + DoubleEndedIterator<Item = &'a U>,
    U: 'a + Copy,
    u32: From<U>,
{
    bit_iter
        .rev()
        .enumerate()
        .rfold(0, |acc, (i, c)| acc + (u32::from(*c) << i))
}
