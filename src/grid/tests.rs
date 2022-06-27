//

use crate::{Error, Grid2d};

#[test]
fn constructors() {
    let g1 = Grid2d::new(0, 2, 3);
    assert_eq![vec![0, 0, 0, 0, 0, 0], g1.vec()];

    let g2 = Grid2d::from_row_order(&[1, 2, 3, 4, 5, 6], 2, 3).unwrap();
    assert_eq![vec![1, 2, 3, 4, 5, 6], g2.vec()];

    let g3 = Grid2d::from_rows(&[vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq![vec![1, 2, 3, 4, 5, 6], g3.vec()];
}

#[test]
fn general_query() {
    let g = Grid2d::new(0, 2, 3);
    assert_eq![2, g.num_rows()];
    assert_eq![3, g.num_cols()];
    assert_eq![6, g.capacity()];
    assert_eq![3, g.row_len()];
    assert_eq![2, g.col_len()];
}

#[test]
fn indexing() {
    let g = Grid2d::new('a', 4, 4);
    assert_eq![Ok(7), g.get_index(1, 3)];
    assert_eq![7, g.get_index_unchecked(1, 3)];
    assert_eq![Ok((1, 3)), g.get_coords(7)];
    assert_eq![(1, 3), g.get_coords_unchecked(7)];
}

#[test]
fn single() {
    // 1 2 3 4
    // 5 6 7 8
    let mut g = Grid2d::from_rows(&[vec![1, 2, 3, 4], vec![5, 6, 7, 8]]).unwrap();

    // get (Copy)

    assert_eq![Ok(7), g.get(1, 2)];
    assert_eq![7, g.get_unchecked(1, 2)];
    assert_eq![Err(Error::IndicesOutOfBounds(3, 4)), g.get(3, 4)];

    assert_eq![Ok(5), g.get_row_order(4)];
    assert_eq![5, g.get_row_order_unchecked(4)];
    assert_eq![Ok(3), g.get_col_order(4)];
    assert_eq![3, g.get_col_order_unchecked(4)];

    // get_ref

    assert_eq![Ok(&7), g.get_ref(1, 2)];
    assert_eq![&7, g.get_ref_unchecked(1, 2)];
    assert_eq![Err(Error::IndicesOutOfBounds(3, 4)), g.get_ref(3, 4)];

    assert_eq![Ok(&5), g.get_ref_row_order(4)];
    assert_eq![&5, g.get_ref_row_order_unchecked(4)];
    assert_eq![Ok(&3), g.get_ref_col_order(4)];
    assert_eq![&3, g.get_ref_col_order_unchecked(4)];

    // get_ref_mut

    assert_eq![Ok(&mut 7), g.get_ref_mut(1, 2)];
    assert_eq![&mut 7, g.get_ref_mut_unchecked(1, 2)];

    assert_eq![Ok(&mut 5), g.get_ref_mut_row_order(4)];
    assert_eq![&mut 5, g.get_ref_mut_row_order_unchecked(4)];
    assert_eq![Ok(&mut 3), g.get_ref_mut_col_order(4)];
    assert_eq![&mut 3, g.get_ref_mut_col_order_unchecked(4)];

    // set

    // 1 2 3 4
    // 5 6 A 8
    assert_eq![Ok(()), g.set(0xA, 1, 2)];
    assert_eq![Err(Error::IndicesOutOfBounds(3, 2)), g.set(0xB, 3, 2)];
    assert_eq![Ok(0xA), g.get(1, 2)];
    // 1 2 3 4
    // 5 6 C 8
    assert_eq![(), g.set_unchecked(0xC, 1, 2)];
    assert_eq![0xC, g.get_unchecked(1, 2)];

    // 1 E 3 4
    // 5 6 C 8
    assert_eq![Ok(()), g.set_row_order(0xE, 1)];
    assert_eq![Ok(0xE), g.get_row_order(1)];
    // 1 F 3 4
    // 5 6 C 8
    assert_eq![(), g.set_row_order_unchecked(0xF, 1)];
    assert_eq![0xF, g.get_row_order_unchecked(1)];

    // 1 F 3 4
    // E 6 C 8
    assert_eq![Ok(()), g.set_col_order(0xE, 1)];
    assert_eq![Ok(0xE), g.get_col_order(1)];
    // 1 F 3 4
    // F 6 C 8
    assert_eq![(), g.set_col_order_unchecked(0xF, 1)];
    assert_eq![0xF, g.get_col_order_unchecked(1)];

    assert_eq![vec![1, 0xF, 3, 4, 0xF, 6, 0xC, 8], g.vec()];
}

#[test]
fn iterators() {
    // all elements

    let mut g1 = Grid2d::from_rows(&[vec![1, 2], vec![3, 4], vec![5, 6]]).unwrap();
    {
        let mut i1 = g1.iter_mut();
        assert_eq![Some(&mut 1), i1.next()];
        assert_eq![Some(&mut 2), i1.next()];
        assert_eq![Some(&mut 3), i1.next()];
        assert_eq![Some(&mut 4), i1.next()];
        assert_eq![Some(&mut 5), i1.next()];
        assert_eq![Some(&mut 6), i1.next()];
        assert_eq![None, i1.next()];
    }
    {
        let mut i2 = g1.iter_col_order();
        assert_eq![Some(&1), i2.next()];
        assert_eq![Some(&3), i2.next()];
        assert_eq![Some(&5), i2.next()];
        assert_eq![Some(&2), i2.next()];
        assert_eq![Some(&4), i2.next()];
        assert_eq![Some(&6), i2.next()];
        assert_eq![None, i2.next()];
    }

    // single row & column

    let mut g2 = Grid2d::from_rows(&[
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ])
    .unwrap();

    {
        let mut irow = g2.row_iter(2).unwrap();
        assert_eq![Some(&9), irow.next()];
        assert_eq![Some(&10), irow.next()];
        assert_eq![Some(&11), irow.next()];
        assert_eq![Some(&12), irow.next()];
        assert_eq![None, irow.next()];
    }
    {
        let mut irowm = g2.row_iter_mut(2).unwrap();
        assert_eq![Some(&mut 9), irowm.next()];
        assert_eq![Some(&mut 10), irowm.next()];
        assert_eq![Some(&mut 11), irowm.next()];
        assert_eq![Some(&mut 12), irowm.next()];
        assert_eq![None, irowm.next()];
    }
    {
        let mut icol = g2.col_iter(2).unwrap();
        assert_eq![Some(&3), icol.next()];
        assert_eq![Some(&7), icol.next()];
        assert_eq![Some(&11), icol.next()];
        assert_eq![Some(&15), icol.next()];
        assert_eq![None, icol.next()];
    }
    {
        let mut icolm = g2.col_iter_mut(2).unwrap();
        assert_eq![Some(&mut 3), icolm.next()];
        assert_eq![Some(&mut 7), icolm.next()];
        assert_eq![Some(&mut 11), icolm.next()];
        assert_eq![Some(&mut 15), icolm.next()];
        assert_eq![None, icolm.next()];
    }
}
